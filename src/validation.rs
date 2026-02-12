pub struct InputValidator;

impl InputValidator {
    pub fn validate_file_path(path: &str) -> bool {
        !path.contains("..") && !path.starts_with('/') || path.starts_with("./")
    }

    pub fn validate_command(cmd: &str) -> bool {
        !cmd.contains("&&") && !cmd.contains(";") && !cmd.contains("|")
    }

    pub fn validate_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}