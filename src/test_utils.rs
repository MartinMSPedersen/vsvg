#[cfg(test)]

mod test_utils {

    /// Macro to return a test file path located in `resources/fixtures`.
    #[macro_export]
    macro_rules! test_file {
        ($fname:expr) => {{
            let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("resources");
            path.push("fixtures");
            path.push($fname);
            path
        }};
    }
}
