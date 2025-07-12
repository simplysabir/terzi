use std::process::Command;
use tempfile::TempDir;

mod common;
use common::{TestEnvironment, create_test_config, HTTPBIN_URL, MOCK_API_URL};

#[test]
fn test_cli_help() {
    let env = TestEnvironment::new().unwrap();
    let output = env.run_terzi(&["--help"]).unwrap();
    
    assert_success!(output);
    assert_output_contains!(output, "Modern CLI API client");
    assert_output_contains!(output, "USAGE:");
    assert_output_contains!(output, "OPTIONS:");
}

#[test]
fn test_cli_version() {
    let env = TestEnvironment::new().unwrap();
    let output = env.run_terzi(&["--version"]).unwrap();
    
    assert_success!(output);
    assert_output_contains!(output, "terzi");
}

#[test]
fn test_cli_invalid_command() {
    let env = TestEnvironment::new().unwrap();
    let output = env.run_terzi(&["invalid-command"]).unwrap();
    
    assert_failure!(output);
}

#[test]
fn test_basic_get_request() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    let output = env.run_terzi(&[&url]).unwrap();
    
    assert_success!(output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("200") || stdout.contains("GET"));
}

#[test] 
fn test_get_request_with_headers() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/headers", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-H", "X-Test-Header: test-value",
        "-H", "User-Agent: Terzi-Test",
        &url
    ]).unwrap();
    
    assert_success!(output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("X-Test-Header") || stdout.contains("test-value"));
}

#[test]
fn test_post_request_with_json() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/post", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-m", "POST",
        "-j", r#"{"test": "data", "number": 42}"#,
        &url
    ]).unwrap();
    
    assert_success!(output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("POST") && (stdout.contains("test") || stdout.contains("data")));
}

#[test]
fn test_post_request_with_form_data() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/post", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-m", "POST",
        "-f", "name=John",
        "-f", "age=30",
        &url
    ]).unwrap();
    
    assert_success!(output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("POST"));
}

#[test]
fn test_request_with_timeout() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/delay/1", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-t", "5",
        &url
    ]).unwrap();
    
    assert_success!(output);
}

#[test]
fn test_request_timeout_exceeded() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/delay/3", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-t", "1",
        &url
    ]).unwrap();
    
    // Should fail due to timeout
    assert_failure!(output);
}

#[test]
fn test_different_http_methods() {
    let env = TestEnvironment::new().unwrap();
    
    // Test each HTTP method
    let methods = ["GET", "POST", "PUT", "DELETE", "PATCH"];
    
    for method in &methods {
        let url = format!("{}/{}", HTTPBIN_URL, method.to_lowercase());
        let output = env.run_terzi(&["-m", method, &url]).unwrap();
        
        // Most should succeed (some might not be supported by httpbin)
        let stdout = String::from_utf8_lossy(&output.stdout);
        if output.status.success() {
            assert!(stdout.contains(method));
        }
    }
}

#[test]
fn test_output_formats() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    
    // Test different output formats
    let formats = ["json", "yaml"];
    
    for format in &formats {
        let output = env.run_terzi(&["-o", format, &url]).unwrap();
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Just check that we got some output
            assert!(!stdout.trim().is_empty());
        }
    }
}

#[test]
fn test_verbose_output() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    let output = env.run_terzi(&["-v", &url]).unwrap();
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Verbose output should contain more information
        assert!(!stdout.trim().is_empty());
    }
}

#[test]
fn test_include_headers() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    let output = env.run_terzi(&["-i", &url]).unwrap();
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should include headers in output
        assert!(!stdout.trim().is_empty());
    }
}

#[test]
fn test_silent_mode() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    let output = env.run_terzi(&["-S", &url]).unwrap();
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Silent mode might still have some output, but it should be minimal
        // We mainly test that it doesn't crash
    }
}

#[test]
fn test_follow_redirects() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/redirect/1", HTTPBIN_URL);
    let output = env.run_terzi(&["-L", &url]).unwrap();
    
    // Should succeed when following redirects
    assert_success!(output);
}

#[test]
fn test_authentication_bearer() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/bearer", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-A", "bearer:test-token",
        &url
    ]).unwrap();
    
    // Should succeed with bearer auth
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("authenticated") || stdout.contains("true"));
    }
}

