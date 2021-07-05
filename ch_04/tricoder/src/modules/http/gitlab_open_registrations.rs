use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct GitlabOpenRegistrations {}

impl GitlabOpenRegistrations {
    pub fn new() -> Self {
        GitlabOpenRegistrations {}
    }
}

impl Module for GitlabOpenRegistrations {
    fn name(&self) -> String {
        String::from("http/gitlab_open_registration")
    }

    fn description(&self) -> String {
        String::from("Check if the GitLab instance is open to registrations")
    }
}

#[async_trait]
impl HttpModule for GitlabOpenRegistrations {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        if res.content_length().is_none() {
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        if res.content_length().unwrap() > 2_000_000 {
            // prevent DOS
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        let body = res.text().await?;
        if body.to_lowercase().contains("ref:") && body.contains("Register") {
            return Ok(Some(HttpFinding::GitlabOpenRegistrations(url)));
        }

        Ok(None)
    }
}