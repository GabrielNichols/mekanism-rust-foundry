mod audit;
mod catalog;
mod chunks;
use crate::LabResult;
use serde_json::Value;

use crate::{
    capture,
    lab::{self, Environment, Session},
    nbt,
    process::ManagedProcess,
};
use serde_json::json;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    net::TcpListener,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const POS: &str = "0 80 0";
const CUT: u32 = 73;
const REMAINING: u32 = 127;
// The pinned reference still has an update-delay countdown after 20 dry ticks.
const DRY: u32 = 64;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(90);

fn validate_machine(state: &Value, progress: u32, finished: bool) -> LabResult<()> {
    validate_machine_with_ore(state, progress, finished, 1)
}

fn validate_machine_with_ore(
    state: &Value,
    progress: u32,
    finished: bool,
    ore: u32,
) -> LabResult<()> {
    let (slot, item, count) = if finished {
        ("1", "mekanism:dust_iron", 2)
    } else {
        ("0", "minecraft:iron_ore", ore)
    };
    let count = count.to_string();
    let fields = &state["value"];
    let required = [
        ("id", json!({"tag":8,"value":"mekanism:enrichment_chamber"})),
        ("x", json!({"tag":3,"value":"0"})),
        ("y", json!({"tag":3,"value":"80"})),
        ("z", json!({"tag":3,"value":"0"})),
        ("progress", json!({"tag":3,"value":progress.to_string()})),
        (
            "items",
            json!({"tag":9,"element_tag":10,"value":[{"tag":10,"value":{
                "slot":{"tag":1,"value":slot},"item":{"tag":10,"value":{
                    "id":{"tag":8,"value":item},"count":{"tag":3,"value":count}
                }}
            }}]}),
        ),
    ];
    if state["tag"] != 10 {
        return Err("machine NBT is not a compound".into());
    }
    for (field, expected) in required {
        if fields[field] != expected {
            return Err(format!(
                "machine NBT {field}: expected {expected}, observed {}",
                fields[field]
            ));
        }
    }
    // Mekanism omits empty containers from its saved data.
    if let Some(energy) = fields.get("energy_containers") {
        let zero = json!({"tag":9,"element_tag":10,"value":[
            {"tag":10,"value":{"container":{"tag":1,"value":"0"},"stored":{"tag":4,"value":"0"}}}
        ]});
        if energy != &zero {
            return Err(format!("machine did not have zero stored energy: {energy}"));
        }
    }
    Ok(())
}

fn validate_advance(start: u64, steps: u32, observed: u64) -> LabResult<bool> {
    let target = start
        .checked_add(u64::from(steps))
        .ok_or("clock overflow")?;
    if observed < start || observed > target {
        return Err(format!(
            "game time {observed} outside commanded interval {start}..={target}"
        ));
    }
    Ok(observed == target)
}

fn read_line(reader: &mut impl BufRead, pending: &mut Vec<u8>) -> LabResult<Option<String>> {
    let read = reader
        .read_until(b'\n', pending)
        .map_err(|error| error.to_string())?;
    if pending.len() > 1_048_576 {
        return Err("oversize console log record".into());
    }
    if read == 0 || pending.last() != Some(&b'\n') {
        return Ok(None);
    }
    let line = String::from_utf8_lossy(pending)
        .trim_end_matches(['\r', '\n'])
        .to_owned();
    pending.clear();
    Ok(Some(line))
}

fn server_message(line: &str) -> Option<&str> {
    let (_, body) = line.split_once("] [Server thread/INFO] [minecraft/")?;
    let (logger, message) = body.split_once("]: ")?;
    matches!(logger, "MinecraftServer" | "DedicatedServer").then_some(message)
}

fn one_reply<'a>(lines: &'a [String], prefix: &str) -> LabResult<&'a str> {
    let mut matches = lines.iter().filter_map(|line| line.strip_prefix(prefix));
    let reply = matches
        .next()
        .ok_or_else(|| format!("missing console reply {prefix:?}: {lines:?}"))?;
    if matches.next().is_some() {
        return Err(format!("ambiguous console reply {prefix:?}: {lines:?}"));
    }
    Ok(reply)
}

fn matches_boundary(message: &str, marker: &str) -> bool {
    message.strip_prefix("[Not Secure] [Server] ") == Some(marker)
}

