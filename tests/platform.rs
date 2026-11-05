use mek_lab::platform::windows_x64_native_matches as matches;
#[test]
fn selects_x64_native_without_arch_suffix() {
    assert!(matches("org.lwjgl:lwjgl-glfw:3.3.3:natives-windows"));
}
#[test]
fn excludes_x86_and_arm64_natives() {
    assert!(!matches("org.lwjgl:lwjgl-glfw:3.3.3:natives-windows-x86"));
    assert!(!matches("org.lwjgl:lwjgl-glfw:3.3.3:natives-windows-arm64"));
}
#[test]
fn excludes_other_operating_systems() {
    assert!(!matches("org.lwjgl:lwjgl-glfw:3.3.3:natives-linux"));
    assert!(!matches("org.lwjgl:lwjgl-glfw:3.3.3:natives-macos"));
}
#[test]
fn ordinary_java_artifacts_are_not_architecture_filtered() {
    assert!(matches("com.mojang:brigadier:1.3.10"));
}
