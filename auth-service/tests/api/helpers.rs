use auth_service::Application;
use serde::{Deserialize, Serialize};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)

    pub async fn signup(&self) -> reqwest::Response {
        #[derive(Debug, Serialize, Deserialize)]
        struct Person {
            first_name: String,
            last_name: String,
        }

        let p = Person {
            first_name: "Foo".into(),
            last_name: "Bar".into(),
        };

        self.http_client
            .post(&format!("{}/signup", &self.address))
            //    .header("Content-Type", "application/json")
            //    .json(&p)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn login(&self) -> reqwest::Response {
        #[derive(Debug, Serialize, Deserialize)]
        struct Person {
            first_name: String,
            last_name: String,
        }

        let p = Person {
            first_name: "Foo".into(),
            last_name: "Bar".into(),
        };

        self.http_client
            .post(&format!("{}/login", &self.address))
            //    .header("Content-Type", "application/json")
            //    .json(&p)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            //    .header("Content-Type", "application/json")
            //    .json(&p)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_2fa", &self.address))
            //    .header("Content-Type", "application/json")
            //    .json(&p)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_token", &self.address))
            //    .header("Content-Type", "application/json")
            //    .json(&p)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
