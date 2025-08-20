#[test]
fn test_greeting_contains_name() {
    let name = "Graeme";
    let output = format!("Hello, my name is {}!", name);
    assert!(output.contains(name));
}s