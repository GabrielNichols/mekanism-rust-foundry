use std::{path::Path, time::Duration};
#[test]
fn actual_processes_receive_disjoint_owned_data_directories() {
    let root = tempfile::tempdir().unwrap();
    let mut observed = Vec::new();
    for name in ["server", "client"] {
        let instance = root.path().join(name);
        std::fs::create_dir(&instance).unwrap();
        let output = instance.join("probe.json");
        let env = mek_lab::lab::private_homes(&instance).unwrap();
        let mut child = mek_lab::process::ManagedProcess::spawn(
            Path::new(env!("CARGO_BIN_EXE_mek-lab")),
            &[
                "fixture-probe".into(),
                output.to_string_lossy().into_owned(),
            ],
            &instance,
            &env,
            &instance.join("probe"),
        )
        .unwrap();
        assert!(
            child
                .wait_timeout(Duration::from_secs(5))
                .unwrap()
                .success()
        );
        let data: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
        let homes = data["homes"].as_object().unwrap();
        assert_eq!(homes.len(), 4);
        let mut paths = Vec::new();
        for value in homes.values() {
            let path = Path::new(value.as_str().expect("private home required"));
            assert!(path.starts_with(&instance) && path.is_dir());
            paths.push(path.to_path_buf());
        }
        observed.push(paths);
    }
    for path in &observed[0] {
        assert!(!observed[1].contains(path));
    }
}
