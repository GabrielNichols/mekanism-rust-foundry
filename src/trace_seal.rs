use crate::{LabResult, Trace};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seal {
    pub frames: usize,
    pub bytes: usize,
    pub sha256: String,
}

fn describe(bytes: &[u8]) -> LabResult<(Trace, Seal)> {
    let trace: Trace = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    crate::validate_trace(&trace)?;
    let seal = Seal {
        frames: trace.frames.len(),
        bytes: bytes.len(),
        sha256: format!("{:x}", Sha256::digest(bytes)),
    };
    Ok((trace, seal))
}

fn verify(bytes: &[u8], expected: &Seal) -> LabResult<Trace> {
    let (trace, actual) = describe(bytes)?;
    if actual.frames != expected.frames {
        return Err("sealed trace frame count mismatch".into());
    }
    if actual.bytes != expected.bytes {
        return Err("sealed trace byte count mismatch".into());
    }
    if actual.sha256 != expected.sha256 {
        return Err("sealed trace hash mismatch".into());
    }
    Ok(trace)
}

pub(crate) fn for_file(path: &Path) -> LabResult<Seal> {
    describe(&fs::read(path).map_err(|error| error.to_string())?).map(|(_, seal)| seal)
}

pub(crate) fn verify_archived(
    directory: &Path,
    run: &Value,
    result: &Value,
) -> LabResult<(Trace, Seal)> {
    if run["schema"] != 2 || result["schema"] != 2 || result["status"] != "functional_pass" {
        return Err("sealed evidence requires a completed schema 2 reference run".into());
    }
    for field in [
        "attempt_id",
        "case",
        "artifact_fingerprint",
        "execution_fingerprint",
        "case_sha256",
    ] {
        let declared = run[field]
            .as_str()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("missing sealed reference {field}"))?;
        if result[field] != declared {
            return Err(format!("sealed run and result differ at {field}"));
        }
    }
    if crate::sha256_file(&directory.join("case.json"))? != run["case_sha256"] {
        return Err("sealed case hash mismatch".into());
    }
    let seal: Seal = serde_json::from_value(result["trace_seal"].clone())
        .map_err(|error| format!("missing or invalid trace seal: {error}"))?;
    let bytes = fs::read(directory.join("trace.json")).map_err(|error| error.to_string())?;
    let trace = verify(&bytes, &seal)?;
    if trace.scenario != run["case"] {
        return Err("sealed trace scenario differs from the declared case".into());
    }
    Ok((trace, seal))
}

pub fn verify_reference(directory: &Path) -> LabResult<Seal> {
    let read = |filename| -> LabResult<Value> {
        serde_json::from_slice(
            &fs::read(directory.join(filename)).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())
    };
    let run = read("run.json")?;
    let result = read("result.json")?;
    verify_archived(directory, &run, &result).map(|(_, seal)| seal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trace() -> serde_json::Value {
        serde_json::json!({
            "schema":1,"scenario":"sealed-case","environment":"reference-lock","complete":true,
            "frames":[
                {"sequence":0,"tick":1,"phase":"after_action","state":{"ore":{"type":"i64","value":"1"}}},
                {"sequence":1,"tick":2,"phase":"after_action","state":{"ore":{"type":"i64","value":"2"}}}
            ]
        })
    }

    #[test]
    fn valid_prefix_with_complete_flag_is_rejected_by_the_seal() {
        let full = trace();
        let (_, seal) = describe(&serde_json::to_vec(&full).unwrap()).unwrap();
        let mut shortened = full;
        shortened["frames"].as_array_mut().unwrap().pop();
        assert!(
            verify(&serde_json::to_vec(&shortened).unwrap(), &seal)
                .unwrap_err()
                .contains("frame count")
        );
    }

    #[test]
    fn same_length_edit_is_rejected_even_when_the_trace_remains_valid() {
        let mut full = trace();
        let bytes = serde_json::to_vec(&full).unwrap();
        let (_, seal) = describe(&bytes).unwrap();
        assert!(verify(&bytes, &seal).is_ok());
        full["frames"][1]["state"]["ore"]["value"] = serde_json::json!("3");
        let edited = serde_json::to_vec(&full).unwrap();
        assert_eq!(bytes.len(), edited.len());
        assert!(verify(&edited, &seal).unwrap_err().contains("hash"));
    }
}
