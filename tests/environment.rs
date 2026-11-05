fn env(seconds: u64) -> mek_lab::lab::Environment {
    serde_json::from_value(serde_json::json!({"schema":1,"lock":{"schema":1,"artifacts":[]},"server_root":"private-server","client_root":"private-client","boot_timeout_seconds":seconds})).unwrap()
}
#[test]
fn slow_boot_has_explicit_bounded_budget() {
    assert_eq!(
        env(900).boot_timeout().unwrap(),
        std::time::Duration::from_secs(900)
    );
}
#[test]
fn budget_is_not_silently_clamped() {
    assert_eq!(
        env(120).boot_timeout().unwrap(),
        std::time::Duration::from_secs(120)
    );
}
#[test]
fn unbounded_boot_is_not_accepted() {
    assert!(env(0).boot_timeout().is_err());
    assert!(env(901).boot_timeout().is_err());
}