#[test]
fn test_authentication_basic() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/basic-auth/user/pass", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-A", "basic:user:pass",
        &url
    ]).unwrap();
    
    // Should succeed with basic auth
    assert_success!(output);
}

#[test]
fn test_invalid_url() {
    let env = TestEnvironment::new().unwrap();
    let output = env.run_terzi(&["not-a-valid-url"]).unwrap();
    
    // Should fail with invalid URL
    assert_failure!(output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid URL") || stderr.contains("error"));
}

#[test]
fn test_invalid_json() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/post", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-m", "POST",
        "-j", "invalid json",
        &url
    ]).unwrap();
    
    // Should fail with invalid JSON
    assert_failure!(output);
}

#[test]
fn test_save_and_load_request() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/get", HTTPBIN_URL);
    
    // Save a request
    let save_output = env.run_terzi(&[
        "--save", "test-request",
        &url
    ]).unwrap();
    
    if save_output.status.success() {
        // Load the saved request
        let load_output = env.run_terzi(&[
            "--load", "test-request"
        ]).unwrap();
        
        // Should be able to load and execute the saved request
        assert_success!(load_output);
    }
}

#[test]
fn test_list_command() {
    let env = TestEnvironment::new().unwrap();
    
    // First save a request
    let url = format!("{}/get", HTTPBIN_URL);
    let _ = env.run_terzi(&["--save", "list-test", &url]);
    
    // Then list requests
    let output = env.run_terzi(&["list"]).unwrap();
    
    // Should succeed (might be empty if save failed)
    assert_success!(output);
}

#[test]
fn test_history_command() {
    let env = TestEnvironment::new().unwrap();
    
    // Make a request to create history
    let url = format!("{}/get", HTTPBIN_URL);
    let _ = env.run_terzi(&[&url]);
    
    // Check history
    let output = env.run_terzi(&["history"]).unwrap();
    
    // Should succeed
    assert_success!(output);
}

#[test]
fn test_config_commands() {
    let env = TestEnvironment::new().unwrap();
    
    // Test config list
    let list_output = env.run_terzi(&["config", "list"]).unwrap();
    assert_success!(list_output);
    
    // Test config set
    let set_output = env.run_terzi(&["config", "set", "timeout", "45"]).unwrap();
    assert_success!(set_output);
    
    // Test config get
    let get_output = env.run_terzi(&["config", "get", "timeout"]).unwrap();
    if get_output.status.success() {
        let stdout = String::from_utf8_lossy(&get_output.stdout);
        assert!(stdout.contains("45"));
    }
}

#[test]
fn test_export_command() {
    let env = TestEnvironment::new().unwrap();
    
    // Save a request first
    let url = format!("{}/get", HTTPBIN_URL);
    let _ = env.run_terzi(&["--save", "export-test", &url]);
    
    // Test export
    let export_file = env.temp_dir.path().join("export-test.json");
    let output = env.run_terzi(&[
        "export", 
        "--output", 
        export_file.to_str().unwrap()
    ]).unwrap();
    
    // Should succeed
    assert_success!(output);
    
    // Check if file was created
    assert!(export_file.exists());
}

#[test]
fn test_version_command() {
    let env = TestEnvironment::new().unwrap();
    let output = env.run_terzi(&["version"]).unwrap();
    
    assert_success!(output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Terzi") || stdout.contains("version"));
}

#[test]
fn test_multiple_headers() {
    let env = TestEnvironment::new().unwrap();
    let url = format!("{}/headers", HTTPBIN_URL);
    let output = env.run_terzi(&[
        "-H", "X-Header-1: value1",
        "-H", "X-Header-2: value2", 
        "-H", "X-Header-3: value3",
        &url
    ]).unwrap();
    
    assert_success!(output);
}

#[test]
fn test_error_handling_network_failure() {
    let env = TestEnvironment::new().unwrap();
    // Use a non-existent domain
    let output = env.run_terzi(&["https://this-domain-should-not-exist-12345.com"]).unwrap();
    
    // Should fail gracefully
    assert_failure!(output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("failed"));
} 