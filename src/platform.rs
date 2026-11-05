pub fn windows_x64_native_matches(coordinate: &str) -> bool {
    match coordinate.split(':').nth(3) {
        Some(classifier) if classifier.starts_with("natives-") => classifier == "natives-windows",
        _ => true,
    }
}
