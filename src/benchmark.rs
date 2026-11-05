use crate::LabResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub reference_ns: u64,
    pub candidate_ns: u64,
    pub reference_units: u64,
    pub candidate_units: u64,
    pub reference_output_sha256: String,
    pub candidate_output_sha256: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Study {
    pub workload_sha256: String,
    pub phase: String,
    pub warmup_units: u64,
    pub trials: Vec<Trial>,
}
#[derive(Debug, Serialize)]
pub struct Summary {
    pub paired_trials: usize,
    pub reference_lower_median_ns: u64,
    pub candidate_lower_median_ns: u64,
    pub descriptive_ratio: f64,
    pub optimization_proven: bool,
}
pub fn summarize(study: &Study) -> LabResult<Summary> {
    let valid_hash = |s: &str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit());
    if !valid_hash(&study.workload_sha256) {
        return Err("invalid workload hash".into());
    }
    if study.trials.len() < 5 {
        return Err("at least five paired trials required".into());
    }
    match study.phase.as_str() {
        "steady_state" if study.warmup_units == 0 => {
            return Err("steady state requires warmup".into());
        }
        "steady_state" | "process_cold_start" => {}
        _ => return Err("unknown measurement phase".into()),
    }
    let units = study.trials[0].reference_units;
    let mut reference = Vec::new();
    let mut candidate = Vec::new();
    for trial in &study.trials {
        if trial.reference_units == 0
            || trial.reference_units != trial.candidate_units
            || trial.reference_units != units
        {
            return Err("work units differ or are empty".into());
        }
        if !valid_hash(&trial.reference_output_sha256)
            || trial.reference_output_sha256 != trial.candidate_output_sha256
        {
            return Err("output digests differ or are invalid".into());
        }
        if trial.reference_ns == 0 || trial.candidate_ns == 0 {
            return Err("zero measurement duration".into());
        }
        reference.push(trial.reference_ns);
        candidate.push(trial.candidate_ns);
    }
    reference.sort_unstable();
    candidate.sort_unstable();
    let position = (reference.len() - 1) / 2;
    Ok(Summary {
        paired_trials: reference.len(),
        reference_lower_median_ns: reference[position],
        candidate_lower_median_ns: candidate[position],
        descriptive_ratio: reference[position] as f64 / candidate[position] as f64,
        optimization_proven: false,
    })
}
