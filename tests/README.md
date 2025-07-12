# Terzi Tests

Comprehensive test suite for Terzi CLI API client.

## 📋 Test Structure

```
tests/
├── README.md                 # This file
├── integration/              # Integration tests
│   ├── cli_tests.rs          # CLI command tests
```

## 🚀 Running Tests

### All Tests
```bash
cargo test
```

### With Output
```bash
cargo test -- --nocapture
```

### Specific Test
```bash
cargo test test_basic_get_request
cargo test test_config_set_timeout
```

## 📊 Test Coverage

Run tests with coverage:

```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage/

# View coverage report
open coverage/tarpaulin-report.html
```

## 🛠️ Test Development

### Adding New Tests

1. **Unit Tests**: Add to `src/` files with `#[cfg(test)]` modules
2. **Integration Tests**: Add to `tests/` directory
3. **Mock Tests**: Use WireMock for HTTP mocking

### Test Naming Convention

```rust
#[test]
fn test_<functionality>_<scenario>() {
    // Test implementation
}
```

Examples:
- `test_cli_basic_get_request()`
- `test_config_set_valid_timeout()`
- `test_auth_bearer_token_success()`
- `test_utils_format_duration_milliseconds()`

### Test Structure

```rust
#[tokio::test]
async fn test_example() {
    // Arrange - Set up test data and conditions
    let client = TerziClient::new(&Config::default()).unwrap();
    let url = "https://httpbin.org/get";
    
    // Act - Execute the functionality being tested
    let result = client.get(url).await;
    
    // Assert - Verify the results
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 200);
}
```

## 🔧 Test Utilities

### Mock Server Helpers

```rust
use tests::common::helpers::*;

#[tokio::test]
async fn test_with_mock_server() {
    let mock_server = create_mock_server().await;
    
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "message": "success"
        })))
        .mount(&mock_server)
        .await;
    
    // Test with mock server URL
    let result = terzi_request(&mock_server.uri()).await;
    assert!(result.is_ok());
}
```

### Environment Variables

Set these for integration tests:

```bash
export TERZI_TEST_MODE=true
export TERZI_TEST_CONFIG_DIR=/tmp/terzi-test
export TERZI_TEST_TIMEOUT=10
```

## 🐛 Debugging Tests

### Verbose Output
```bash
cargo test -- --nocapture
```

### Test-specific Logging
```rust
#[tokio::test]
async fn test_debug_example() {
    env_logger::init();
    
    // Your test code here
    log::debug!("Debug message");
}
```

### Failed Test Investigation
```bash
# Run only failing tests
cargo test --failed

# Show test output
cargo test test_name -- --exact --nocapture
```

## 🚦 Continuous Integration

### GitHub Actions

Tests run automatically on:
- Pull requests
- Pushes to master branch

### Local Pre-commit

Run before committing:

```bash
# Format code
cargo fmt --all

# Check linting
cargo clippy --all-targets --all-features

# Run all tests
cargo test --all

# Check documentation
cargo doc --no-deps
```

## 📋 Test Checklist

When adding new features:

- [ ] Unit tests for new functions
- [ ] Integration tests for CLI commands
- [ ] Error handling tests
- [ ] Edge case tests
- [ ] Documentation tests (if applicable)
- [ ] Performance tests (if relevant)

When fixing bugs:

- [ ] Reproduction test that fails before fix
- [ ] Test passes after fix
- [ ] Related functionality still works
- [ ] Regression test added

## 🎯 Testing Best Practices

1. **Test Independence** - Each test should be independent
2. **Clear Naming** - Test names should describe what they test
3. **Arrange-Act-Assert** - Follow the AAA pattern
4. **Mock External Dependencies** - Use mocks for HTTP APIs
5. **Test Error Cases** - Don't just test the happy path
6. **Keep Tests Fast** - Use mocks instead of real network calls
7. **Clean Up** - Remove test artifacts after tests

## 📞 Getting Help

- **Test Failures** - Check logs and use `--nocapture`
- **Slow Tests** - Profile with `cargo test --release`
- **Flaky Tests** - Run multiple times to identify issues
- **Mock Issues** - Check WireMock documentation

---

For questions about testing, see the [contributing guide](../docs/contributing.md) or open an issue. 