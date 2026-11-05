use mek_lab::lab::frame_tick;
use serde_json::json;
#[test]
fn client_counter_is_independent_of_world_time() {
    assert_eq!(
        frame_tick(&json!({"tick":"198","client_tick":"102"}), "client_applied").unwrap(),
        102
    );
}
#[test]
fn client_counter_cannot_be_defaulted_from_world_time() {
    assert!(
        frame_tick(&json!({"tick":"198"}), "client_applied")
            .unwrap_err()
            .contains("client")
    );
}
#[test]
fn malformed_client_counter_is_rejected() {
    assert!(
        frame_tick(
            &json!({"tick":"198","client_tick":"0102"}),
            "client_applied"
        )
        .unwrap_err()
        .contains("canonical")
    );
}
#[test]
fn observed_client_world_time_remains_required() {
    assert!(
        frame_tick(&json!({"client_tick":"102"}), "client_applied")
            .unwrap_err()
            .contains("tick")
    );
}
