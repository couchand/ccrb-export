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

#[derive(Debug)]
pub enum OfficerError {
    NotEnoughColumns,
    TooManyColumns,
}

impl std::fmt::Display for OfficerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for OfficerError {
}

impl core::convert::TryFrom<Vec<String>> for Officer {
    type Error = OfficerError;

    fn try_from(mut row: Vec<String>) -> Result<Self, Self::Error> {
        let shield_no = row.pop().ok_or(OfficerError::NotEnoughColumns)?;
        let rank = row.pop().ok_or(OfficerError::NotEnoughColumns)?;
        let first_name = row.pop().ok_or(OfficerError::NotEnoughColumns)?;
        let last_name = row.pop().ok_or(OfficerError::NotEnoughColumns)?;
        let command = row.pop().ok_or(OfficerError::NotEnoughColumns)?;
        let id = row.pop().ok_or(OfficerError::NotEnoughColumns)?;

        if !row.is_empty() {
            return Err(OfficerError::TooManyColumns);
        }

        Ok(Officer { id, command, last_name, first_name, rank, shield_no })
    }
}
