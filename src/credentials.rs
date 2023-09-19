use std::cell::RefCell;

use chrono::{DateTime, Duration, Utc};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use tonic::Request;

#[derive(Debug, Clone, Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Clone, Default)]
struct Credential {
    access_token: String,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CredentialsService {
    oauth_url: String,
    client_id: String,
    client_secret: String,
    credential: RefCell<Credential>,
}

impl CredentialsService {
    const AUTH_PARAMS: [(&'static str, &'static str); 2] = [
        ("grant_type", "client_credentials"),
        ("scope", "openid urn:zitadel:iam:user:metadata"),
    ];

    pub fn new(
        oauth_url: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            oauth_url,
            client_id,
            client_secret,
            credential: RefCell::new(Credential::default()),
        }
    }

    fn is_expired(&self) -> bool {
        self.credential.borrow().expires_at < Utc::now()
    }

    fn get_token_url(&self) -> String {
        format!("{}/v2/token", self.oauth_url)
    }

    async fn get_token(&self) {
        let client = reqwest::Client::new();

        let now = Utc::now();

        let response: AuthResponse = client
            .post(self.get_token_url())
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&Self::AUTH_PARAMS)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let new_credentials = Credential {
            access_token: response.access_token,
            expires_at: now + Duration::seconds(response.expires_in),
        };

        self.credential.replace(new_credentials);
    }

    async fn ensure_fresh_token(&self) -> String {
        if self.is_expired() {
            self.get_token().await;
        }

        self.credential.borrow().access_token.to_owned()
    }

    pub async fn with_auth_header<T>(&self, request: &mut Request<T>) {
        let token = self.ensure_fresh_token().await;
        let header_value = format!("Bearer {}", token);

        request
            .metadata_mut()
            .insert(AUTHORIZATION.as_str(), header_value.parse().unwrap());
    }
}
