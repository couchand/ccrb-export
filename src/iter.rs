use crate::{model, query, response, HOST};

pub struct Index {
    items: Vec<Vec<String>>,
    db: query::Database,
    rt: Option<Vec<String>>,
    client: reqwest::Client,
    progress: Option<Vec<String>>,
}

impl Index {
    pub async fn new(client: reqwest::Client, db: query::Database) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_restart_tokens(client, db, None).await
    }

    pub async fn new_after_officer(client: reqwest::Client, db: query::Database, officer: &model::Officer) -> Result<Self, Box<dyn std::error::Error>> {
        let restart_tokens = vec![
            query::IntoLiteral::stringify(&officer.command),
            query::IntoLiteral::stringify(&officer.id),
            query::IntoLiteral::stringify(&officer.last_name),
            query::IntoLiteral::stringify(&officer.first_name),
            query::IntoLiteral::stringify(&officer.rank),
            query::IntoLiteral::stringify(&officer.shield_no),
        ];
        Self::new_with_restart_tokens(client, db, Some(restart_tokens)).await
    }

    async fn new_with_restart_tokens(client: reqwest::Client, db: query::Database, rt: Option<Vec<String>>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut me = Index {
            items: vec![],
            db,
            rt,
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
        let req = query::get_index(&self.db, self.rt.clone());

        let resp = self.client.post(HOST)
            .header("X-PowerBI-ResourceKey", self.db.get_bi_resource_key())
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
    officer_id: String,
    items: Vec<Vec<String>>,
}

impl Details {
    pub async fn new(client: &reqwest::Client, db: query::Database, officer: &model::Officer) -> Result<Self, Box<dyn std::error::Error>> {
        let req = query::get_followup(&db, officer);

        let resp = client.post(HOST)
            .header("X-PowerBI-ResourceKey", db.get_bi_resource_key())
            .json(&req)
            .send()
            .await?
            .json::<response::Response>()
            .await?;

        let mut items = resp.get_data();
        items.reverse();

        Ok(Details { items, officer_id: officer.id.clone() })
    }
}

impl Iterator for Details {
    type Item = Result<model::Details, model::DeserializeError>;

    fn next(&mut self) -> Option<Self::Item> {
        use core::convert::TryFrom;

        self.items
            .pop()
            .map(model::Details::try_from)
            .map(|res| res.map(|mut i| {
                i.officer_id = self.officer_id.clone();
                i
            }))
    }
}
