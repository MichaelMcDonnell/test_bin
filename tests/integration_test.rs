#[test]
fn basic_usage() {
    // This is the simplest way of using the crate.
    let output = test_bin::get_test_bin("test_bin")
        .output()
        .expect("Failed to start test_bin");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Output from my CLI app!\n"
    );
}

#[test]
fn macro_usage() {
    // The macro was added because there was talk of changing cargo in a way
    // that would have broken the `get_test_bin` function. Those plans changed
    // and it is ok to use the function.
    let output = test_bin::get_test_bin!("test_bin")
        .output()
        .expect("Failed to start test_bin");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Output from my CLI app!\n"
    );
}

macro_rules! bin_name {
    () => {
        "test_bin"
    };
}

#[test]
fn macro_usage_with_name_macro() {
    // You unfortunately need a macro if you don't want to retype the binary
    // name because the `concat!` macro doesn't support constant string
    // references. You can alternatively just use the `get_test_bin` function
    // instead.
    let output = test_bin::get_test_bin!(bin_name!())
        .output()
        .expect("Failed to start test_bin");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Output from my CLI app!\n"
    );
}
