use reqwest::cookie::Jar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

//use auth_service::app_state::{AppState, UserStoreType};
//use auth_service::services::hashmap_user_store::HashmapUserStore;
//use auth_service::Application;
use auth_service::{
    app_state::{AppState, UserStoreType},
    services::hashmap_user_store::HashmapUserStore,
    utils::constants::test,
    Application,
};

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());

        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
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

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    //X    pub async fn login(&self) -> reqwest::Response {
    //X        #[derive(Debug, Serialize, Deserialize)]
    //X        struct Person {
    //X            first_name: String,
    //X            last_name: String,
    //X        }
    //X
    //X        let p = Person {
    //X            first_name: "Foo".into(),
    //X            last_name: "Bar".into(),
    //X        };
    //X
    //X        self.http_client
    //X            .post(&format!("{}/login", &self.address))
    //X            //    .header("Content-Type", "application/json")
    //X            //    .json(&p)
    //X            .send()
    //X            .await
    //X            .expect("Failed to execute request.")
    //X    }

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

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
