//! A module for getting the crate binary in an integration test.
//!
//! If you are writing a command-line interface app then it is useful to write
//! an integration test that uses the binary. You most likely want to launch the
//! binary and inspect the output. This module lets you get the binary so it can
//! be tested.
//!
//! # Examples
//!
//! basic usage:
//!
//! ```no_run
//! let output = test_bin::get_test_bin!("my_cli_app")
//!     .output()
//!     .expect("Failed to start my_binary");
//! assert_eq!(
//!     String::from_utf8_lossy(&output.stdout),
//!     "Output from my CLI app!\n"
//! );
//! ```
//!
//! Refer to the [`std::process::Command` documentation](https://doc.rust-lang.org/std/process/struct.Command.html)
//! for how to pass arguments, check exit status and more.
//!
//! NOTE: There is also the older non-macro `get_test_bin` which has been
//! deprecated. I has been deprecated because there is work to allow cargo to
//! write its output to new paths. See [Cargo issue 14125](https://github.com/rust-lang/cargo/issues/14125).
//!
//! The `get_test_bin` macro uses the `CARGO_BIN_EXE_<name>` environment
//! variable which was introduced in [Rust 1.43 released on 23 April 2020](https://releases.rs/docs/1.43.0/).
//!

// Returns the crate's binary as a `Command` that can be used for integration
/// tests.
///
/// # Arguments
///
/// * `bin_name` - The name of the binary you want to test.
///
/// # Remarks
///
/// It will fail to compile if the `bin_name` is incorrect. The `bin_name` is
/// used for creating an environment variable.
#[macro_export]
macro_rules! get_test_bin {
    ($x:expr) => {
        {
            // Get path string. See the CARGO_BIN_EXE_<name> documentation:
            // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
            let path_str = env!(concat!("CARGO_BIN_EXE_", $x));
            // Create command
            std::process::Command::new(path_str)
        }
    };
}

/// Returns the crate's binary as a `Command` that can be used for integration
/// tests.
///
/// # Arguments
///
/// * `bin_name` - The name of the binary you want to test.
///
/// # Remarks
///
/// It panics on error. This is by design so the test that uses it fails.
#[deprecated(since = "0.5.0", note = "please use `get_test_bin!` macro instead")]
pub fn get_test_bin(bin_name: &str) -> std::process::Command {
    // Create full path to binary
    let mut path = get_test_bin_dir();
    path.push(bin_name);
    path.set_extension(std::env::consts::EXE_EXTENSION);

    if !path.exists() {
        // Print all environment variables.
        for (key, value) in std::env::vars() {
            println!("{key}: {value}");
        }
        let path: &'static str = env!("PATH");
        println!("the $PATH variable at the time of compiling was: {path}");
        let build_dir = std::env::var("CARGO_BIN_EXE_test_bin").unwrap();
        panic!("Environment variable is {}", build_dir);
    }

    assert!(path.exists());

    // Create command
    std::process::Command::new(path.into_os_string())
}

/// Returns the directory of the crate's binary.
///
/// # Remarks
///
/// It panics on error. This is by design so the test that uses it fails.
fn get_test_bin_dir() -> std::path::PathBuf {
    // Cargo puts the integration test binary in target/debug/deps
    let current_exe =
        std::env::current_exe().expect("Failed to get the path of the integration test binary");
    let current_dir = current_exe
        .parent()
        .expect("Failed to get the directory of the integration test binary");

    let test_bin_dir = current_dir
        .parent()
        .expect("Failed to get the binary folder");
    test_bin_dir.to_owned()
}