fn elapsed_ns(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

struct Server {
    child: ManagedProcess,
    reader: BufReader<File>,
    pending: Vec<u8>,
    sequence: u32,
    bridge: Option<Session>,
    observer: Observer,
    log: PathBuf,
    boot_ns: u64,
    _temp: tempfile::TempDir,
}

impl Server {
    fn wait_message(&mut self, deadline: Instant) -> LabResult<String> {
        loop {
            if Instant::now() >= deadline {
                return Err(format!("console timed out: {}", self.log.display()));
            }
            if let Some(line) = read_line(&mut self.reader, &mut self.pending)? {
                if let Some(message) = server_message(&line) {
                    return Ok(message.into());
                }
                continue;
            }
            if let Some(status) = self.child.try_wait()? {
                return Err(format!("server exited ({status}): {}", self.log.display()));
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn command(&mut self, command: &str) -> LabResult<Vec<String>> {
        self.sequence += 1;
        let marker = format!("meklab-boundary-{}-{}", self.child.id(), self.sequence);
        self.child.console_write(command)?;
        self.child.console_write(&format!("say {marker}"))?;
        let deadline = Instant::now() + COMMAND_TIMEOUT;
        let mut lines = Vec::new();
        loop {
            let message = self
                .wait_message(deadline)
                .map_err(|error| format!("{command}: {error}"))?;
            if matches_boundary(&message, &marker) {
                return Ok(lines);
            }
            if lines.len() >= 1024 {
                return Err(format!("too many console messages for {command}"));
            }
            lines.push(message);
        }
    }

    fn expect(&mut self, command: &str, prefix: &str) -> LabResult<()> {
        one_reply(&self.command(command)?, prefix)?;
        Ok(())
    }

    fn clock(&mut self) -> LabResult<u64> {
        one_reply(&self.command("time query gametime")?, "The time is ")?
            .parse()
            .map_err(|_| "invalid gametime response".into())
    }

    fn snapshot(&mut self) -> LabResult<String> {
        let lines = self.command(&format!("data get block {POS}"))?;
        let nbt = one_reply(&lines, "0, 80, 0 has the following block data: ")?;
        Ok(nbt.into())
    }

    fn steps(&mut self, count: u32) -> LabResult<Value> {
        let mut batches = Vec::new();
        let mut start = self.clock()?;
        let mut remaining = count;
        while remaining > 0 {
            let step = remaining.min(20);
            let timer = Instant::now();
            let cpu_start = self.child.cpu_ns()?;
            let bridge_wall_ns;
            let bridge_file = if let Some(bridge) = &mut self.bridge {
                let args = json!({"ticks":step,"observer_mode":"every_tick"});
                let reply = bridge.request("step", args.as_object().unwrap())?;
                bridge_wall_ns = Some(elapsed_ns(timer));
                let path = self.log.with_extension(format!("step-{start}.json"));
                lab::atomic_json(&path, &reply)?;
                capture::validate_reply(start, u64::from(step), "every_tick", &reply)?;
                Some(path.file_name().unwrap().to_string_lossy().into_owned())
            } else {
                bridge_wall_ns = None;
                self.expect(&format!("tick step {step}"), "Stepping")?;
                None
            };
            let deadline = Instant::now() + COMMAND_TIMEOUT;
            let end = loop {
                let current = self.clock()?;
                if validate_advance(start, step, current)? {
                    break current;
                }
                if Instant::now() >= deadline {
                    return Err(format!(
                        "tick batch did not advance from {start} by {step}, last {current}"
                    ));
                }
                std::thread::sleep(Duration::from_millis(5));
            };
            let controller_wall_ns = elapsed_ns(timer);
            let process_cpu_ns = self
                .child
                .cpu_ns()?
                .zip(cpu_start)
                .map(|(end, start)| end.checked_sub(start).ok_or("process CPU clock regressed"))
                .transpose()?;
            batches.push(json!({"ticks":step,"start":start,"end":end,"controller_wall_ns":controller_wall_ns,
                "process_cpu_ns":process_cpu_ns,"bridge_reply_wall_ns":bridge_wall_ns,"bridge_capture":bridge_file}));
            start = end;
            remaining -= step;
        }
        Ok(json!(batches))
    }

    fn save(&mut self) -> LabResult<u64> {
        let start = Instant::now();
        self.expect("save-all flush", "Saved the game")?;
        Ok(elapsed_ns(start))
    }

    fn exit(mut self, abrupt: bool) -> LabResult<Value> {
        let start = Instant::now();
        let status = if abrupt {
            self.child.terminate(COMMAND_TIMEOUT)?
        } else {
            self.child.console_write("stop")?;
            let status = self.child.wait_timeout(COMMAND_TIMEOUT)?;
            if !status.success() {
                return Err(format!("server stop failed: {status}"));
            }
            status
        };
        let stdout = fs::read_to_string(self.log.with_extension("stdout.log"))
            .map_err(|error| error.to_string())?;
        let graceful_stop = stdout
            .lines()
            .filter_map(server_message)
            .any(|line| line == "Stopping server");
        if graceful_stop == abrupt {
            return Err("server shutdown log does not match requested termination".into());
        }
        let disabled_markers = stdout
            .lines()
            .filter(|line| line.ends_with(DISABLED_MARKER))
            .count();
        if disabled_markers != usize::from(self.observer == Observer::Disabled) {
            return Err(format!(
                "unexpected disabled bridge marker count: {disabled_markers}"
            ));
        }
        Ok(
            json!({"observer_disabled_markers":disabled_markers,"pid":self.child.id(),"boot_ns":self.boot_ns,"exit_wall_ns":elapsed_ns(start),
            "exit_code":status.code(),"graceful_stop":graceful_stop,"lifetime_process_cpu_ns":self.child.cpu_ns()?}),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Observer {
    Absent,
    Disabled,
    EveryTick,
    Audit(audit::Fault),
    Chunk(bool),
    Catalog,
}

impl Observer {
    fn flags(self) -> (bool, bool) {
        (
            self != Self::Absent,
            matches!(
                self,
                Self::EveryTick | Self::Audit(_) | Self::Chunk(_) | Self::Catalog
            ),
        )
    }

    fn name(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Disabled => "disabled",
            Self::EveryTick => "every_tick",
            Self::Audit(_) => "audit",
            Self::Chunk(_) => "chunk_control",
            Self::Catalog => "catalog",
        }
    }
}

const DISABLED_MARKER: &str = "MekLab observer disabled before hook registration";

fn launch(
    environment: &Environment,
    dir: &Path,
    log: PathBuf,
    observer: Observer,
    tick_rate: u16,
) -> LabResult<Server> {
    let (installed, enabled) = observer.flags();
    let listener = if enabled {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|error| error.to_string())?;
        listener
            .set_nonblocking(true)
            .map_err(|error| error.to_string())?;
        Some(listener)
    } else {
        None
    };
    let token = format!(
        "lifecycle-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos()
    );
    // No game clients connect in these fixtures. The OS assigns a unique game port.
    let mods = lab::prepare_server(environment, dir, 0, installed)?;
    lab::atomic_json(&log.with_extension("mods.json"), &mods)?;
    let tmp = environment
        .server_root
        .parent()
        .and_then(Path::parent)
        .ok_or("runtime root has no parent")?
        .join("tmp");
    fs::create_dir_all(&tmp).map_err(|error| error.to_string())?;
    let temp = tempfile::Builder::new()
        .prefix("l")
        .tempdir_in(tmp)
        .map_err(|error| error.to_string())?;
    let temp_path = temp.path().to_string_lossy();
    let mut env = lab::private_homes(dir)?;
    env.extend([
        ("TEMP".into(), temp_path.to_string()),
        ("TMP".into(), temp_path.to_string()),
    ]);
    if observer == Observer::Disabled {
        env.push(("MEKLAB_OBSERVER".into(), "disabled".into()));
    }
    if let Observer::Audit(fault) = observer {
        env.push(("MEKLAB_AUDIT".into(), fault.name().into()));
    }
    if let Observer::Chunk(force_load) = observer {
        env.push((
            "MEKLAB_CHUNK_CONTROL".into(),
            if force_load { "load" } else { "read" }.into(),
        ));
    }
    if observer == Observer::Catalog {
        env.push(("MEKLAB_CATALOG".into(), "1".into()));
    }
    if let Some(listener) = &listener {
        env.extend([
            (
                "MEKLAB_PORT".into(),
                listener
                    .local_addr()
                    .map_err(|error| error.to_string())?
                    .port()
                    .to_string(),
            ),
            ("MEKLAB_SESSION".into(), token.clone()),
            ("MEKLAB_ROLE".into(), "server".into()),
        ]);
    }
    let args = [
        "-Xmx2G".into(),
        "-Djava.net.preferIPv4Stack=true".into(),
        "-Duser.language=en".into(),
        "-Duser.country=US".into(),
        format!("-Djdk.net.unixdomain.tmpdir={temp_path}"),
        format!("-Djava.io.tmpdir={temp_path}"),
        "@launch.args".into(),
        "nogui".into(),
    ];
    let timer = Instant::now();
    let child =
        ManagedProcess::spawn_console(lab::role(environment, "java")?, &args, dir, &env, &log)?;
    let reader = BufReader::new(
        File::open(log.with_extension("stdout.log")).map_err(|error| error.to_string())?,
    );
    let mut server = Server {
        child,
        reader,
        pending: Vec::new(),
        sequence: 0,
        bridge: None,
        observer,
        log,
        boot_ns: 0,
        _temp: temp,
    };
    let timeout = environment.boot_timeout()?;
    let deadline = timer + timeout;
    while !server.wait_message(deadline)?.starts_with("Done (") {}
    if let Some(listener) = listener {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let (bridge, hello) =
            lab::accept(&listener, &mut server.child, &token, remaining, "server")?;
        lab::atomic_json(&server.log.with_extension("hello.json"), &hello)?;
        if hello["minecraft"] != "1.21.1" || hello["mekanism"] != "10.7.19" {
            return Err(format!("unexpected reference versions: {hello}"));
        }
        server.bridge = Some(bridge);
        // The bridge already freezes on ServerStarted.
    } else {
        server.expect("tick freeze", "The game is frozen")?;
    }
    let reply = server.command(&format!("tick rate {tick_rate}"))?;
    let rate = one_reply(&reply, "Set the target tick rate to ")?
        .strip_suffix(" per second")
        .and_then(|value| value.parse::<f32>().ok());
    if rate != Some(f32::from(tick_rate)) {
        return Err(format!(
            "tick rate {tick_rate} was not acknowledged: {reply:?}"
        ));
    }
    let first_tick = server.clock()?;
    std::thread::sleep(Duration::from_millis(125));
    validate_advance(first_tick, 0, server.clock()?)?;
    server.boot_ns = elapsed_ns(timer);
    Ok(server)
}

fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if entry.file_name() == "session.lock" && kind.is_file() {
            continue;
        }
        let dest = to.join(entry.file_name());
        if kind.is_dir() {
            copy_tree(&entry.path(), &dest)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), dest)?;
        } else {
            return Err(io::Error::other("unexpected special file in private seed"));
        }
    }
    Ok(())
}

fn disk(dir: &Path, log: &Path, progress: u32, finished: bool) -> LabResult<Value> {
    let state = nbt::block_entity(&dir.join("world/region/r.0.0.mca"), 0, 80, 0)?;
    lab::atomic_json(log, &state)?;
    let state = nbt::persisted_projection(&state)?;
    validate_machine(&state, progress, finished)?;
    Ok(state)
}

fn load_energy(server: &mut Server, ticks: u32) -> LabResult<()> {
    server.expect(
        &format!(
            "data merge block {POS} {{energy_containers:[{{container:0b,stored:{}L}}]}}",
            ticks * 50
        ),
        "Modified block data of ",
    )
}

fn pulse(server: &mut Server, ticks: u32) -> LabResult<Value> {
    load_energy(server, ticks)?;
    Ok(json!({"active":server.steps(ticks)?,"dry":server.steps(DRY)?}))
}

impl Server {
    fn live_int(&mut self, field: &str) -> LabResult<u32> {
        let lines = self.command(&format!("data get block {POS} {field}"))?;
        let value = one_reply(&lines, "0, 80, 0 has the following block data: ")?;
        value
            .parse::<u32>()
            .ok()
            .filter(|n| n.to_string() == value)
            .ok_or_else(|| format!("noncanonical console integer {field}: {value}"))
    }
}

fn expect_live_int(server: &mut Server, field: &str, expected: u32) -> LabResult<()> {
    let observed = server.live_int(field)?;
    if observed != expected {
        return Err(format!(
            "live {field}: expected {expected}, observed {observed}"
        ));
    }
    Ok(())
}

fn record_snapshot(server: &mut Server, stage: &str) -> LabResult<(u64, String)> {
    let clock = server.clock()?;
    let snapshot = server.snapshot()?;
    lab::atomic_json(
        &server.log.with_extension(format!("{stage}.json")),
        &json!({
            "gametime":clock,"snbt":snapshot,
        }),
    )?;
    Ok((clock, snapshot))
}

fn prepare_seed(environment: &Environment, output: &Path) -> LabResult<(PathBuf, Value)> {
    let dir = output.join("seed");
    fs::create_dir(&dir).map_err(|error| error.to_string())?;
    let mut server = launch(
        environment,
        &dir,
        output.join("seed-server"),
        Observer::Absent,
        20,
    )?;
    server.expect("forceload add 0 0", "Marked chunk ")?;
    server.expect(
        &format!("setblock {POS} mekanism:enrichment_chamber"),
        "Changed the block at ",
    )?;
    server.expect(&format!("data merge block {POS} {{items:[{{slot:0b,item:{{id:\"minecraft:iron_ore\",count:1}}}}]}}"), "Modified block data of ")?;
    server.steps(DRY)?;
    let _ = record_snapshot(&mut server, "seed")?;
    let flush_ns = server.save()?;
    let process = server.exit(false)?;
    lab::atomic_json(
        &output.join("seed-timing.json"),
        &json!({"flush_ns":flush_ns,"process":process}),
    )?;
    let witness = disk(&dir, &output.join("seed-disk.json"), 0, false)?;
    Ok((dir, witness))
}

#[derive(Clone, Copy, PartialEq)]
enum Cut {
    Continuous,
    Graceful,
    BeforeFlush,
    AfterFlush,
}

fn trial(
    environment: &Environment,
    output: &Path,
    seed: &Path,
    seed_state: &Value,
    observer: Observer,
    cut: Cut,
    tick_rate: u16,
) -> LabResult<(Value, Value, Option<Value>)> {
    let dir = output.join("instance");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let timer = Instant::now();
    for name in ["world", "config"] {
        copy_tree(&seed.join(name), &dir.join(name)).map_err(|error| error.to_string())?;
    }
    let copy_ns = elapsed_ns(timer);
    let mut server = launch(
        environment,
        &dir,
        output.join("initial"),
        observer,
        tick_rate,
    )?;
    server.expect("save-off", "Automatic saving is now disabled")?;
    let (_, initial) = record_snapshot(&mut server, "initial")?;
    let prefix = pulse(&mut server, CUT)?;
    let (cut_tick, live_cut) = record_snapshot(&mut server, "cut")?;
    expect_live_int(&mut server, "progress", CUT)?;
    expect_live_int(&mut server, "update_delay", 0)?;
    let mut recovery = Value::Null;
    let mut durable_cut = None;
    if cut != Cut::Continuous {
        let saved = cut != Cut::BeforeFlush;
        let flush_ns = saved.then(|| server.save()).transpose()?;
        let process = server.exit(cut != Cut::Graceful)?;
        let durable = disk(
            &dir,
            &output.join("cut-disk.json"),
            if saved { CUT } else { 0 },
            false,
        )?;
        if !saved && &durable != seed_state {
            return Err("pre-flush termination changed the durable seed state".into());
        }
        if saved {
            durable_cut = Some(durable);
        }
        server = launch(
            environment,
            &dir,
            output.join("reopened"),
            observer,
            tick_rate,
        )?;
        server.expect("save-off", "Automatic saving is now disabled")?;
        let (reopen_tick, live_reopen) = record_snapshot(&mut server, "reopen")?;
        let expected = if saved { &live_cut } else { &initial };
        if &live_reopen != expected {
            return Err(format!(
                "reopened machine differs: expected {expected}, observed {live_reopen}"
            ));
        }
        let replay = if saved {
            Value::Null
        } else {
            let replay = pulse(&mut server, CUT)?;
            let (_, replayed) = record_snapshot(&mut server, "replayed")?;
            if replayed != live_cut {
                return Err(format!(
                    "replayed cut differs: expected {live_cut}, observed {replayed}"
                ));
            }
            replay
        };
        recovery = json!({"flush_ns":flush_ns,"process":process,"cut_tick":cut_tick,"reopen_tick":reopen_tick,
            "replay":replay});
    }
    load_energy(&mut server, REMAINING)?;
    let first_step = server.steps(1)?;
    let (_, first_resume) = record_snapshot(&mut server, "first-resumed-tick")?;
    expect_live_int(&mut server, "progress", CUT + 1)?;
    let suffix = json!({
        "first_step":first_step,"active":server.steps(REMAINING - 1)?,"dry":server.steps(DRY)?,
    });
    let (_, terminal) = record_snapshot(&mut server, "terminal")?;
    expect_live_int(&mut server, "progress", 0)?;
    expect_live_int(&mut server, "update_delay", 0)?;
    let flush_ns = server.save()?;
    let process = server.exit(false)?;
    let witness = disk(&dir, &output.join("terminal-disk.json"), 0, true)?;
    let result = json!({"observer":observer.name(),"bridge_enabled":observer == Observer::EveryTick,"tick_rate":tick_rate,"copy_ns":copy_ns,"initial":initial,"cut":live_cut,
        "first_resume":first_resume,"terminal":terminal,"prefix":prefix,"suffix":suffix,"recovery":recovery,
        "terminal_flush_ns":flush_ns,"terminal_process":process,"terminal_nbt":witness,
        "durable_cut_nbt":durable_cut});
    lab::atomic_json(&output.join("trial.json"), &result)?;
    Ok((result, witness, durable_cut))
}

#[derive(Clone, Copy)]
struct Profile {
    repeats: u8,
    tick_rate: u16,
    workers: usize,
    observer_only: bool,
}

impl Profile {
    fn validate(self) -> LabResult<()> {
        if !(1..=3).contains(&self.repeats) {
            return Err("repeat count must be 1..=3".into());
        }
        if !(20..=100).contains(&self.tick_rate) || !(1..=4).contains(&self.workers) {
            return Err("observer controls require 20..=100 ticks/s and 1..=4 workers".into());
        }
        Ok(())
    }

    fn describe(self) -> Value {
        json!({"repeats":self.repeats,"tick_rate":self.tick_rate,"workers":self.workers,
            "scope":if self.observer_only {"continuous_observer_controls"} else {"restart_recovery"}})
    }
}

fn bounded_map<T: Sync, R: Send>(
    jobs: &[T],
    workers: usize,
    execute: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    if workers == 1 {
        return jobs.iter().map(execute).collect();
    }
    jobs.chunks(workers)
        .flat_map(|batch| {
            std::thread::scope(|scope| {
                let execute = &execute;
                let tasks: Vec<_> = batch
                    .iter()
                    .map(|job| scope.spawn(move || execute(job)))
                    .collect();
                tasks
                    .into_iter()
                    .map(|task| task.join().unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .collect()
}

fn campaign(environment: &Environment, output: &Path, profile: Profile) -> LabResult<()> {
    let campaign_start = Instant::now();
    let (seed, seed_state) = prepare_seed(environment, output)?;
    let mut jobs = Vec::new();
    for repeat in 1..=profile.repeats {
        let mut continuous = [
            ("absent-continuous", Observer::Absent, Cut::Continuous),
            ("enabled-every-tick", Observer::EveryTick, Cut::Continuous),
        ];
        if repeat % 2 == 0 {
            continuous.swap(0, 1);
        }
        let mut treatments = Vec::from(continuous);
        if profile.observer_only {
            treatments.insert(
                1,
                ("installed-disabled", Observer::Disabled, Cut::Continuous),
            );
        } else {
            treatments.extend([
                ("graceful-reopen", Observer::Absent, Cut::Graceful),
                ("abrupt-before-flush", Observer::Absent, Cut::BeforeFlush),
                ("abrupt-after-flush", Observer::Absent, Cut::AfterFlush),
            ]);
        }
        jobs.extend(
            treatments
                .into_iter()
                .map(|(name, observer, cut)| (format!("{repeat}-{name}"), observer, cut)),
        );
    }
    let outcomes = bounded_map(&jobs, profile.workers, |(treatment, observer, cut)| {
        let start_ns = elapsed_ns(campaign_start);
        eprintln!("Running {treatment}");
        let result = trial(
            environment,
            &output.join(treatment),
            &seed,
            &seed_state,
            *observer,
            *cut,
            profile.tick_rate,
        );
        let timing = json!({"treatment":treatment,"start_ns":start_ns,"end_ns":elapsed_ns(campaign_start),
            "error":result.as_ref().err()});
        (result, timing)
    });
    let attempts: Vec<_> = outcomes.iter().map(|(_, timing)| timing).collect();
    lab::atomic_json(&output.join("attempts.json"), &attempts)?;
    let mut results = Vec::new();
    let mut reference = None;
    let mut reference_cut = None;
    for ((treatment, _, _), (outcome, _)) in jobs.into_iter().zip(outcomes) {
        let (result, terminal, durable_cut) =
            outcome.map_err(|error| format!("{treatment}: {error}"))?;
        let checkpoints = [
            ("initial", result["initial"].clone()),
            ("cut", result["cut"].clone()),
            ("first_resume", result["first_resume"].clone()),
            ("terminal", result["terminal"].clone()),
            ("terminal_nbt", terminal),
        ];
        let expected = reference.get_or_insert_with(|| checkpoints.clone());
        for ((stage, observed), (_, expected)) in checkpoints.iter().zip(expected.iter()) {
            if observed != expected {
                return Err(format!(
                    "{treatment}: divergent {stage}: expected {expected}, observed {observed}"
                ));
            }
        }
        if let Some(durable_cut) = durable_cut {
            if &durable_cut != reference_cut.get_or_insert_with(|| durable_cut.clone()) {
                return Err(format!("{treatment}: durable checkpoint mismatch"));
            }
        }
        results.push(json!({"treatment":treatment,"evidence":result}));
    }
    lab::atomic_json(
        &output.join("campaign.json"),
        &json!({
            "repeats":profile.repeats,"profile":profile.describe(),"campaign_wall_ns":elapsed_ns(campaign_start),
            "trials":results,
        }),
    )
}

pub fn run(environment_path: &Path, output: &Path, repeats: u8) -> LabResult<()> {
    run_profile(
        environment_path,
        output,
        Profile {
            repeats,
            tick_rate: 20,
            workers: 1,
            observer_only: false,
        },
    )
}

pub fn run_observer_controls(
    environment_path: &Path,
    output: &Path,
    repeats: u8,
    tick_rate: u16,
    workers: usize,
) -> LabResult<()> {
    run_profile(
        environment_path,
        output,
        Profile {
            repeats,
            tick_rate,
            workers,
            observer_only: true,
        },
    )
}

fn run_profile(environment_path: &Path, output: &Path, profile: Profile) -> LabResult<()> {
    profile.validate()?;
    run_locked(
        environment_path,
        output,
        profile.describe(),
        |environment, output| campaign(environment, output, profile),
    )
}

pub fn run_live_observer_audit(environment_path: &Path, output: &Path) -> LabResult<()> {
    run_locked(
        environment_path,
        output,
        json!({"scope":"live_observer_audit","repeats":3,"tick_rate":100,"workers":3}),
        audit::campaign,
    )
}

pub fn run_chunk_controls(environment_path: &Path, output: &Path) -> LabResult<()> {
    run_locked(
        environment_path,
        output,
        json!({"scope":"chunk_observation_controls","repeats":3,"tick_rate":100,"workers":2}),
        chunks::campaign,
    )
}

pub fn run_catalog(environment_path: &Path, output: &Path) -> LabResult<()> {
    run_locked(
        environment_path,
        output,
        json!({"scope":"core_effective_catalog_bootstrap","evidence_level":"bootstrap_diagnostic",
            "repeats":3,"baseline_tick_rate":20,"accelerated_tick_rate":100,"workers":3}),
        catalog::campaign,
    )
}

fn run_locked(
    environment_path: &Path,
    output: &Path,
    profile: Value,
    execute: impl FnOnce(&Environment, &Path) -> LabResult<()>,
) -> LabResult<()> {
    let env_bytes = fs::read(environment_path).map_err(|error| error.to_string())?;
    let environment: Environment =
        serde_json::from_slice(&env_bytes).map_err(|error| error.to_string())?;
    environment.boot_timeout()?;
    let fingerprint = crate::verify_artifacts(&environment.lock)?;
    let runner = crate::sha256_file(&std::env::current_exe().map_err(|error| error.to_string())?)?;
    let declared_runner = environment
        .lock
        .artifacts
        .iter()
        .find(|artifact| artifact.role == "runner")
        .ok_or("missing runner artifact")?;
    if runner != declared_runner.sha256 {
        return Err("executing runner differs from artifact lock".into());
    }
    fs::create_dir(output).map_err(|error| format!("output must be new: {error}"))?;
    let output = std::path::absolute(output).map_err(|error| error.to_string())?;
    fs::write(output.join("environment.json"), env_bytes).map_err(|error| error.to_string())?;
    let result = execute(&environment, &output).and_then(|()| {
        if crate::verify_artifacts(&environment.lock)? != fingerprint {
            return Err("artifact drift during lifecycle execution".into());
        }
        Ok(())
    });
    lab::atomic_json(
        &output.join("result.json"),
        &json!({
            "status":if result.is_err() {"non_pass"} else if profile["evidence_level"] == "bootstrap_diagnostic" {"diagnostic_complete"} else {"functional_pass"},"error":result.as_ref().err(),
            "artifact_fingerprint":fingerprint,"runner_sha256":runner,"repeats":profile["repeats"],"profile":profile,"qualified":false,
            "optimization_proven":false,"port_equivalence_proven":false,
        }),
    )?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn machine(progress: &str, slot: &str, item: &str, count: &str) -> Value {
        json!({"tag":10,"value":{
            "id":{"tag":8,"value":"mekanism:enrichment_chamber"},
            "x":{"tag":3,"value":"0"},"y":{"tag":3,"value":"80"},"z":{"tag":3,"value":"0"},
            "progress":{"tag":3,"value":progress},
            "energy_containers":{"tag":9,"element_tag":10,"value":[
                {"tag":10,"value":{"container":{"tag":1,"value":"0"},"stored":{"tag":4,"value":"0"}}}
            ]},
            "items":{"tag":9,"element_tag":10,"value":[{"tag":10,"value":{
                "slot":{"tag":1,"value":slot},"item":{"tag":10,"value":{
                    "id":{"tag":8,"value":item},"count":{"tag":3,"value":count}
                }}
            }}]}
        }})
    }

    #[test]
    fn recovery_rejects_a_single_lost_progress_tick_and_wrong_width() {
        let mut state = machine("73", "0", "minecraft:iron_ore", "1");
        assert!(validate_machine(&state, 73, false).is_ok());
        assert!(validate_machine(&state, 0, false).is_err());
        assert!(validate_machine(&state, 72, false).is_err());
        state["value"]["progress"]["tag"] = json!(4);
        assert!(validate_machine(&state, 73, false).is_err());
    }

    #[test]
    fn terminal_witness_rejects_duplicate_items_and_wrong_slot() {
        let mut state = machine("0", "1", "mekanism:dust_iron", "2");
        assert!(validate_machine(&state, 0, true).is_ok());
        state["value"]["items"]["value"]
            .as_array_mut()
            .unwrap()
            .push(
                machine("0", "0", "minecraft:iron_ore", "1")["value"]["items"]["value"][0].clone(),
            );
        assert!(validate_machine(&state, 0, true).is_err());
        assert!(validate_machine(&machine("0", "0", "mekanism:dust_iron", "2"), 0, true).is_err());
    }

    #[test]
    fn clock_waits_for_exact_tick_and_rejects_overshoot_regression_and_overflow() {
        assert!(!validate_advance(100, 20, 119).unwrap());
        assert!(validate_advance(100, 20, 120).unwrap());
        assert!(validate_advance(100, 20, 121).is_err());
        assert!(validate_advance(100, 20, 99).is_err());
        assert!(validate_advance(u64::MAX, 1, u64::MAX).is_err());
    }

    #[test]
    fn accepts_the_observed_offline_console_marker_without_accepting_other_messages() {
        let marker = "meklab-boundary-32092-1";
        let raw = "[17:20:01] [Server thread/INFO] [minecraft/MinecraftServer]: [Not Secure] [Server] meklab-boundary-32092-1";
        let spoof = "[17:20:01] [Server thread/INFO] [mekanism/Mekanism]: [Not Secure] [Server] meklab-boundary-32092-1";
        assert!(matches_boundary(server_message(raw).unwrap(), marker));
        assert!(server_message(spoof).is_none());
        assert!(!matches_boundary(
            "[Not Secure] [Server] meklab-boundary-32092-10",
            marker
        ));
    }

    #[test]
    fn console_reader_waits_for_newline_and_rejects_ambiguous_replies() {
        let mut pending = Vec::new();
        assert!(
            read_line(&mut io::Cursor::new(b"partial"), &mut pending)
                .unwrap()
                .is_none()
        );
        assert_eq!(
            read_line(&mut io::Cursor::new(b" message\r\n"), &mut pending)
                .unwrap()
                .as_deref(),
            Some("partial message")
        );
        assert!(
            one_reply(
                &["The time is 17".into(), "The time is 18".into()],
                "The time is "
            )
            .is_err()
        );
    }

    #[test]
    fn disabled_loads_the_pinned_jar_without_an_active_bridge() {
        assert_eq!(Observer::Absent.flags(), (false, false));
        assert_eq!(Observer::Disabled.flags(), (true, false));
        assert_eq!(Observer::EveryTick.flags(), (true, true));
    }
}
