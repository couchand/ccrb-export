use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Officer {
    pub id: String,
    pub command: String,
    pub last_name: String,
    pub first_name: String,
    pub rank: String,
    pub shield_no: String,
}

impl core::convert::TryFrom<Vec<String>> for Officer {
    type Error = DeserializeError;

    fn try_from(mut row: Vec<String>) -> Result<Self, Self::Error> {
        let shield_no = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let rank = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let first_name = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let last_name = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let command = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let id = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;

        if !row.is_empty() {
            return Err(DeserializeError::TooManyColumns);
        }

        Ok(Officer { id, command, last_name, first_name, rank, shield_no })
    }
}

#[derive(Debug, Serialize)]
pub struct Details {
    pub officer_id: String,
    pub index: String,
    pub complaint_id: String,
    pub incident_date: String,
    pub fado_type: String,
    pub allegation: String,
    pub board_disposition: String,
    pub nypd_disposition: String,
    pub penalty: String,
}

impl core::convert::TryFrom<Vec<String>> for Details {
    type Error = DeserializeError;

    fn try_from(mut row: Vec<String>) -> Result<Self, Self::Error> {
        let penalty = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let nypd_disposition = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let board_disposition = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let allegation = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let fado_type = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let incident_date = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let complaint_id = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;
        let index = row.pop().ok_or(DeserializeError::NotEnoughColumns)?;

        if !row.is_empty() {
            return Err(DeserializeError::TooManyColumns);
        }

        Ok(Details {
            officer_id: String::new(), // TODO: this is awkward
            index,
            complaint_id,
            incident_date,
            fado_type,
            allegation,
            board_disposition,
            nypd_disposition,
            penalty,
        })
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    NotEnoughColumns,
    TooManyColumns,
}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DeserializeError {
}
