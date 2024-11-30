use crate::env::Dotenv;

/// Retrieves the server address by combining the `HOST` and `PORT` environment variables.
#[doc = include_str!("docs/helpers/get_address.md")]
pub fn get_address() -> String {
    let host: String = Dotenv::get_or_default("HOST", "127.0.0.1");

    let port: u16 = Dotenv::get_or_default("PORT", "3000")
        .parse::<u16>() // Converte a string para u16
        .unwrap_or_else(|_| panic!("Invalid PORT value in environment variables"));

    format!("{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use serial_test::serial;

    /// Helper function to clear environment variables used in tests.
    fn clear_env_var(key: &str) {
        env::remove_var(key);
    }

    /// Helper function to set multiple environment variables.
    fn set_env_vars(vars: &[(&str, &str)]) {
        for (key, value) in vars {
            env::set_var(key, value);
        }
    }

    #[test]
    #[serial]
    fn test_get_address_with_default_values() {
        clear_env_var("HOST");
        clear_env_var("PORT");

        let address = get_address();
        assert_eq!(address, "127.0.0.1:3000");
    }

    #[test]
    #[serial]
    fn test_get_address_with_custom_host_and_default_port() {
        set_env_vars(&[("HOST", "192.168.1.1")]);
        clear_env_var("PORT");

        let address = get_address();
        assert_eq!(address, "192.168.1.1:3000");

        clear_env_var("HOST");
    }

    #[test]
    #[serial]
    fn test_get_address_with_custom_host_and_port() {
        set_env_vars(&[("HOST", "192.168.1.1"), ("PORT", "8080")]);

        let address = get_address();
        assert_eq!(address, "192.168.1.1:8080");

        clear_env_var("HOST");
        clear_env_var("PORT");
    }

    #[test]
    #[serial]
    #[should_panic(expected = "Invalid PORT value in environment variables")]
    fn test_get_address_with_invalid_port() {
        set_env_vars(&[("HOST", "192.168.1.1"), ("PORT", "invalid_port")]);

        get_address();

        clear_env_var("HOST");
        clear_env_var("PORT");
    }
}
