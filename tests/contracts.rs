use mek_lab::{Artifact, ArtifactLock, Datum, Frame, Trace, compare_traces, verify_artifacts};
use std::collections::BTreeMap;

fn trace(value: Datum) -> Trace {
    Trace {
        schema: 1,
        scenario: "machine-progress".into(),
        environment: "sealed-reference".into(),
        complete: true,
        frames: vec![Frame {
            sequence: 0,
            tick: 0,
            phase: "end_server_tick".into(),
            state: BTreeMap::from([("energy".into(), value)]),
        }],
    }
}
fn int(value: &str) -> Datum {
    Datum::I64 {
        value: value.into(),
    }
}

#[test]
fn identical_nonempty_traces_pass() {
    let t = trace(int("9007199254740993"));
    let result = compare_traces(&t, &t).unwrap();
    assert!(result.equal);
    assert_eq!(result.compared_frames, 1);
}
#[test]
fn client_and_server_world_clocks_are_not_collapsed() {
    let mut t = trace(int("1"));
    let base = t.frames[0].clone();
    t.frames = vec![
        Frame {
            sequence: 0,
            tick: 200,
            phase: "end_server_tick".into(),
            ..base.clone()
        },
        Frame {
            sequence: 1,
            tick: 198,
            phase: "client_applied".into(),
            ..base.clone()
        },
        Frame {
            sequence: 2,
            tick: 200,
            phase: "end_server_tick".into(),
            ..base
        },
    ];
    assert!(compare_traces(&t, &t).unwrap().equal);
}
#[test]
fn regression_within_client_clock_is_rejected() {
    let mut t = trace(int("1"));
    let base = t.frames[0].clone();
    t.frames = vec![
        Frame {
            sequence: 0,
            tick: 198,
            phase: "client_applied".into(),
            ..base.clone()
        },
        Frame {
            sequence: 1,
            tick: 200,
            phase: "end_server_tick".into(),
            ..base.clone()
        },
        Frame {
            sequence: 2,
            tick: 197,
            phase: "client_applied".into(),
            ..base
        },
    ];
    assert!(compare_traces(&t, &t).unwrap_err().contains("regression"));
}
#[test]
fn integer_above_float_precision_detects_one_unit_difference() {
    let result = compare_traces(
        &trace(int("9007199254740993")),
        &trace(int("9007199254740992")),
    )
    .unwrap();
    assert!(!result.equal);
    assert!(result.first_difference.unwrap().contains("energy"));
}
#[test]
fn signed_zero_is_preserved() {
    let result = compare_traces(
        &trace(Datum::F64 {
            bits: "8000000000000000".into(),
        }),
        &trace(Datum::F64 {
            bits: "0000000000000000".into(),
        }),
    )
    .unwrap();
    assert!(!result.equal);
}
#[test]
fn transient_difference_is_not_hidden_by_same_final_state() {
    let mut a = trace(int("1"));
    let mut b = trace(int("2"));
    let final_frame = Frame {
        sequence: 1,
        tick: 1,
        phase: "end_server_tick".into(),
        state: BTreeMap::from([("energy".into(), int("0"))]),
    };
    a.frames.push(final_frame.clone());
    b.frames.push(final_frame);
    assert!(!compare_traces(&a, &b).unwrap().equal);
}
#[test]
fn empty_trace_cannot_pass() {
    let mut t = trace(int("1"));
    t.frames.clear();
    assert!(compare_traces(&t, &t).unwrap_err().contains("empty"));
}
#[test]
fn incomplete_trace_cannot_pass() {
    let mut t = trace(int("1"));
    t.complete = false;
    assert!(compare_traces(&t, &t).unwrap_err().contains("incomplete"));
}
#[test]
fn sequence_gap_is_not_semantic_mismatch() {
    let a = trace(int("1"));
    let mut b = trace(int("1"));
    b.frames[0].sequence = 2;
    assert!(compare_traces(&a, &b).unwrap_err().contains("sequence"));
}
#[test]
fn environment_drift_blocks_comparison() {
    let a = trace(int("1"));
    let mut b = trace(int("1"));
    b.environment = "other-config".into();
    assert!(compare_traces(&a, &b).unwrap_err().contains("environment"));
}
#[test]
fn invalid_integer_is_not_an_acceptable_golden() {
    let t = trace(int("9223372036854775808"));
    assert!(compare_traces(&t, &t).unwrap_err().contains("i64"));
}
#[test]
fn absent_and_empty_are_different() {
    let r = compare_traces(
        &trace(Datum::Absent),
        &trace(Datum::List { values: vec![] }),
    )
    .unwrap();
    assert!(!r.equal);
}
#[test]
fn unknown_trace_field_is_rejected() {
    let t = serde_json::json!({"schema":1,"scenario":"s","environment":"e","complete":true,"frames":[],"ignored":true});
    assert!(serde_json::from_value::<Trace>(t).is_err());
}
#[test]
fn sha256_known_vector_and_artifact_change_are_detected() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("artifact with spaces.bin");
    std::fs::write(&path, b"abc").unwrap();
    let lock = ArtifactLock {
        schema: 1,
        artifacts: vec![Artifact {
            role: "bridge".into(),
            path: path.clone(),
            sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into(),
        }],
    };
    let fingerprint = verify_artifacts(&lock).unwrap();
    assert_eq!(fingerprint.len(), 64);
    std::fs::write(path, b"abd").unwrap();
    assert!(verify_artifacts(&lock).unwrap_err().contains("hash"));
}
#[test]
fn empty_or_duplicate_artifact_roles_are_rejected() {
    assert!(
        verify_artifacts(&ArtifactLock {
            schema: 1,
            artifacts: vec![]
        })
        .unwrap_err()
        .contains("empty")
    );
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("a");
    std::fs::write(&path, b"abc").unwrap();
    let a = Artifact {
        role: "bridge".into(),
        path,
        sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into(),
    };
    assert!(
        verify_artifacts(&ArtifactLock {
            schema: 1,
            artifacts: vec![a.clone(), a]
        })
        .unwrap_err()
        .contains("duplicate")
    );
}

#[test]
fn shadow_ci_intentional_failure() {
    assert_eq!(
        "observed", "expected",
        "shadow CI must block a Rust test failure"
    );
}
