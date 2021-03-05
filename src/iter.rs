use crate::{query, response, HOST};

pub struct Index {
    items: Vec<Vec<String>>,
    rt: Option<Vec<String>>,
    client: reqwest::Client,
    progress: Option<Vec<String>>,
}

impl Index {
    pub async fn new(client: reqwest::Client) -> Result<Self, Box<dyn std::error::Error>> {
        let mut me = Index {
            items: vec![],
            rt: None,
            client,
            progress: None,
        };

        me.query_more()
            .await?;

        Ok(me)
    }

    pub async fn next(&mut self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        if let Some(item) = self.items.pop() {
            return Ok(Some(item));
        }

        if self.rt.is_none() {
            return Ok(None);
        }

        self.query_more()
            .await?;

        Ok(self.items.pop())
    }

    async fn query_more(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let req = query::get_index(self.rt.clone());

        let resp = self.client.post(HOST)
            .json(&req)
            .send()
            .await?
            .json::<response::Response>()
            .await?;

        self.items = resp.get_data();
        self.items.reverse();

        self.rt = resp.get_restart_tokens();
        self.progress = self.rt.clone();

        Ok(())
    }

    pub fn progress(&mut self) -> Option<Vec<String>> {
        self.progress.take()
    }
}
