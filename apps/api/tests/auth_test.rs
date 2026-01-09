// Unit tests for authentication service

#[cfg(test)]
mod tests {
    use lean_wms_api::services::auth::AuthService;
    use lean_wms_api::config::Config;

    fn create_test_config() -> Config {
        Config {
            database_url: "postgresql://test:test@localhost/test".to_string(),
            jwt_secret: "test-secret-key-for-testing-only".to_string(),
            jwt_refresh_secret: "test-refresh-secret-key-for-testing-only".to_string(),
            api_port: 3000,
        }
    }

    #[tokio::test]
    async fn test_generate_token() {
        let config = create_test_config();
        let auth_service = AuthService::new(&config);
        
        let token = auth_service.generate_token("test-user-id");
        assert!(token.is_ok());
    }

    #[tokio::test]
    async fn test_verify_token() {
        let config = create_test_config();
        let auth_service = AuthService::new(&config);
        
        let token = auth_service.generate_token("test-user-id").unwrap();
        let claims = auth_service.verify_token(&token);
        
        assert!(claims.is_ok());
        assert_eq!(claims.unwrap().sub, "test-user-id");
    }

    #[tokio::test]
    async fn test_hash_and_verify_password() {
        let password = "test-password-123";
        let hash = AuthService::hash_password(password).unwrap();
        
        assert!(AuthService::verify_password(password, &hash).unwrap());
        assert!(!AuthService::verify_password("wrong-password", &hash).unwrap());
    }
}

