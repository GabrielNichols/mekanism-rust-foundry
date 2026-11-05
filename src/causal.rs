use crate::{
    LabResult, Trace,
    lab::{self, Session},
};
use serde_json::{Map, Value, json};
use std::{
    path::Path,
    time::{Duration, Instant},
};

fn number(state: &Value, field: &str) -> LabResult<u64> {
    let raw = state[field]
        .as_str()
        .ok_or_else(|| format!("missing phase counter {field}"))?;
    let value: u64 = raw
        .parse()
        .map_err(|_| format!("invalid phase counter {field}"))?;
    if value.to_string() != raw {
        return Err(format!("noncanonical phase counter {field}"));
    }
    Ok(value)
}

pub(crate) fn validate(phases: &Value) -> LabResult<()> {
    let [before, sent, server, applied, rendered] =
        ["before", "sent", "server", "applied", "rendered"].map(|key| &phases[key]);
    if [before, sent, server, applied, rendered]
        .iter()
        .any(|state| state["owner_thread"] != true)
    {
        return Err("phase receipt lacks its owner thread".into());
    }
    let menu = before["menu_id"]
        .as_u64()
        .filter(|id| *id > 0)
        .ok_or("missing menu identity")?;
    if [sent, server, applied, rendered]
        .iter()
        .any(|state| state["menu_id"] != menu)
    {
        return Err("phase menu identity mismatch".into());
    }
    let revision = server["menu_state_id"]
        .as_u64()
        .filter(|id| *id <= 32767)
        .ok_or("invalid authoritative menu revision")?;
    if before["menu_state_id"].as_u64().is_none()
        || sent["menu_state_id"].as_u64().is_none()
        || before["menu_state_id"] == revision
        || sent["menu_state_id"] == revision
        || applied["menu_state_id"] != revision
        || rendered["menu_state_id"] != revision
    {
        return Err("authoritative menu revision was not freshly applied and rendered".into());
    }
    if number(before, "tick")? != number(server, "tick")? {
        return Err("frozen server clock changed between action boundaries".into());
    }
    if number(applied, "client_tick")? <= number(sent, "client_tick")?
        || number(rendered, "client_tick")? < number(applied, "client_tick")?
        || number(rendered, "frame")? <= number(applied, "frame")?
    {
        return Err("client apply and render phases did not advance".into());
    }
    if phases["early_capture"] != json!({"gate":"blocked","captured":false}) {
        return Err("premature capture was not rejected".into());
    }
    let empty = json!({"item":"minecraft:air","count":0});
    let ore = json!({"item":"minecraft:iron_ore","count":3});
    if before["cursor"] != empty
        || [server, applied, rendered]
            .iter()
            .any(|state| state["cursor"] != ore)
    {
        return Err("phase cursor does not prove the authoritative transfer".into());
    }
    Ok(())
}

fn client_witness(state: &Value, owner: &Value) -> Value {
    let mut state = state.clone();
    state["owner_thread"] = owner.clone();
    state
}

fn server_witness(state: &Value) -> LabResult<Value> {
    let players = state["players"]
        .as_array()
        .filter(|players| players.len() == 1)
        .ok_or("causal fixture requires exactly one server player")?;
    let mut witness = players[0].clone();
    witness["tick"] = state["tick"].clone();
    witness["owner_thread"] = state["owner_thread"].clone();
    Ok(witness)
}

struct Audit<'a> {
    output: &'a Path,
    receipts: Vec<Value>,
}

impl Audit<'_> {
    fn request(
        &mut self,
        session: &mut Session,
        role: &str,
        action: &str,
        args: &Map<String, Value>,
    ) -> LabResult<Value> {
        let start = Instant::now();
        let reply = session.request(action, args)?;
        let round_trip_ns = u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX);
        self.receipts
            .push(json!({"role":role,"round_trip_ns":round_trip_ns,"receipt":reply}));
        lab::atomic_json(&self.output.join("causal-receipts.json"), &self.receipts)?;
        Ok(reply)
    }
}

