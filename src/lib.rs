use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
pub mod benchmark;
pub mod capture;
mod causal;
pub mod lab;
pub mod lifecycle;
pub mod measurement;
pub mod nbt;
pub mod observer;
pub mod observer_faults;
pub mod platform;
pub mod process;
pub mod trace_seal;
pub mod witness;
pub mod work;

pub type LabResult<T> = Result<T, String>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Datum {
    #[serde(rename = "i64")]
    I64 { value: String },
    #[serde(rename = "f64")]
    F64 { bits: String },
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "bool")]
    Bool { value: bool },
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "compound")]
    Compound { entries: BTreeMap<String, Datum> },
    #[serde(rename = "list")]
    List { values: Vec<Datum> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    pub sequence: u64,
    pub tick: u64,
    pub phase: String,
    pub state: BTreeMap<String, Datum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    pub schema: u32,
    pub scenario: String,
    pub environment: String,
    pub complete: bool,
    pub frames: Vec<Frame>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Comparison {
    pub equal: bool,
    pub first_difference: Option<String>,
    pub compared_frames: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub role: String,
    pub path: PathBuf,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactLock {
    pub schema: u32,
    pub artifacts: Vec<Artifact>,
}

pub fn verify_artifacts(lock: &ArtifactLock) -> LabResult<String> {
    use sha2::{Digest, Sha256};
    if lock.schema != 1 {
        return Err("unsupported artifact schema".into());
    }
    if lock.artifacts.is_empty() {
        return Err("empty artifact lock".into());
    }
    let mut identities = BTreeMap::new();
    for artifact in &lock.artifacts {
        if artifact.role.is_empty() {
            return Err("empty artifact role".into());
        }
        if identities.contains_key(&artifact.role) {
            return Err("duplicate artifact role".into());
        }
        if artifact.sha256.len() != 64 || !artifact.sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(format!("invalid hash: {}", artifact.role));
        }
        let actual = sha256_file(&artifact.path)?;
        if actual != artifact.sha256.to_ascii_lowercase() {
            return Err(format!("hash mismatch: {}", artifact.role));
        }
        identities.insert(&artifact.role, actual);
    }
    let encoded = serde_json::to_vec(&identities).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(encoded)))
}

pub fn compare_traces(expected: &Trace, actual: &Trace) -> LabResult<Comparison> {
    validate_trace(expected)?;
    validate_trace(actual)?;
    if expected.environment != actual.environment {
        return Err("environment differs; explicit comparison profile required".into());
    }
    if expected.scenario != actual.scenario {
        return Err("scenario differs".into());
    }
    for (index, (a, b)) in expected.frames.iter().zip(&actual.frames).enumerate() {
        let prefix = format!("frames[{index}]");
        let difference = if a.tick != b.tick {
            Some(format!("{prefix}.tick"))
        } else if a.phase != b.phase {
            Some(format!("{prefix}.phase"))
        } else {
            diff_map(&a.state, &b.state, &format!("{prefix}.state"))
        };
        if let Some(path) = difference {
            return Ok(Comparison {
                equal: false,
                first_difference: Some(path),
                compared_frames: index + 1,
            });
        }
    }
    let same_len = expected.frames.len() == actual.frames.len();
    Ok(Comparison {
        equal: same_len,
        first_difference: (!same_len).then(|| "frames.length".into()),
        compared_frames: expected.frames.len().min(actual.frames.len()),
    })
}

pub fn sha256_file(path: &std::path::Path) -> LabResult<String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(|e| format!("artifact read: {e}"))?;
    let mut hash = Sha256::new();
    let mut bytes = [0u8; 65536];
    loop {
        let count = file.read(&mut bytes).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        hash.update(&bytes[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

pub(crate) fn is_client_phase(phase: &str) -> bool {
    matches!(phase, "client_sent" | "client_applied" | "client_rendered")
}

fn validate_trace(trace: &Trace) -> LabResult<()> {
    if trace.schema != 1 {
        return Err("unsupported trace schema".into());
    }
    if !trace.complete {
        return Err("incomplete trace".into());
    }
    if trace.scenario.is_empty() || trace.environment.is_empty() {
        return Err("empty trace identity".into());
    }
    if trace.frames.is_empty() {
        return Err("empty trace".into());
    }
    let mut last_tick = [0u64; 2];
    for (index, frame) in trace.frames.iter().enumerate() {
        if frame.sequence != index as u64 {
            return Err("sequence gap".into());
        }
        let domain = usize::from(is_client_phase(&frame.phase));
        if frame.tick < last_tick[domain] {
            return Err("tick regression".into());
        }
        if ![
            "before_action",
            "after_action",
            "end_server_tick",
            "client_sent",
            "server_applied",
            "client_applied",
            "client_rendered",
        ]
        .contains(&frame.phase.as_str())
        {
            return Err("unknown phase".into());
        }
        if frame.state.is_empty() {
            return Err("empty snapshot".into());
        }
        for value in frame.state.values() {
            validate_datum(value)?;
        }
        last_tick[domain] = frame.tick;
    }
    Ok(())
}

fn validate_datum(value: &Datum) -> LabResult<()> {
    match value {
        Datum::I64 { value } => {
            let parsed = value
                .parse::<i64>()
                .map_err(|_| "invalid i64".to_string())?;
            if parsed.to_string() != *value {
                return Err("noncanonical i64".into());
            }
        }
        Datum::F64 { bits }
            if bits.len() != 16
                || !bits
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
        {
            return Err("invalid f64 bits".into());
        }
        Datum::Compound { entries } => {
            for item in entries.values() {
                validate_datum(item)?;
            }
        }
        Datum::List { values } => {
            for item in values {
                validate_datum(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn diff_map(
    a: &BTreeMap<String, Datum>,
    b: &BTreeMap<String, Datum>,
    path: &str,
) -> Option<String> {
    let keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();
    for key in keys {
        let nested = format!("{path}.{key}");
        match (a.get(key), b.get(key)) {
            (Some(Datum::Compound { entries: x }), Some(Datum::Compound { entries: y })) => {
                if let Some(diff) = diff_map(x, y, &nested) {
                    return Some(diff);
                }
            }
            (Some(x), Some(y)) if x == y => {}
            _ => return Some(nested),
        }
    }
    None
}
