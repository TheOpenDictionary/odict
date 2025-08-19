use reqwest::Client;
use std::sync::OnceLock;

/// Global HTTP client singleton for efficient connection reuse
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// Get the global HTTP client instance
pub fn get_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("odict/2.9.1")
            .build()
            .expect("Failed to create HTTP client")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_singleton() {
        let client1 = get_client();
        let client2 = get_client();

        // Should be the same instance
        assert!(std::ptr::eq(client1, client2));
    }
}
