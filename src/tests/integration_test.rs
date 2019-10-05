#[test]
fn basic_usage() {
    let output = test_bin::get_test_bin("test_bin")
        .output()
        .expect("Failed to start test_bin");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Output from my CLI app!\n"
    );
}
