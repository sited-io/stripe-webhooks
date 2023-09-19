#[derive(Debug, Clone)]
pub struct AppSettings {
    pub stripe_endpoint_secret: String,
}

impl AppSettings {
    pub fn new(stripe_endpoint_secret: String) -> Self {
        Self {
            stripe_endpoint_secret,
        }
    }
}
