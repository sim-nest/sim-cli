use std::{fs, process::Command};

#[test]
fn bare_bootloader_reports_effective_config_without_codec() {
    let root = temp_root("effective");
    let config_file = root.join("sim.toml");
    fs::create_dir_all(&root).unwrap();
    fs::write(
        &config_file,
        r#"[sim/cookbook]
minimum_loaded = ["codec/lisp"]
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_sim"))
        .arg("--config-file")
        .arg(&config_file)
        .arg("config")
        .arg("effective")
        .arg("sim/cookbook")
        .arg("--json")
        .output()
        .expect("run sim config effective");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "{\"lib\":\"sim/cookbook\",\"table\":{\"minimum_loaded\":[\"codec/lisp\"]}}\n"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn bare_bootloader_reports_missing_config_source() {
    let root = temp_root("missing");
    let missing = root.join("missing.toml");

    let output = Command::new(env!("CARGO_BIN_EXE_sim"))
        .arg("--config-work")
        .arg(root.join("work"))
        .arg("--config-file")
        .arg(&missing)
        .arg("config")
        .arg("sources")
        .arg("--json")
        .output()
        .expect("run sim config sources");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"status\":\"missing\""), "{stdout}");
    assert!(
        stdout.contains(&format!("config file not found: {}", missing.display())),
        "{stdout}"
    );

    let _ = fs::remove_dir_all(root);
}

fn temp_root(label: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sim-run-cli-config-report-{}-{label}-{nanos}",
        std::process::id()
    ))
}
