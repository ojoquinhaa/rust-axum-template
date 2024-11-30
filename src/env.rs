use std::env;

/// A utility struct for managing environment variables using dotenv.
#[doc = include_str!("docs/dotenv/main.md")]
pub struct Dotenv;

impl Dotenv {
    #[doc = include_str!("docs/dotenv/new.md")]
    /// Initializes the dotenv loader.
    ///
    /// This function loads environment variables from a `.env` file
    /// into the application's runtime environment. If the file is
    /// missing or an error occurs, the function silently ignores it.
    pub fn new() {
        dotenvy::dotenv().ok();
    }

    #[doc = include_str!("docs/dotenv/get.md")]
    /// Retrieves the value of an environment variable.
    ///
    /// This function fetches the value of a given environment variable by its key.
    /// If the variable is not found, it returns an error with a descriptive message.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the name of the environment variable.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(String)` containing the value of the environment variable if found.
    /// - `Err(String)` with an error message if the variable is not set.
    pub fn get(key: &str) -> Result<String, String> {
        env::var(key).map_err(|_| format!("Environment variable `{}` not found", key))
    }

    #[doc = include_str!("docs/dotenv/get_or_default.md")]
    /// Retrieves the value of an environment variable or returns a default value.
    ///
    /// This function attempts to fetch the value of a given environment variable
    /// by its key. If the variable is not set, it falls back to the provided default value.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the name of the environment variable.
    /// * `default` - A string slice that holds the default value to return if the variable is not set.
    ///
    /// # Returns
    ///
    /// A `String` containing:
    /// - The value of the environment variable if found.
    /// - The provided default value if the variable is not set.
    pub fn get_or_default(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Helper function to clear environment variables used in tests.
    fn clear_env_var(key: &str) {
        env::remove_var(key);
    }

    #[test]
    fn test_new_loads_env() {
        // Preparação: Define uma variável no .env ou diretamente
        // Para este teste, vamos definir manualmente uma variável
        let key = "TEST_NEW_VAR";
        let value = "new_value";
        env::set_var(key, value);

        // Executa a função new
        Dotenv::new();

        // Verifica se a variável está definida
        let result = Dotenv::get(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);

        // Limpeza
        clear_env_var(key);
    }

    #[test]
    fn test_get_existing_var() {
        let key = "TEST_GET_VAR";
        let value = "get_value";
        env::set_var(key, value);

        // Testa a função get com uma variável existente
        let result = Dotenv::get(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);

        // Limpeza
        clear_env_var(key);
    }

    #[test]
    fn test_get_non_existing_var() {
        let key = "TEST_NON_EXISTING_VAR";

        // Assegura que a variável não está definida
        clear_env_var(key);

        // Testa a função get com uma variável inexistente
        let result = Dotenv::get(key);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            format!("Environment variable `{}` not found", key)
        );
    }

    #[test]
    fn test_get_or_default_existing_var() {
        let key = "TEST_GET_OR_DEFAULT_EXISTING_VAR";
        let value = "existing_value";
        let default = "default_value";
        env::set_var(key, value);

        // Testa a função get_or_default com uma variável existente
        let result = Dotenv::get_or_default(key, default);
        assert_eq!(result, value);

        // Limpeza
        clear_env_var(key);
    }

    #[test]
    fn test_get_or_default_non_existing_var() {
        let key = "TEST_GET_OR_DEFAULT_NON_EXISTING_VAR";
        let default = "default_value";

        // Assegura que a variável não está definida
        clear_env_var(key);

        // Testa a função get_or_default com uma variável inexistente
        let result = Dotenv::get_or_default(key, default);
        assert_eq!(result, default);
    }
}