pub(crate) fn run(
    server: &mut Session,
    client: &mut Session,
    output: &Path,
    trace: &mut Trace,
) -> LabResult<Value> {
    let mut audit = Audit {
        output,
        receipts: Vec::new(),
    };
    let empty = Map::new();
    let ore = json!({"item":"minecraft:iron_ore","count":3});
    let no_items = json!({"item":"minecraft:air","count":0});
    let before = audit.request(server, "server", "read", &empty)?;
    let before = server_witness(&before["state"])?;
    if before["source"] != ore || before["cursor"] != no_items || before["menu_open"] != true {
        return Err("source pickup requires the initial authoritative menu state".into());
    }
    let sent = audit.request(
        client,
        "client",
        "click_slot",
        json!({"selector":"source"}).as_object().unwrap(),
    )?;
    let sent = client_witness(&sent["state"], &sent["owner_thread"]);
    let capture_path = output.join("instances/client-p1/meklab-causal.png");
    let early = audit.request(client, "client", "barrier_capture", &empty)?;
    if capture_path
        .try_exists()
        .map_err(|error| error.to_string())?
    {
        return Err("premature capture wrote an image".into());
    }

    let deadline = Instant::now() + Duration::from_secs(15);
    let mut server_polls = 0;
    let applied_on_server = loop {
        let reply = audit.request(server, "server", "read", &empty)?;
        let state = server_witness(&reply["state"])?;
        server_polls += 1;
        if state["cursor"] == ore && state["source"] == no_items {
            break state;
        }
        if Instant::now() >= deadline {
            return Err("server did not apply the single Screen pickup".into());
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    let sync_reply = audit.request(server, "server", "barrier_sync", &empty)?;
    let sync = server_witness(&sync_reply["state"])?;
    if sync["menu_id"] != applied_on_server["menu_id"]
        || sync["cursor"] != ore
        || sync["source"] != no_items
        || sync["tick"] != applied_on_server["tick"]
        || sync["menu_state_id"] == applied_on_server["menu_state_id"]
    {
        return Err("server did not send a fresh authoritative menu revision".into());
    }
    let args = json!({"menu_id":sync["menu_id"],"menu_state_id":sync["menu_state_id"]});
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut client_polls = 0;
    loop {
        let reply = audit.request(client, "client", "read", &empty)?;
        let state = &reply["state"];
        client_polls += 1;
        if state["menu_id"] == sync["menu_id"]
            && state["menu_state_id"] == sync["menu_state_id"]
            && state["cursor"] == ore
            && state["source"] == no_items
            && state["container_screen"] == true
        {
            break;
        }
        if Instant::now() >= deadline {
            return Err("client did not apply the authoritative menu revision".into());
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    let applied = audit.request(client, "client", "barrier_arm", args.as_object().unwrap())?;
    let applied = client_witness(&applied["state"], &applied["owner_thread"]);
    let rendered = audit.request(client, "client", "barrier_capture", &empty)?;
    if rendered["capture"] != json!({"gate":"rendered","captured":true}) {
        return Err("armed capture did not complete at render post".into());
    }
    let rendered = client_witness(&rendered["state"], &rendered["owner_thread"]);
    let png = std::fs::read(&capture_path).map_err(|error| format!("rendered image: {error}"))?;
    if !png.starts_with(b"\x89PNG\r\n\x1a\n") || png.len() <= 24 {
        return Err("rendered image has no valid PNG header".into());
    }
    let screenshot_sha256 = crate::sha256_file(&capture_path)?;
    let evidence = json!({"before":before,"sent":sent,"early_capture":early["capture"],
        "server":sync,"applied":applied,"rendered":rendered,"server_polls":server_polls,"client_polls":client_polls,
        "screenshot_sha256":screenshot_sha256,"screenshot_bytes":png.len()});
    lab::atomic_json(&output.join("causal-barrier.json"), &evidence)?;
    validate(&evidence)?;
    for (key, phase) in [
        ("before", "before_action"),
        ("sent", "client_sent"),
        ("server", "server_applied"),
        ("applied", "client_applied"),
        ("rendered", "client_rendered"),
    ] {
        lab::push_frame(trace, &evidence[key], phase)?;
    }
    Ok(sync_reply["state"].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn evidence() -> Value {
        json!({
            "before":{"tick":"11","menu_id":1,"menu_state_id":4,"cursor":{"item":"minecraft:air","count":0},"owner_thread":true},
            "sent":{"client_tick":"140","menu_id":1,"menu_state_id":4,"owner_thread":true},
            "early_capture":{"gate":"blocked","captured":false},
            "server":{"tick":"11","menu_id":1,"menu_state_id":5,"cursor":{"item":"minecraft:iron_ore","count":3},"owner_thread":true},
            "applied":{"client_tick":"142","frame":"30","menu_id":1,"menu_state_id":5,"cursor":{"item":"minecraft:iron_ore","count":3},"owner_thread":true},
            "rendered":{"client_tick":"143","frame":"31","menu_id":1,"menu_state_id":5,"cursor":{"item":"minecraft:iron_ore","count":3},"owner_thread":true}
        })
    }

    #[test]
    fn local_prediction_does_not_prove_the_authoritative_menu_revision() {
        let mut receipt = evidence();
        assert!(validate(&receipt).is_ok());
        receipt["applied"]["menu_state_id"] = json!(4);
        assert!(validate(&receipt).unwrap_err().contains("revision"));
    }

    #[test]
    fn fails_on_reused_menu_wrong_owner_or_nonadvancing_render() {
        let mut receipt = evidence();
        receipt["applied"]["menu_id"] = json!(2);
        assert!(validate(&receipt).unwrap_err().contains("identity"));
        receipt = evidence();
        receipt["server"]["owner_thread"] = json!(false);
        assert!(validate(&receipt).unwrap_err().contains("owner"));
        receipt = evidence();
        receipt["rendered"]["frame"] = json!("30");
        assert!(validate(&receipt).unwrap_err().contains("render"));
    }

    #[test]
    fn cannot_promote_a_capture_before_the_barrier() {
        let mut receipt = evidence();
        receipt["early_capture"]["captured"] = json!(true);
        assert!(validate(&receipt).unwrap_err().contains("premature"));
    }
}
