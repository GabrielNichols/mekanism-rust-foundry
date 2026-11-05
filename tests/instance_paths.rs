use mek_lab::lab::instance_root;
#[test]
fn relative_destination_resolves_absolute_instance_root() {
    let cwd = std::env::current_dir().unwrap();
    let dir = tempfile::tempdir_in(&cwd).unwrap();
    let relative = dir.path().strip_prefix(&cwd).unwrap();
    assert!(instance_root(relative).unwrap().is_absolute());
}
#[test]
fn relative_and_absolute_destinations_select_the_same_root() {
    let cwd = std::env::current_dir().unwrap();
    let dir = tempfile::tempdir_in(&cwd).unwrap();
    let relative = dir.path().strip_prefix(&cwd).unwrap();
    assert_eq!(
        instance_root(relative).unwrap(),
        instance_root(dir.path()).unwrap()
    );
}
