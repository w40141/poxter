use anyhow::{anyhow, Result};
use chrono::prelude::*;

use super::content::Content;
use super::profile_name::ProfileName;

#[derive(Debug, Clone)]
pub struct Bio {
    profile_name: ProfileName,
    content: Content,
    created_date: DateTime<Local>,
    updated_date: DateTime<Local>,
}

impl Bio {
    pub fn profile_name(&self) -> &String {
        &self.profile_name.name()
    }

    pub fn content(&self) -> &String {
        self.content.content()
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }

    pub fn updated_date(&self) -> &DateTime<Local> {
        &self.updated_date
    }
}

pub struct BioBuilder {
    profile_name: Option<String>,
    content: Option<String>,
    created_date: Option<DateTime<Local>>,
    updated_date: Option<DateTime<Local>>,
}

impl BioBuilder {
    pub fn default() -> Self {
        Self {
            profile_name: None,
            content: None,
            created_date: None,
            updated_date: None,
        }
    }

    pub fn profit_name(mut self, v: String) -> Self {
        self.profile_name = Some(v);
        self
    }

    pub fn content(mut self, v: String) -> Self {
        self.content = Some(v);
        self
    }

    pub fn created_date(mut self, v: DateTime<Local>) -> Self {
        self.created_date = Some(v);
        self
    }

    pub fn updated_date(mut self, v: DateTime<Local>) -> Self {
        self.updated_date = Some(v);
        self
    }

    pub fn build(&self) -> Result<Bio> {
        let profile_name = match &self.profile_name {
            Some(v) => ProfileName::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let content = match &self.content {
            Some(v) => Content::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let created_date = match self.created_date {
            Some(v) => v,
            None => Local::now(),
        };

        let updated_date = match self.updated_date {
            Some(v) => v,
            None => Local::now(),
        };

        Ok(Bio {
            profile_name,
            content,
            created_date,
            updated_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bio_test() {
        {
            // Correct
            let result = BioBuilder::default()
                .profit_name("taro".to_string())
                .content("abc".to_string())
                .build();
            assert!(result.is_ok());
        }
        {
            // Incorrect
            let result = BioBuilder::default().build();
            assert!(result.is_err());
        }
        {
            let result = BioBuilder::default()
                .profit_name("taro".to_string())
                .build();
            assert!(result.is_err());
        }
        {
            // Incorrect
            let result = BioBuilder::default().content("abc".to_string()).build();
            assert!(result.is_err());
        }
    }
}
