fn main() {
    let result = (|| -> mek_lab::LabResult<()> {
        let path = std::env::args_os()
            .nth(1)
            .ok_or("usage: mek-work RUN_DIRECTORY")?;
        let run = mek_lab::observer::load(std::path::Path::new(&path))?;
        let observations = run["observations"]
            .as_array()
            .ok_or("missing observations")?;
        let mode = run["case"]["observer_mode"]
            .as_str()
            .ok_or("missing observer mode")?;
        let mut report = mek_lab::work::analyze(observations, mode)?;
        report["analysis_binary_sha256"] = serde_json::json!(mek_lab::sha256_file(
            &std::env::current_exe().map_err(|e| e.to_string())?
        )?);
        report["reference_runner_sha256"] = run["runner_sha256"].clone();
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
