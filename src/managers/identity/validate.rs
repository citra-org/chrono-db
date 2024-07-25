use crate::managers;

pub fn validate_keeper(
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    match managers::identity::helper::validate_credentials_helper(username, password) {
        Ok(true) => Ok("OK\n".to_string()),
        Ok(false) => Ok("Error: Invalid credentials\n".to_string()),
        Err(e) => Ok(format!("Error: {:?}\n", e)),
    }
}
