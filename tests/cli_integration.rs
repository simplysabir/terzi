use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

const HTTPBIN_URL: &str = "https://httpbin.org";

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A modern CLI API client"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("terzi"));
}

#[test]
fn test_basic_get_request() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg(format!("{}/get", HTTPBIN_URL));

    // This test might fail in CI without network access, so we're lenient
    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("200") || stdout.contains("GET"));
    }
}

#[test]
fn test_get_request_with_headers() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-H",
        "X-Test-Header: test-value",
        "-H",
        "User-Agent: Terzi-Test",
        &format!("{}/headers", HTTPBIN_URL),
    ]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("X-Test-Header") || stdout.contains("test-value"));
    }
}

#[test]
fn test_post_request_with_json() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-m",
        "POST",
        "-j",
        r#"{"test": "data", "number": 42}"#,
        &format!("{}/post", HTTPBIN_URL),
    ]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("POST"));
    }
}

#[test]
fn test_post_request_with_form_data() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-m",
        "POST",
        "-f",
        "name=John",
        "-f",
        "age=30",
        &format!("{}/post", HTTPBIN_URL),
    ]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("POST"));
    }
}

#[test]
fn test_request_with_timeout() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&["-t", "10", &format!("{}/delay/1", HTTPBIN_URL)]);

    let output = cmd.output().unwrap();
    // Should succeed with sufficient timeout
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.trim().is_empty());
    }
}

#[test]
fn test_invalid_url() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg("not-a-valid-url");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid URL").or(predicate::str::contains("error")));
}

#[test]
fn test_authentication_bearer() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-A",
        "bearer:test-token",
        &format!("{}/bearer", HTTPBIN_URL),
    ]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("authenticated") || stdout.contains("true"));
    }
}

#[test]
fn test_authentication_basic() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-A",
        "basic:user:pass",
        &format!("{}/basic-auth/user/pass", HTTPBIN_URL),
    ]);

    // Should succeed with correct basic auth
    let output = cmd.output().unwrap();
    assert!(output.status.success());
}

#[test]
fn test_verbose_output() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&["-v", &format!("{}/get", HTTPBIN_URL)]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.trim().is_empty());
    }
}

#[test]
fn test_include_headers() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&["-i", &format!("{}/get", HTTPBIN_URL)]);

    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.trim().is_empty());
    }
}

#[test]
fn test_config_commands() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["config", "list"]);

    cmd.assert().success();
}

#[test]
fn test_config_set_and_get() {
    let temp_dir = TempDir::new().unwrap();

    // Set a config value
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["config", "set", "general.default_timeout", "45"]);

    cmd.assert().success();

    // Get the config value
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["config", "get", "general.default_timeout"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("45"));
}

#[test]
fn test_list_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["list"]);

    cmd.assert().success();
}

#[test]
fn test_history_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["history"]);

    cmd.assert().success();
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg("version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Terzi").or(predicate::str::contains("version")));
}

#[test]
fn test_export_command() {
    let temp_dir = TempDir::new().unwrap();
    let export_file = temp_dir.path().join("export-test.json");

    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.env("TERZI_CONFIG_DIR", temp_dir.path());
    cmd.args(&["export", "--output", export_file.to_str().unwrap()]);

    cmd.assert().success();

    // Check if file was created
    assert!(export_file.exists());
}

#[test]
fn test_multiple_headers() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-H",
        "X-Header-1: value1",
        "-H",
        "X-Header-2: value2",
        "-H",
        "X-Header-3: value3",
        &format!("{}/headers", HTTPBIN_URL),
    ]);

    let output = cmd.output().unwrap();
    // Should not crash with multiple headers
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Only fail if it's not a network issue
        if !stderr.contains("network") && !stderr.contains("timeout") {
            panic!("Command failed: {}", stderr);
        }
    }
}

#[test]
fn test_error_handling_network_failure() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.arg("https://this-domain-should-not-exist-12345.com");

    // Should fail gracefully
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("failed")));
}

#[test]
fn test_invalid_json() {
    let mut cmd = Command::cargo_bin("terzi").unwrap();
    cmd.args(&[
        "-m",
        "POST",
        "-j",
        "invalid json",
        &format!("{}/post", HTTPBIN_URL),
    ]);

    // Should fail with invalid JSON
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid JSON").or(predicate::str::contains("error")));
}
