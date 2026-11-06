use crate::{
    ArtifactLock, Datum, Frame, LabResult, Trace, process::ManagedProcess, verify_artifacts,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    pub schema: u32,
    pub lock: ArtifactLock,
    pub server_root: PathBuf,
    pub client_root: PathBuf,
    pub boot_timeout_seconds: u64,
}
impl Environment {
    pub fn boot_timeout(&self) -> LabResult<Duration> {
        if self.schema != 1 || !(1..=900).contains(&self.boot_timeout_seconds) {
            return Err("invalid environment boot budget".into());
        }
        Ok(Duration::from_secs(self.boot_timeout_seconds))
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub client: bool,
    pub integration_level: String,
    pub actions: Vec<Action>,
    #[serde(default = "default_observer_mode")]
    pub observer_mode: String,
    #[serde(default)]
    pub required_server_capabilities: Vec<String>,
    #[serde(default)]
    pub require_persisted_machine: bool,
}
fn default_observer_mode() -> String {
    "every_tick".into()
}
pub fn instance_root(output: &Path) -> LabResult<PathBuf> {
    Ok(std::path::absolute(output)
        .map_err(|e| e.to_string())?
        .join("instances"))
}
pub fn private_homes(instance: &Path) -> LabResult<Vec<(String, String)>> {
    let root = std::path::absolute(instance)
        .map_err(|e| e.to_string())?
        .join("homes");
    let mut env = Vec::new();
    for (key, name) in [
        ("HORIZON_HOME", "game"),
        ("HORIZON_LAUNCHER_HOME", "launcher"),
        ("XDG_DATA_HOME", "xdg-data"),
        ("XDG_CONFIG_HOME", "xdg-config"),
    ] {
        let path = root.join(name);
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        env.push((key.into(), path.to_string_lossy().into_owned()));
    }
    env.push(("HORIZON_LAUNCHER_NO_SELF_UPDATE".into(), "1".into()));
    env.push(("HORIZON_NO_AUDIO".into(), "1".into()));
    Ok(env)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub role: String,
    pub action: String,
    #[serde(default)]
    pub args: serde_json::Map<String, Value>,
    #[serde(default)]
    pub expect: Vec<Assertion>,
    #[serde(default)]
    pub await_condition: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    pub pointer: String,
    pub equals: Value,
}
pub fn observation_tick(state: &Value) -> LabResult<u64> {
    let raw = state
        .get("tick")
        .and_then(Value::as_str)
        .ok_or("missing or non-string observation tick")?;
    let tick = raw.parse::<u64>().map_err(|_| "invalid observation tick")?;
    if tick.to_string() != raw {
        return Err("noncanonical observation tick".into());
    }
    Ok(tick)
}
pub fn frame_tick(state: &Value, phase: &str) -> LabResult<u64> {
    if !crate::is_client_phase(phase) {
        return observation_tick(state);
    }
    observation_tick(state)?;
    let raw = state
        .get("client_tick")
        .and_then(Value::as_str)
        .ok_or("missing or non-string client tick")?;
    let tick = raw.parse::<u64>().map_err(|_| "invalid client tick")?;
    if tick.to_string() != raw {
        return Err("noncanonical client tick".into());
    }
    Ok(tick)
}
impl Case {
    pub fn validate(&self) -> LabResult<()> {
        if !matches!(self.observer_mode.as_str(), "every_tick" | "terminal") {
            return Err("unsupported observer mode".into());
        }
        if self.id.is_empty() || self.integration_level.is_empty() || self.actions.is_empty() {
            return Err("empty scenario identity/actions".into());
        }
        if !self.actions.iter().any(|a| !a.expect.is_empty()) {
            return Err("scenario requires an assertion".into());
        }
        for action in &self.actions {
            let supported = match action.role.as_str() {
                "server" => [
                    "read",
                    "fixture_barrel",
                    "insert_barrel",
                    "setup_player",
                    "step",
                    "advance_until",
                    "setup_machine",
                    "setup_factory",
                    "insert_input",
                    "charge_machine",
                    "save",
                    "drain_machine",
                    "fixture_block_output",
                    "extract_output",
                ]
                .contains(&action.action.as_str()),
                "client" => {
                    if !self.client {
                        return Err("client action requires real client".into());
                    }
                    [
                        "read",
                        "aim",
                        "click_use",
                        "click_slot",
                        "capture",
                        "causal_pickup",
                    ]
                    .contains(&action.action.as_str())
                }
                _ => false,
            };
            if !supported {
                return Err("unsupported scenario action".into());
            }
            if action.action == "causal_pickup" && !action.expect.is_empty() {
                return Err("causal pickup has a fixed scenario contract".into());
            }
            if action.await_condition && action.action != "read" {
                return Err("mutation retry forbidden".into());
            }
            if ["session", "sequence", "action"]
                .iter()
                .any(|key| action.args.contains_key(*key))
            {
                return Err("reserved protocol argument".into());
            }
            let allowed_args: &[&str] = match action.action.as_str() {
                "step" => &["ticks"],
                "setup_factory" => &["blocked"],
                "advance_until" => &["max_ticks", "batch_ticks"],
                "insert_input" => &["side", "slot", "item", "count"],
                "charge_machine" | "drain_machine" => &["side", "amount"],
                "extract_output" => &["side", "slot", "count"],
                "click_slot" => &["selector"],
                _ => &[],
            };
            if action
                .args
                .keys()
                .any(|key| !allowed_args.contains(&key.as_str()))
            {
                return Err("unsupported action argument".into());
            }
            if action.action == "click_slot"
                && !matches!(
                    action.args.get("selector").and_then(Value::as_str),
                    Some("source" | "input" | "output")
                )
            {
                return Err("invalid slot selector".into());
            }
            if action.action == "setup_factory"
                && (self.client
                    || action
                        .args
                        .get("blocked")
                        .and_then(Value::as_bool)
                        .is_none())
            {
                return Err(
                    "factory setup requires a dedicated server and a boolean blocked flag".into(),
                );
            }
            if action.action == "step"
                && !matches!(
                    action.args.get("ticks").and_then(Value::as_u64),
                    Some(1..=20)
                )
            {
                return Err("tick budget outside 1..20".into());
            }
            if action.action == "advance_until" {
                if !matches!(
                    action.args.get("max_ticks").and_then(Value::as_u64),
                    Some(1..=1000)
                ) || !matches!(
                    action.args.get("batch_ticks").and_then(Value::as_u64),
                    Some(1..=20)
                ) {
                    return Err("invalid advance tick budget".into());
                }
                if action.expect.is_empty() {
                    return Err("advance requires goal assertion".into());
                }
            }
            if matches!(
                action.action.as_str(),
                "insert_input" | "charge_machine" | "drain_machine" | "extract_output"
            ) && !matches!(
                action.args.get("side").and_then(Value::as_str),
                Some("unsided" | "down" | "up" | "north" | "south" | "west" | "east")
            ) {
                return Err("invalid capability side".into());
            }
            if matches!(action.action.as_str(), "insert_input" | "extract_output") {
                if !matches!(
                    action.args.get("count").and_then(Value::as_u64),
                    Some(1..=64)
                ) {
                    return Err("invalid input count".into());
                }
                if !matches!(
                    action.args.get("slot").and_then(Value::as_u64),
                    Some(0..=63)
                ) {
                    return Err("invalid input slot".into());
                }
                if action.action == "insert_input"
                    && !action
                        .args
                        .get("item")
                        .and_then(Value::as_str)
                        .is_some_and(|s| {
                            s.contains(':')
                                && s.bytes().all(|b| {
                                    b.is_ascii_lowercase()
                                        || b.is_ascii_digit()
                                        || b"_:/.-".contains(&b)
                                })
                        })
                {
                    return Err("invalid item identifier".into());
                }
            }
            if matches!(action.action.as_str(), "charge_machine" | "drain_machine")
                && !matches!(
                    action.args.get("amount").and_then(Value::as_u64),
                    Some(1..=1_000_000)
                )
            {
                return Err("invalid FE amount".into());
            }
            if action.expect.iter().any(|a| !a.pointer.starts_with('/')) {
                return Err("assertion requires absolute JSON pointer".into());
            }
        }
        Ok(())
    }
}

pub(crate) struct Session {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
    token: String,
    next: u64,
}
impl Session {
    fn new(stream: TcpStream, token: &str) -> LabResult<(Self, Value)> {
        stream.set_nonblocking(false).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        let writer = stream.try_clone().map_err(|e| e.to_string())?;
        let mut session = Self {
            reader: BufReader::new(stream),
            writer,
            token: token.into(),
            next: 0,
        };
        let mut hello = session.read()?;
        if hello["kind"] != "hello" || hello["protocol"] != 1 || hello["session"] != token {
            return Err("invalid bridge hello".into());
        }
        hello
            .as_object_mut()
            .ok_or("invalid hello")?
            .remove("session");
        Ok((session, hello))
    }
    fn read(&mut self) -> LabResult<Value> {
        let mut bytes = Vec::new();
        self.reader
            .by_ref()
            .take(1_048_577)
            .read_until(b'\n', &mut bytes)
            .map_err(|e| format!("bridge read: {e}"))?;
        if bytes.is_empty() || bytes.len() > 1_048_576 || bytes.last() != Some(&b'\n') {
            return Err("truncated or oversize bridge frame".into());
        }
        serde_json::from_slice(&bytes).map_err(|e| format!("bridge JSON: {e}"))
    }
    pub(crate) fn request(
        &mut self,
        action: &str,
        args: &serde_json::Map<String, Value>,
    ) -> LabResult<Value> {
        if args.contains_key("session")
            || args.contains_key("sequence")
            || args.contains_key("action")
        {
            return Err("reserved action argument".into());
        }
        let mut message = args.clone();
        message.insert("session".into(), json!(self.token));
        message.insert("sequence".into(), json!(self.next));
        message.insert("action".into(), json!(action));
        serde_json::to_writer(&mut self.writer, &message).map_err(|e| e.to_string())?;
        self.writer.write_all(b"\n").map_err(|e| e.to_string())?;
        let reply = self.read()?;
        if reply["kind"] != "observation"
            || reply["sequence"] != self.next
            || reply["action"] != action
        {
            return Err("bridge sequence/action mismatch".into());
        }
        self.next += 1;
        if let Some(error) = reply["state"].get("error") {
            return Err(format!("bridge action rejected: {error}"));
        }
        Ok(reply)
    }
}

pub(crate) fn accept(
    listener: &TcpListener,
    child: &mut ManagedProcess,
    token: &str,
    timeout: Duration,
    role: &str,
) -> LabResult<(Session, Value)> {
    let start = Instant::now();
    loop {
        match listener.accept() {
            Ok((socket, address)) => {
                if !address.ip().is_loopback() {
                    return Err("nonlocal bridge".into());
                }
                let (session, hello) = Session::new(socket, token)?;
                if hello["role"] != role {
                    return Err("wrong instance role".into());
                }
                return Ok((session, hello));
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error.to_string()),
        }
        if child.try_wait()?.is_some() {
            return Err(format!("{role} exited before bridge ready"));
        }
        if start.elapsed() > timeout {
            return Err(format!("{role} boot timeout"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
pub(crate) fn role<'a>(environment: &'a Environment, name: &str) -> LabResult<&'a Path> {
    environment
        .lock
        .artifacts
        .iter()
        .find(|a| a.role == name)
        .map(|a| a.path.as_path())
        .ok_or_else(|| format!("missing artifact role: {name}"))
}
pub(crate) fn atomic_json(path: &Path, value: &impl Serialize) -> LabResult<()> {
    let temp = path.with_extension("partial");
    let mut file = fs::File::create(&temp).map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        match fs::rename(&temp, path) {
            Ok(()) => return Ok(()),
            Err(error)
                if cfg!(windows)
                    && matches!(error.raw_os_error(), Some(5 | 32 | 33))
                    && Instant::now() < deadline =>
            {
                std::thread::sleep(Duration::from_millis(10))
            }
            Err(error) => return Err(format!("publish {}: {error}", path.display())),
        }
    }
}
pub fn retain_observation(
    output: &Path,
    observations: &mut Vec<Value>,
    observation: &Value,
) -> LabResult<usize> {
    let index = observations.len();
    observations.push(observation.clone());
    atomic_json(&output.join("observations.json"), observations)?;
    if let Some(machine) = observation["state"].get("machine") {
        crate::witness::inventory(machine)?;
    }
    if let Some(samples) = observation["tick_samples"].as_array() {
        for sample in samples {
            if let Some(machine) = sample["state"].get("machine") {
                crate::witness::inventory(machine)?;
            }
        }
    }
    Ok(index)
}
fn datum(value: &Value) -> LabResult<Datum> {
    Ok(match value {
        Value::String(v) => Datum::String { value: v.clone() },
        Value::Bool(v) => Datum::Bool { value: *v },
        Value::Number(v) => Datum::I64 {
            value: v
                .as_i64()
                .ok_or("non-i64 observation requires typed protocol")?
                .to_string(),
        },
        Value::Array(v) => Datum::List {
            values: v.iter().map(datum).collect::<LabResult<_>>()?,
        },
        Value::Object(v) => Datum::Compound {
            entries: v
                .iter()
                .map(|(k, v)| Ok((k.clone(), datum(v)?)))
                .collect::<LabResult<_>>()?,
        },
        Value::Null => return Err("null observation is not absent".into()),
    })
}
fn require_offscreen_bridge(jar: &[u8]) -> LabResult<()> {
    // ZIP entry names are stored verbatim. The artifact lock verifies the exact jar bytes.
    for entry in [
        "META-INF/meklab.offscreen.mixins.json",
        "dev/meklab/mixin/OffscreenWindowMixin.class",
    ] {
        if !jar
            .windows(entry.len())
            .any(|window| window == entry.as_bytes())
        {
            return Err("client profile requires the pinned offscreen bridge jar".into());
        }
    }
    Ok(())
}

pub fn run(environment_path: &Path, case_path: &Path, output: &Path) -> LabResult<()> {
    use sha2::{Digest, Sha256};
    let environment_bytes = fs::read(environment_path).map_err(|e| e.to_string())?;
    let environment: Environment =
        serde_json::from_slice(&environment_bytes).map_err(|e| e.to_string())?;
    let case_bytes = fs::read(case_path).map_err(|e| e.to_string())?;
    let case: Case = serde_json::from_slice(&case_bytes).map_err(|e| e.to_string())?;
    environment.boot_timeout()?;
    case.validate()?;
    let fingerprint = verify_artifacts(&environment.lock)?;
    if case.client {
        require_offscreen_bridge(
            &fs::read(role(&environment, "bridge")?).map_err(|e| e.to_string())?,
        )?;
    }
    let case_sha256 = format!("{:x}", Sha256::digest(&case_bytes));
    let runner_sha256 = crate::sha256_file(&std::env::current_exe().map_err(|e| e.to_string())?)?;
    let declared_runner = environment
        .lock
        .artifacts
        .iter()
        .find(|a| a.role == "runner")
        .ok_or("missing runner artifact")?;
    if declared_runner.sha256 != runner_sha256 {
        return Err("executing runner differs from artifact lock".into());
    }
    let execution_fingerprint=format!("{:x}",Sha256::digest(serde_json::to_vec(&json!({"artifacts":fingerprint,"case":case_sha256,"runner":runner_sha256,"os":std::env::consts::OS,"arch":std::env::consts::ARCH})).map_err(|e|e.to_string())?));
    fs::create_dir(output).map_err(|e| format!("output must be new: {e}"))?;
    fs::write(output.join("case.json"), &case_bytes).map_err(|e| e.to_string())?;
    fs::write(output.join("environment.json"), &environment_bytes).map_err(|e| e.to_string())?;
    let attempt_id = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos()
    );
    atomic_json(
        &output.join("run.json"),
        &json!({"schema":2,"attempt_id":attempt_id,"status":"running","qualified":false,"case":case.id,"artifact_fingerprint":fingerprint,"execution_fingerprint":execution_fingerprint,"case_sha256":case_sha256,"runner_sha256":runner_sha256}),
    )?;
    let result = execute(&environment, &case, output, &execution_fingerprint).and_then(|_| {
        if verify_artifacts(&environment.lock)? != fingerprint {
            return Err("artifact drift during execution".into());
        }
        crate::trace_seal::for_file(&output.join("trace.json"))
    });
    let status = if result.is_ok() {
        "functional_pass"
    } else {
        "non_pass"
    };
    atomic_json(
        &output.join("result.json"),
        &json!({"schema":2,"attempt_id":attempt_id,"status":status,"qualified":false,"optimization_proven":false,"port_equivalence_proven":false,"integration_level":case.integration_level,"case":case.id,"case_sha256":case_sha256,"error":result.as_ref().err(),"artifact_fingerprint":fingerprint,"execution_fingerprint":execution_fingerprint,"trace_seal":result.as_ref().ok()}),
    )?;
    result.map(drop)
}
pub(crate) fn prepare_server(
    environment: &Environment,
    server_dir: &Path,
    game_port: u16,
    bridge_enabled: bool,
) -> LabResult<BTreeMap<String, String>> {
    let properties = format!(
        "server-ip=127.0.0.1\nserver-port={game_port}\nonline-mode=false\nenforce-secure-profile=false\nlevel-seed=12345\nlevel-type=minecraft\\:flat\ngenerator-settings={{\"layers\":[{{\"block\":\"minecraft:bedrock\",\"height\":1}},{{\"block\":\"minecraft:dirt\",\"height\":2}},{{\"block\":\"minecraft:grass_block\",\"height\":1}}],\"biome\":\"minecraft:plains\"}}\nview-distance=2\nsimulation-distance=2\nspawn-protection=0\ndifficulty=peaceful\ngenerate-structures=false\nmax-players=3\n"
    );
    fs::write(server_dir.join("server.properties"), &properties).map_err(|e| e.to_string())?;
    fs::write(server_dir.join("eula.txt"), "eula=true\n").map_err(|e| e.to_string())?;
    let mod_dir = server_dir.join("mods");
    fs::create_dir_all(&mod_dir).map_err(|e| e.to_string())?;
    let names: &[&str] = if bridge_enabled {
        &["bridge", "mekanism"]
    } else {
        &["mekanism"]
    };
    let mut mods = BTreeMap::new();
    for name in names {
        let source = role(environment, name)?;
        let filename = format!("{name}.jar");
        let dest = mod_dir.join(&filename);
        if !dest.exists() {
            fs::copy(source, &dest).map_err(|e| e.to_string())?;
        }
        let hash = crate::sha256_file(&dest)?;
        let expected = environment
            .lock
            .artifacts
            .iter()
            .find(|artifact| artifact.role == *name)
            .ok_or("missing mod role")?;
        if hash != expected.sha256 {
            return Err(format!(
                "installed mod differs from artifact lock: {filename}"
            ));
        }
        mods.insert(filename, hash);
    }
    for entry in fs::read_dir(&mod_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_file()
            || !mods.contains_key(entry.file_name().to_string_lossy().as_ref())
        {
            return Err("unexpected entry in isolated mod directory".into());
        }
    }
    let original = fs::read_to_string(
        environment
            .server_root
            .join("libraries/net/neoforged/neoforge/21.1.200/win_args.txt"),
    )
    .map_err(|e| e.to_string())?;
    let library_root = format!(
        "{}/libraries",
        environment.server_root.to_string_lossy().replace('\\', "/")
    );
    let launch = original
        .replace("libraries/", &format!("{library_root}/"))
        .replace(
            "-DlibraryDirectory=libraries",
            &format!("-DlibraryDirectory={library_root}"),
        );
    fs::write(server_dir.join("launch.args"), launch).map_err(|e| e.to_string())?;
    Ok(mods)
}

fn execute(
    environment: &Environment,
    case: &Case,
    output: &Path,
    fingerprint: &str,
) -> LabResult<()> {
    let scratch = instance_root(output)?;
    fs::create_dir(&scratch).map_err(|e| e.to_string())?;
    // Windows AF_UNIX socket paths have a short limit. Never inherit a long user TEMP.
    let temp_root = environment
        .server_root
        .parent()
        .and_then(Path::parent)
        .ok_or("runtime root has no parent")?
        .join("tmp");
    fs::create_dir_all(&temp_root).map_err(|e| e.to_string())?;
    let java_temp = tempfile::Builder::new()
        .prefix("s")
        .tempdir_in(&temp_root)
        .map_err(|e| e.to_string())?;
    let client_temp = tempfile::Builder::new()
        .prefix("c")
        .tempdir_in(&temp_root)
        .map_err(|e| e.to_string())?;
    let server_dir = scratch.join("server");
    fs::create_dir(&server_dir).map_err(|e| e.to_string())?;
    let control = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    control.set_nonblocking(true).map_err(|e| e.to_string())?;
    let control_port = control.local_addr().map_err(|e| e.to_string())?.port();
    let reservation = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let game_port = reservation.local_addr().map_err(|e| e.to_string())?.port();
    drop(reservation);
    let token = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos()
    );
    prepare_server(environment, &server_dir, game_port, true)?;
    let temp_value = java_temp.path().to_string_lossy().into_owned();
    let mut env = vec![
        ("MEKLAB_PORT".into(), control_port.to_string()),
        ("MEKLAB_SESSION".into(), token.clone()),
        ("MEKLAB_ROLE".into(), "server".into()),
        ("TEMP".into(), temp_value.clone()),
        ("TMP".into(), temp_value.clone()),
    ];
    env.extend(private_homes(&server_dir)?);
    let args = vec![
        "-Xmx2G".into(),
        "-Djava.net.preferIPv4Stack=true".into(),
        format!("-Djdk.net.unixdomain.tmpdir={temp_value}"),
        format!("-Djava.io.tmpdir={temp_value}"),
        "@launch.args".into(),
        "nogui".into(),
    ];
    let mut server = ManagedProcess::spawn(
        role(environment, "java")?,
        &args,
        &server_dir,
        &env,
        &output.join("server"),
    )?;
    let timeout = environment.boot_timeout()?;
    let (mut server_session, hello) = accept(&control, &mut server, &token, timeout, "server")?;
    // The mod declares 10.7.19; build 85 is pinned by the artifact hash, not its display version.
    if hello["minecraft"] != "1.21.1" || hello["mekanism"] != "10.7.19" {
        return Err(format!(
            "loaded reference differs from pinned version: {hello}"
        ));
    }
    atomic_json(&output.join("server-hello.json"), &hello)?;
    for required in &case.required_server_capabilities {
        if !hello["capabilities"]
            .as_array()
            .is_some_and(|caps| caps.iter().any(|c| c.as_str() == Some(required)))
        {
            return Err(format!("missing server capability: {required}"));
        }
    }
    let config_fingerprints = config_hashes(&server_dir)?;
    atomic_json(&output.join("effective-config.json"), &config_fingerprints)?;
    use sha2::{Digest, Sha256};
    let mut effective_fingerprint=format!("{:x}",Sha256::digest(serde_json::to_vec(&json!({"execution":fingerprint,"configs":config_fingerprints,"fixture":"flat-seed-12345-frozen-v1","port_policy":"ephemeral_loopback"})).map_err(|e|e.to_string())?));
    let mut client_process = None;
    let mut client_session = None;
    let mut client_configs = None;
    if case.client {
        let client_dir = scratch.join("client-p1");
        fs::create_dir(&client_dir).map_err(|e| e.to_string())?;
        fs::create_dir(client_dir.join("mods")).map_err(|e| e.to_string())?;
        for name in ["bridge", "mekanism"] {
            fs::copy(
                role(environment, name)?,
                client_dir.join("mods").join(format!("{name}.jar")),
            )
            .map_err(|e| e.to_string())?;
        }
        fs::write(client_dir.join("options.txt"),"pauseOnLostFocus:false\nfullscreen:false\nrenderDistance:2\nsimulationDistance:5\nsoundCategory_master:0.0\nskipMultiplayerWarning:true\nonboardAccessibility:false\n").map_err(|e|e.to_string())?;
        fs::create_dir(client_dir.join("config")).map_err(|e| e.to_string())?;
        fs::write(
            client_dir.join("config/fml.toml"),
            "earlyWindowControl = false\n",
        )
        .map_err(|e| e.to_string())?;
        atomic_json(
            &output.join("client-profile.json"),
            &json!({"loader_splash_enabled":false,"presentation":"hidden_glfw","render_distance":2,"simulation_distance":5,"master_volume":0,"options_seed_sha256":crate::sha256_file(&client_dir.join("options.txt"))?,"loader_seed_sha256":crate::sha256_file(&client_dir.join("config/fml.toml"))?}),
        )?;
        let private_natives = client_dir.join("natives");
        fs::create_dir(&private_natives).map_err(|e| e.to_string())?;
        for entry in
            fs::read_dir(environment.client_root.join("natives")).map_err(|e| e.to_string())?
        {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().extension().is_some_and(|e| e == "dll") {
                fs::copy(entry.path(), private_natives.join(entry.file_name()))
                    .map_err(|e| e.to_string())?;
            }
        }
        let launch: Value = serde_json::from_slice(
            &fs::read(environment.client_root.join("launch.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let temp_value = client_temp.path().to_string_lossy().into_owned();
        let mut args = vec![
            "-Xmx2G".into(),
            "-Djava.net.preferIPv4Stack=true".into(),
            format!("-Djdk.net.unixdomain.tmpdir={temp_value}"),
            format!("-Djava.io.tmpdir={temp_value}"),
        ];
        for argument in launch["args"].as_array().ok_or("client args absent")? {
            args.push(
                argument
                    .as_str()
                    .ok_or("invalid client arg")?
                    .replace("{game_dir}", &client_dir.to_string_lossy())
                    .replace("{server_address}", &format!("127.0.0.1:{game_port}"))
                    .replace(
                        &environment
                            .client_root
                            .join("natives")
                            .to_string_lossy()
                            .to_string(),
                        &private_natives.to_string_lossy(),
                    ),
            );
        }
        let mut env = vec![
            ("MEKLAB_PORT".into(), control_port.to_string()),
            ("MEKLAB_SESSION".into(), token.clone()),
            ("MEKLAB_ROLE".into(), "client".into()),
            ("MEKLAB_OFFSCREEN".into(), "1".into()),
            ("TEMP".into(), temp_value.clone()),
            ("TMP".into(), temp_value.clone()),
        ];
        env.extend(private_homes(&client_dir)?);
        let mut child = ManagedProcess::spawn(
            role(environment, "java")?,
            &args,
            &client_dir,
            &env,
            &output.join("client"),
        )?;
        let (session, hello) = accept(&control, &mut child, &token, timeout, "client")?;
        atomic_json(&output.join("client-hello.json"), &hello)?;
        if hello["presentation"] != "hidden_glfw" {
            return Err("client did not attest to a hidden GLFW window".into());
        }
        let hashes = config_hashes(&client_dir)?;
        atomic_json(&output.join("client-effective-config.json"), &hashes)?;
        effective_fingerprint=format!("{:x}",Sha256::digest(serde_json::to_vec(&json!({"server":effective_fingerprint,"client_configs":hashes,"client_profile":"hidden-glfw-no-loader-splash-v1"})).map_err(|e|e.to_string())?));
        client_configs = Some((client_dir, hashes));
        client_session = Some(session);
        client_process = Some(child);
    }
    let mut trace = Trace {
        schema: 1,
        scenario: case.id.clone(),
        environment: effective_fingerprint,
        complete: false,
        frames: Vec::new(),
    };
    let mut observations = Vec::new();
    let mut measurements = Vec::new();
    let mut last_server_state =
        server_session.request("read", &Default::default())?["state"].clone();
    for action in &case.actions {
        if action.action == "causal_pickup" {
            last_server_state = crate::causal::run(
                &mut server_session,
                client_session
                    .as_mut()
                    .ok_or("causal pickup requires a client")?,
                output,
                &mut trace,
            )?;
            continue;
        }
        let session = match action.role.as_str() {
            "server" => &mut server_session,
            "client" => client_session
                .as_mut()
                .ok_or("client action without client")?,
            _ => return Err("unknown action role".into()),
        };
        if action.await_condition && action.action != "read" {
            return Err("only read may be repeated; mutation retry forbidden".into());
        }
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut advanced = 0u64;
        loop {
            let stepping = matches!(action.action.as_str(), "step" | "advance_until");
            let mut args = action.args.clone();
            let requested = if action.action == "advance_until" {
                let remaining = action.args["max_ticks"].as_u64().unwrap() - advanced;
                let n = remaining.min(action.args["batch_ticks"].as_u64().unwrap());
                args.clear();
                args.insert("ticks".into(), json!(n));
                n
            } else if stepping {
                action.args["ticks"].as_u64().unwrap()
            } else {
                0
            };
            if stepping {
                args.insert("observer_mode".into(), json!(case.observer_mode));
            }
            let observation =
                session.request(if stepping { "step" } else { &action.action }, &args)?;
            let observation_index = retain_observation(output, &mut observations, &observation)?;
            let state = &observation["state"];
            if stepping {
                let samples: Vec<crate::capture::TickSample> =
                    serde_json::from_value(observation["tick_samples"].clone())
                        .map_err(|e| format!("tick samples: {e}"))?;
                crate::capture::validate_batch(
                    observation_tick(&last_server_state)?,
                    requested,
                    &case.observer_mode,
                    &samples,
                    state,
                )?;
                crate::measurement::validate(
                    &observation["measurement"],
                    observation_tick(&last_server_state)?,
                    requested,
                    &case.observer_mode,
                )?;
                for sample in samples {
                    push_frame(&mut trace, &sample.state, "end_server_tick")?;
                }
                measurements.push(json!({"action_index":observation_index,"observer_mode":case.observer_mode,"measurement":observation["measurement"]}));
                atomic_json(&output.join("measurements.json"), &measurements)?;
                advanced += requested;
            }
            if action.role == "server" {
                last_server_state = state.clone();
            }
            let failed = action
                .expect
                .iter()
                .find(|expected| state.pointer(&expected.pointer) != Some(&expected.equals));
            if let Some(failed) = failed {
                if action.action == "advance_until"
                    && advanced < action.args["max_ticks"].as_u64().unwrap()
                {
                    continue;
                }
                if action.await_condition && Instant::now() < deadline {
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }
                return Err(format!(
                    "functional assertion {}: expected {}, actual {:?}",
                    failed.pointer,
                    failed.equals,
                    state.pointer(&failed.pointer)
                ));
            }
            if !stepping {
                push_frame(
                    &mut trace,
                    state,
                    if action.role == "client" {
                        "client_applied"
                    } else {
                        "end_server_tick"
                    },
                )?;
            }
            break;
        }
    }
    if let Some(client) = client_session.as_mut() {
        client.request("close", &Default::default())?;
    }
    if let Some(client) = client_process.as_mut() {
        if !client.wait_timeout(Duration::from_secs(30))?.success() {
            return Err("client exited unsuccessfully".into());
        }
    }
    server_session.request("close", &Default::default())?;
    if !server.wait_timeout(Duration::from_secs(30))?.success() {
        return Err("server exited unsuccessfully".into());
    }
    if case.require_persisted_machine {
        let saved = crate::nbt::block_entity(&server_dir.join("world/region/r.0.0.mca"), 0, 80, 0)?;
        atomic_json(&output.join("persisted-machine.json"), &saved)?;
        let projected = crate::nbt::persisted_projection(&saved)?;
        atomic_json(&output.join("persisted-machine-projected.json"), &projected)?;
        let key = if case
            .actions
            .iter()
            .any(|action| action.action == "setup_factory")
        {
            "factory"
        } else {
            "machine"
        };
        if projected != last_server_state[key]["nbt"] {
            return Err("independent persisted machine differs from final live NBT".into());
        }
    }
    if !measurements.is_empty() {
        atomic_json(
            &output.join("performance.json"),
            &crate::measurement::summarize(&measurements)?,
        )?;
    }
    if config_hashes(&server_dir)? != config_fingerprints {
        return Err("effective configuration changed during scenario".into());
    }
    if let Some((directory, hashes)) = client_configs {
        if config_hashes(&directory)? != hashes {
            return Err("effective client configuration changed during scenario".into());
        }
    }
    trace.complete = true;
    crate::validate_trace(&trace)?;
    atomic_json(&output.join("trace.json"), &trace)?;
    Ok(())
}

pub(crate) fn push_frame(trace: &mut Trace, state: &Value, phase: &str) -> LabResult<()> {
    let object = state.as_object().ok_or("snapshot not an object")?;
    trace.frames.push(Frame {
        sequence: trace.frames.len() as u64,
        tick: frame_tick(state, phase)?,
        phase: phase.into(),
        state: object
            .iter()
            .map(|(k, v)| Ok((k.clone(), datum(v)?)))
            .collect::<LabResult<BTreeMap<_, _>>>()?,
    });
    Ok(())
}

pub(crate) fn config_hashes(server_dir: &Path) -> LabResult<BTreeMap<String, String>> {
    fn walk(root: &Path, directory: &Path, result: &mut BTreeMap<String, String>) -> LabResult<()> {
        if !directory.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() {
                return Err("config symlink unsupported".into());
            }
            if kind.is_dir() {
                walk(root, &entry.path(), result)?;
            } else if kind.is_file() {
                result.insert(
                    entry
                        .path()
                        .strip_prefix(root)
                        .map_err(|e| e.to_string())?
                        .to_string_lossy()
                        .replace('\\', "/"),
                    crate::sha256_file(&entry.path())?,
                );
            }
        }
        Ok(())
    }
    let mut result = BTreeMap::new();
    walk(server_dir, &server_dir.join("config"), &mut result)?;
    walk(
        server_dir,
        &server_dir.join("world/serverconfig"),
        &mut result,
    )?;
    Ok(result)
}

#[cfg(test)]
mod transport_tests {
    use super::*;
    #[cfg(windows)]
    #[test]
    fn atomic_publish_waits_for_transient_read_lock() {
        use std::os::windows::fs::OpenOptionsExt;
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("observation.json");
        fs::write(&path, b"{\"old\":true}").unwrap();
        let (sent, received) = std::sync::mpsc::channel();
        let reader_path = path.clone();
        let reader = std::thread::spawn(move || {
            let handle = fs::OpenOptions::new()
                .read(true)
                .share_mode(1)
                .open(reader_path)
                .unwrap();
            sent.send(()).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            drop(handle);
        });
        received.recv_timeout(Duration::from_secs(2)).unwrap();
        let result = atomic_json(&path, &json!({"new":true}));
        reader.join().unwrap();
        result.unwrap();
        let saved: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        assert_eq!(saved, json!({"new":true}));
    }
    #[test]
    fn accepted_socket_waits_for_fragmented_delayed_hello() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let peer = std::thread::spawn(move || {
            let mut socket = TcpStream::connect(address).unwrap();
            std::thread::sleep(Duration::from_millis(40));
            socket
                .write_all(b"{\"kind\":\"hello\",\"protocol\":1,")
                .unwrap();
            std::thread::sleep(Duration::from_millis(40));
            socket
                .write_all(b"\"session\":\"fixture\",\"role\":\"server\"}\n")
                .unwrap();
        });
        let deadline = Instant::now() + Duration::from_secs(2);
        let socket = loop {
            assert!(Instant::now() < deadline, "fixture connection timeout");
            match listener.accept() {
                Ok((socket, _)) => break socket,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5))
                }
                Err(e) => panic!("{e}"),
            }
        };
        let (_, hello) = Session::new(socket, "fixture").unwrap();
        assert_eq!(hello["role"], "server");
        peer.join().unwrap();
    }
}

#[cfg(test)]
mod presentation_tests {
    use super::require_offscreen_bridge;

    #[test]
    fn refuses_a_client_bridge_without_both_offscreen_entries() {
        let config = b"META-INF/meklab.offscreen.mixins.json";
        let hook = b"dev/meklab/mixin/OffscreenWindowMixin.class";
        assert!(require_offscreen_bridge(b"old laboratory bridge").is_err());
        assert!(require_offscreen_bridge(config).is_err());
        assert!(require_offscreen_bridge(hook).is_err());
        let current = [config.as_slice(), hook.as_slice()].concat();
        assert!(require_offscreen_bridge(&current).is_ok());
    }
}
