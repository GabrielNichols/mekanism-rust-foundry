fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("fixture-probe") {
        if args.get(1).map(String::as_str) == Some("sleep") {
            std::thread::sleep(std::time::Duration::from_secs(60));
        } else {
            let output = args.get(1).expect("probe output");
            let result = serde_json::json!({"cwd":std::env::current_dir().unwrap(),"args":&args[2..],"marker":std::env::var("MEKLAB_PROBE").unwrap_or_default()});
            let mut result = result;
            result["homes"] = serde_json::to_value(
                [
                    "HORIZON_HOME",
                    "HORIZON_LAUNCHER_HOME",
                    "XDG_DATA_HOME",
                    "XDG_CONFIG_HOME",
                ]
                .into_iter()
                .map(|key| (key, std::env::var(key).ok()))
                .collect::<std::collections::BTreeMap<_, _>>(),
            )
            .unwrap();
            std::fs::write(output, serde_json::to_vec(&result).unwrap()).unwrap();
        }
        return;
    }
    let result = match args.first().map(String::as_str) {
        Some("compare-observer") if args.len() == 3 => (|| {
            let a = mek_lab::observer::load(std::path::Path::new(&args[1]))?;
            let b = mek_lab::observer::load(std::path::Path::new(&args[2]))?;
            let report = mek_lab::observer::check_pair(&a, &b)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
            );
            Ok(())
        })(),
        Some("select-windows-x64") if args.len() == 2 => (|| {
            let names: Vec<String> =
                serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let selected: Vec<_> = names
                .into_iter()
                .filter(|s| mek_lab::platform::windows_x64_native_matches(s))
                .collect();
            println!(
                "{}",
                serde_json::to_string(&selected).map_err(|e| e.to_string())?
            );
            Ok(())
        })(),
        Some("benchmark-summary") if args.len() == 2 => (|| {
            let study: mek_lab::benchmark::Study =
                serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let summary = mek_lab::benchmark::summarize(&study)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?
            );
            Ok(())
        })(),
        Some("verify-reference") if args.len() == 2 => (|| {
            let seal = mek_lab::trace_seal::verify_reference(std::path::Path::new(&args[1]))?;
            println!("{}", serde_json::to_string_pretty(&seal).map_err(|error| error.to_string())?);
            Ok(())
        })(),
        Some("run-catalog") if args.len() == 3 => mek_lab::lifecycle::run_catalog(
            std::path::Path::new(&args[1]), std::path::Path::new(&args[2]),
        ),
        Some("run-chunk-controls") if args.len() == 3 => mek_lab::lifecycle::run_chunk_controls(
            std::path::Path::new(&args[1]), std::path::Path::new(&args[2]),
        ),
        Some("run-live-observer-audit") if args.len() == 3 => {
            mek_lab::lifecycle::run_live_observer_audit(
                std::path::Path::new(&args[1]), std::path::Path::new(&args[2]),
            )
        }
        Some("simulate-observer") if args.len() == 2 => {
            mek_lab::observer_faults::run(std::path::Path::new(&args[1]))
        }
        Some("run-observer-controls") if (3..=6).contains(&args.len()) => (|| {
            let number = |index, default| {
                args.get(index).map_or(Ok(default), |value: &String| value.parse::<u16>())
                    .map_err(|_| "invalid observer control profile argument".to_string())
            };
            let repeats = u8::try_from(number(3, 1)?).map_err(|_| "invalid repeat count")?;
            mek_lab::lifecycle::run_observer_controls(
                std::path::Path::new(&args[1]), std::path::Path::new(&args[2]),
                repeats, number(4, 20)?, usize::from(number(5, 1)?),
            )
        })(),
        Some("run-lifecycle") if args.len() == 3 || args.len() == 4 => (|| {
            let repeats = args.get(3).map_or(Ok(1), |value| value.parse::<u8>())
                .map_err(|_| "invalid repeat count".to_string())?;
            mek_lab::lifecycle::run(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
                repeats,
            )
        })(),
        Some("run-reference") if args.len() == 4 => mek_lab::lab::run(
            std::path::Path::new(&args[1]),
            std::path::Path::new(&args[2]),
            std::path::Path::new(&args[3]),
        ),
        Some("compare") if args.len() == 3 => (|| {
            let a: mek_lab::Trace =
                serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let b: mek_lab::Trace =
                serde_json::from_slice(&std::fs::read(&args[2]).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let result = mek_lab::compare_traces(&a, &b)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
            );
            if !result.equal {
                return Err("semantic mismatch".into());
            }
            Ok(())
        })(),
        _ => {
            Err("usage: mek-lab run-reference ENV CASE NEW_OUTPUT | run-lifecycle ENV NEW_OUTPUT [REPEATS] | run-observer-controls ENV NEW_OUTPUT [REPEATS [TICK_RATE [WORKERS]]] | simulate-observer NEW_OUTPUT | run-live-observer-audit ENV NEW_OUTPUT | run-chunk-controls ENV NEW_OUTPUT | run-catalog ENV NEW_OUTPUT | verify-reference EVIDENCE_DIR | compare EXPECTED ACTUAL".into())
        }
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
