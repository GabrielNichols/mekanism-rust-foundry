use crate::LabResult;
use std::{
    path::Path,
    process::{Child, Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};

pub struct ManagedProcess {
    child: Child,
    #[cfg(windows)]
    job: windows_sys::Win32::Foundation::HANDLE,
}
impl ManagedProcess {
    pub fn spawn(
        program: &Path,
        args: &[String],
        cwd: &Path,
        env: &[(String, String)],
        log: &Path,
    ) -> LabResult<Self> {
        Self::spawn_with_stdin(program, args, cwd, env, log, Stdio::null())
    }

    pub(crate) fn spawn_console(
        program: &Path,
        args: &[String],
        cwd: &Path,
        env: &[(String, String)],
        log: &Path,
    ) -> LabResult<Self> {
        Self::spawn_with_stdin(program, args, cwd, env, log, Stdio::piped())
    }

    fn spawn_with_stdin(
        program: &Path,
        args: &[String],
        cwd: &Path,
        env: &[(String, String)],
        log: &Path,
        stdin: Stdio,
    ) -> LabResult<Self> {
        let stdout =
            std::fs::File::create(log.with_extension("stdout.log")).map_err(|e| e.to_string())?;
        let stderr =
            std::fs::File::create(log.with_extension("stderr.log")).map_err(|e| e.to_string())?;
        let mut command = Command::new(program);
        for (key, _) in std::env::vars_os() {
            if key
                .to_string_lossy()
                .to_ascii_uppercase()
                .starts_with("MEKLAB_")
            {
                command.env_remove(key);
            }
        }
        command
            .args(args)
            .current_dir(cwd)
            .envs(env.iter().cloned())
            .env_remove("JAVA_TOOL_OPTIONS")
            .env_remove("JDK_JAVA_OPTIONS")
            .env_remove("_JAVA_OPTIONS")
            .env_remove("CLASSPATH")
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW: suppress console helpers.
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let mut child = command.spawn().map_err(|e| format!("spawn: {e}"))?;
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::{Foundation::CloseHandle, System::JobObjects::*};
            // The job handle is owned here; the process handle is borrowed from Child.
            let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
            if job.is_null() {
                let _ = child.kill();
                let _ = child.wait();
                return Err("CreateJobObject failed".into());
            }
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let configured = unsafe {
                SetInformationJobObject(
                    job,
                    JobObjectExtendedLimitInformation,
                    &info as *const _ as *const _,
                    std::mem::size_of_val(&info) as u32,
                )
            };
            let assigned = configured != 0
                && unsafe { AssignProcessToJobObject(job, child.as_raw_handle()) } != 0;
            if !assigned {
                unsafe {
                    CloseHandle(job);
                }
                let _ = child.kill();
                let _ = child.wait();
                return Err("process job assignment failed".into());
            }
            Ok(Self { child, job })
        }
        #[cfg(not(windows))]
        {
            Ok(Self { child })
        }
    }
    pub(crate) fn console_write(&mut self, command: &str) -> LabResult<()> {
        use std::io::Write;
        let stdin = self.child.stdin.as_mut().ok_or("console input is closed")?;
        writeln!(stdin, "{command}")
            .and_then(|_| stdin.flush())
            .map_err(|error| format!("console write: {error}"))
    }

    pub(crate) fn cpu_ns(&self) -> LabResult<Option<u64>> {
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::{Foundation::FILETIME, System::Threading::GetProcessTimes};
            let mut creation = FILETIME::default();
            let mut exit = FILETIME::default();
            let mut kernel = FILETIME::default();
            let mut user = FILETIME::default();
            let ok = unsafe {
                GetProcessTimes(
                    self.child.as_raw_handle(),
                    &mut creation,
                    &mut exit,
                    &mut kernel,
                    &mut user,
                )
            };
            if ok == 0 {
                return Err(format!(
                    "process CPU time: {}",
                    std::io::Error::last_os_error()
                ));
            }
            let ticks = |time: FILETIME| {
                (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime)
            };
            Ok(Some((ticks(kernel) + ticks(user)) * 100))
        }
        #[cfg(not(windows))]
        {
            Ok(None)
        }
    }

    pub(crate) fn terminate(&mut self, timeout: Duration) -> LabResult<ExitStatus> {
        if self.try_wait()?.is_some() {
            return Err("owned process exited before intentional termination".into());
        }
        self.child
            .kill()
            .map_err(|error| format!("terminate owned child: {error}"))?;
        let status = self.wait_timeout(timeout)?;
        if status.success() {
            return Err("intentional termination reported a clean exit".into());
        }
        Ok(status)
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }
    pub fn try_wait(&mut self) -> LabResult<Option<ExitStatus>> {
        self.child.try_wait().map_err(|e| e.to_string())
    }
    pub fn wait_timeout(&mut self, timeout: Duration) -> LabResult<ExitStatus> {
        let start = Instant::now();
        loop {
            if let Some(status) = self.try_wait()? {
                return Ok(status);
            }
            if start.elapsed() >= timeout {
                return Err("process timeout".into());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
impl Drop for ManagedProcess {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.job);
        }
        #[cfg(unix)]
        {
            unsafe extern "C" {
                fn kill(pid: i32, signal: i32) -> i32;
            }
            // Only the group created for this child is targeted, while its PID is owned.
            if self.child.try_wait().ok().flatten().is_none() {
                unsafe {
                    kill(-(self.child.id() as i32), 9);
                }
            }
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
