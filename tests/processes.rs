use mek_lab::process::ManagedProcess;
use std::{path::Path, time::Duration};

#[test]
fn real_process_receives_exact_arguments_and_private_working_directory() {
    let scratch = tempfile::tempdir().unwrap();
    let output = scratch.path().join("observation.json");
    let args = vec![
        "fixture-probe".into(),
        output.to_str().unwrap().into(),
        "a space".into(),
        "$(not a shell);&".into(),
    ];
    let mut child = ManagedProcess::spawn(
        Path::new(env!("CARGO_BIN_EXE_mek-lab")),
        &args,
        scratch.path(),
        &[("MEKLAB_PROBE".into(), "isolated".into())],
        &scratch.path().join("probe"),
    )
    .unwrap();
    assert!(
        child
            .wait_timeout(Duration::from_secs(5))
            .unwrap()
            .success()
    );
    let observed: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(
        observed["args"],
        serde_json::json!(["a space", "$(not a shell);&"])
    );
    assert_eq!(observed["marker"], "isolated");
    assert_eq!(
        Path::new(observed["cwd"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        scratch.path().canonicalize().unwrap()
    );
}
#[test]
fn deadline_is_reported_instead_of_waiting_for_process_to_finish() {
    let scratch = tempfile::tempdir().unwrap();
    let mut child = ManagedProcess::spawn(
        Path::new(env!("CARGO_BIN_EXE_mek-lab")),
        &["fixture-probe".into(), "sleep".into()],
        scratch.path(),
        &[],
        &scratch.path().join("probe"),
    )
    .unwrap();
    let started = std::time::Instant::now();
    assert!(
        child
            .wait_timeout(Duration::from_millis(50))
            .unwrap_err()
            .contains("timeout")
    );
    drop(child);
    assert!(started.elapsed() < Duration::from_secs(3));
}
