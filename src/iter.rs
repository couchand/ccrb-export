use crate::{model, query, response, HOST};

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

    pub async fn next(&mut self) -> Result<Option<model::Officer>, Box<dyn std::error::Error>> {
        use core::convert::TryFrom;

        if let Some(row) = self.items.pop() {
            return Ok(Some(model::Officer::try_from(row)?));
        }

        if self.rt.is_none() {
            return Ok(None);
        }

        self.query_more()
            .await?;

        let row = self.items.pop();
        Ok(row.map(|row| model::Officer::try_from(row)).transpose()?)
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

pub struct Details {
    items: Vec<Vec<String>>,
}

impl Details {
    pub async fn new(client: &reqwest::Client, officer: &model::Officer) -> Result<Self, Box<dyn std::error::Error>> {
        let req = query::get_followup(officer);

        let resp = client.post(HOST)
            .json(&req)
            .send()
            .await?
            .json::<response::Response>()
            .await?;

        let mut items = resp.get_data();
        items.reverse();

        for item in &mut items {
            item.officer_id = officer.id.clone();
        }

        Ok(Details { items })
    }
}

impl Iterator for Details {
    type Item = Result<model::Details, model::DeserializeError>;

    fn next(&mut self) -> Option<Self::Item> {
        use core::convert::TryFrom;

        self.items.pop().map(model::Details::try_from)
    }
}
