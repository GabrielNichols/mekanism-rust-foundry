use super::{
    Environment, Observer, Server, bounded_map, elapsed_ns, lab, launch, validate_advance,
};
use crate::LabResult;
use serde_json::{Value, json};
use std::{fs, path::Path, time::Instant};

const ENRICHMENT_CHAMBER: &str = "mekanism:enrichment_chamber";
const IRON_FROM_ORE: &str = "mekanism:processing/iron/dust/from_ore";
const ENRICHING: &str = "mekanism:enriching";

fn validate_enrichment_binding(catalog: &Value) -> LabResult<()> {
    let blocks = &catalog["registries"]["blocks"];
    if !blocks
        .as_array()
        .is_some_and(|blocks| blocks.iter().any(|block| block == ENRICHMENT_CHAMBER))
    {
        return Err(format!(
            "effective catalog drift: missing block {ENRICHMENT_CHAMBER}"
        ));
    }
    let recipe = catalog["recipes"]
        .get(IRON_FROM_ORE)
        .ok_or_else(|| format!("effective catalog drift: missing recipe {IRON_FROM_ORE}"))?;
    for field in ["type", "serializer"] {
        let actual = &recipe[field];
        if actual.as_str() != Some(ENRICHING) {
            return Err(format!(
                "effective catalog drift: recipe {IRON_FROM_ORE} {field} expected {ENRICHING}, found {actual}"
            ));
        }
    }
    Ok(())
}

fn compare(baseline: &Value, candidate: &Value) -> LabResult<()> {
    let baseline_fields = baseline.as_object().ok_or("missing baseline catalog")?;
    let candidate_fields = candidate.as_object().ok_or("missing candidate catalog")?;
    validate_enrichment_binding(baseline)?;
    validate_enrichment_binding(candidate)?;
    for field in baseline_fields.keys().chain(candidate_fields.keys()) {
        if baseline_fields.get(field) != candidate_fields.get(field) {
            return Err(format!("effective catalog differs at {field}"));
        }
    }
    Ok(())
}

fn capture(server: &mut Server, path: &Path) -> LabResult<(Value, u64)> {
    let tick = server.clock()?;
    let start = Instant::now();
    let receipt = server
        .bridge
        .as_mut()
        .ok_or("catalog requires a bridge")?
        .request("effective_catalog", &Default::default())?;
    let request_ns = elapsed_ns(start);
    lab::atomic_json(path, &receipt)?;
    let mut catalog = receipt["catalog"].clone();
    let fields = catalog.as_object_mut().ok_or("missing catalog payload")?;
    if fields.remove("owner_thread") != Some(json!(true))
        || fields.remove("tick") != Some(json!(tick.to_string()))
    {
        return Err("catalog owner or clock does not match the frozen server".into());
    }
    validate_advance(tick, 0, server.clock()?)?;
    let modules = catalog["modules"]
        .as_object()
        .ok_or("missing module list")?;
    for required in ["minecraft", "neoforge", "mekanism", "meklab_bridge"] {
        if !modules.contains_key(required) {
            return Err(format!("missing required module {required}"));
        }
    }
    if modules
        .keys()
        .any(|name| name.starts_with("mekanism") && name != "mekanism")
    {
        return Err("Core bootstrap loaded an unexpected Mekanism module".into());
    }
    for name in [
        "blocks",
        "items",
        "block_entities",
        "menus",
        "fluids",
        "recipe_types",
        "recipe_serializers",
        "chemicals",
    ] {
        if catalog["registries"][name]
            .as_array()
            .is_none_or(|values| values.is_empty())
        {
            return Err(format!("missing or empty registry {name}"));
        }
    }
    if catalog["recipes"]
        .as_object()
        .is_none_or(|recipes| recipes.is_empty())
        || catalog["packs"]
            .as_array()
            .is_none_or(|packs| packs.is_empty())
    {
        return Err("empty recipe or selected pack catalog".into());
    }
    Ok((catalog, request_ns))
}

