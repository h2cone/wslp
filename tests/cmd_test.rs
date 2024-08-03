#[cfg(test)]
mod tests {
    use clap::Parser;
    use std::process::Command;
    use wslp::cmd::Args;

    #[test]
    fn test_convert_wsl_to_windows() {
        // Set up the arguments
        let args = Args::parse_from(&["test", "--reverse", "/mnt/c/Users/test"]);

        // Check the conversion
        assert_eq!(args.reverse, true);
        assert_eq!(args.paths, vec!["/mnt/c/Users/test"]);

        // Capture the output
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--reverse")
            .arg("/mnt/c/Users/test")
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains(r"c:\Users\test"));
    }

    #[test]
    fn test_convert_windows_to_wsl() {
        // Set up the arguments
        let args = Args::parse_from(&["test", r"C:\Users\test"]);

        // Check the conversion
        assert_eq!(args.reverse, false);
        assert_eq!(args.paths, vec![r"C:\Users\test"]);

        // Capture the output
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(r"C:\Users\test")
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("/mnt/c/Users/test"));
    }
}