fn trial(environment: &Environment, output: &Path, rate: u16) -> LabResult<(Value, Value)> {
    let start = Instant::now();
    let dir = output.join("instance");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let mut server = launch(
        environment,
        &dir,
        output.join("server"),
        Observer::Catalog,
        rate,
    )?;
    let configs = lab::config_hashes(&dir)?;
    if !configs
        .keys()
        .any(|name| name.to_ascii_lowercase().contains("mekanism"))
    {
        return Err("missing generated Mekanism configuration".into());
    }
    let (mut before, initial_catalog_ns) =
        capture(&mut server, &output.join("catalog-before.json"))?;
    let ticks = server.steps(3)?;
    let (after, final_catalog_ns) = capture(&mut server, &output.join("catalog-after.json"))?;
    compare(&before, &after)?;
    let flush_ns = server.save()?;
    let process = server.exit(false)?;
    if lab::config_hashes(&dir)? != configs {
        return Err("generated configuration changed during catalog bootstrap".into());
    }
    before["effective_config"] = json!(configs);
    lab::atomic_json(&output.join("effective-catalog.json"), &before)?;
    let timing = json!({"tick_rate":rate,"ticks":ticks,"observer_before_ns":initial_catalog_ns,
        "observer_after_ns":final_catalog_ns,"flush_ns":flush_ns,"process":process,"total_ns":elapsed_ns(start)});
    lab::atomic_json(&output.join("timing.json"), &timing)?;
    Ok((before, timing))
}

pub(super) fn campaign(environment: &Environment, output: &Path) -> LabResult<()> {
    let timer = Instant::now();
    let (baseline, serial) = trial(environment, &output.join("serial-20"), 20)?;
    let jobs = [1, 2, 3];
    let accelerated = bounded_map(&jobs, 3, |worker| {
        trial(environment, &output.join(format!("worker-{worker}")), 100)
    });
    let mut timings = vec![serial];
    for result in accelerated {
        let (candidate, timing) = result?;
        compare(&baseline, &candidate)?;
        timings.push(timing);
    }
    lab::atomic_json(
        &output.join("catalog-campaign.json"),
        &json!({
            "scope":"core_effective_catalog_bootstrap","cross_profile_catalog_exact":true,
            "timings":timings,"campaign_wall_ns":elapsed_ns(timer),"qualified":false,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture() -> Value {
        json!({"modules":{"mekanism":"10.7.19"},
            "registries":{"blocks":["mekanism:enrichment_chamber","mekanism:metallurgic_infuser"]},
            "recipes":{(IRON_FROM_ORE):{"type":"mekanism:enriching","serializer":"mekanism:enriching"},
                "mekanism:enriching/example":{"type":"mekanism:enriching","serializer":"mekanism:enriching"}},
            "packs":["vanilla"],"effective_config":{"mekanism-common.toml":"pinned"}})
    }

    #[test]
    fn rejects_identical_catalogs_missing_enrichment_chamber() {
        let mut catalog = fixture();
        catalog["registries"]["blocks"] = json!(["mekanism:metallurgic_infuser"]);
        assert_eq!(
            compare(&catalog, &catalog),
            Err("effective catalog drift: missing block mekanism:enrichment_chamber".into())
        );
    }

    #[test]
    fn rejects_identical_catalogs_missing_iron_from_ore() {
        let mut catalog = fixture();
        catalog["recipes"]
            .as_object_mut()
            .unwrap()
            .remove(IRON_FROM_ORE);
        assert_eq!(
            compare(&catalog, &catalog),
            Err(format!(
                "effective catalog drift: missing recipe {IRON_FROM_ORE}"
            ))
        );
    }

    #[test]
    fn rejects_rebound_iron_from_ore_type_or_serializer() {
        let baseline = fixture();
        for field in ["type", "serializer"] {
            let mut candidate = fixture();
            candidate["recipes"][IRON_FROM_ORE][field] = json!("mekanism:crushing");
            assert_eq!(
                compare(&baseline, &candidate),
                Err(format!(
                    "effective catalog drift: recipe {IRON_FROM_ORE} {field} expected mekanism:enriching, found \"mekanism:crushing\""
                ))
            );
        }
    }

    #[test]
    fn preserves_runtime_catalog_semantics() {
        let baseline = fixture();
        assert!(compare(&baseline, &baseline).is_ok());
        let mut candidate = fixture();
        candidate["registries"]["blocks"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(
            compare(&baseline, &candidate)
                .unwrap_err()
                .contains("registries")
        );
        let mut candidate = fixture();
        candidate["recipes"]["mekanism:enriching/example"]["type"] = json!("mekanism:crushing");
        assert!(
            compare(&baseline, &candidate)
                .unwrap_err()
                .contains("recipes")
        );
        let mut candidate = fixture();
        candidate["modules"]["mekanism"] = json!("10.7.20");
        assert!(
            compare(&baseline, &candidate)
                .unwrap_err()
                .contains("modules")
        );
        let mut candidate = fixture();
        candidate["effective_config"]["mekanism-common.toml"] = json!("drift");
        assert!(
            compare(&baseline, &candidate)
                .unwrap_err()
                .contains("effective_config")
        );
    }
}
