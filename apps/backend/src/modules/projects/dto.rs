use serde::{Deserialize, Serialize};

const TITLE_MAX_LEN: usize = 100;
const TAG_MAX_LEN: usize = 32;
const QUERY_MAX_LEN: usize = 100;
const MAX_TAG_COUNT: usize = 10;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectListDto {
    pub items: Vec<ProjectDto>,
}

pub enum ProjectFilter {
    Json(ProjectFilterJson),
    Query(ProjectFilterQuery),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProjectFilterJson {
    pub title: Option<String>,
}

pub trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

fn validate_optional_max(value: Option<&str>, max: usize, message: &str) -> Result<(), String> {
    if let Some(value) = value && value.len() > max {
        return Err(message.to_string());
    }

    Ok(())
}

fn validate_max_items<T>(items: &[T], max: usize, message: &str) -> Result<(), String> {
    if items.len() > max {
        return Err(message.to_string());
    }

    Ok(())
}

fn validate_item_max_len(items: &[String], max: usize, message: &str) -> Result<(), String> {
    if items.iter().any(|item| item.len() > max) {
        return Err(message.to_string());
    }

    Ok(())
}

impl ProjectFilterJson {
    pub fn validate(&self) -> Result<(), String> {
        validate_optional_max(
            self.title.as_deref(),
            TITLE_MAX_LEN,
            "title terlalu panjang (max 100)",
        )
    }
}

impl Validatable for ProjectFilterJson {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProjectFilterQuery {
    pub tag: Option<String>,
    pub q: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl ProjectFilterQuery {
    pub fn validate(&self) -> Result<(), String> {
        validate_optional_max(self.tag.as_deref(), TAG_MAX_LEN, "tag terlalu panjang (max 32)")?;
        validate_optional_max(self.q.as_deref(), QUERY_MAX_LEN, "q terlalu panjang (max 100)")?;
        validate_max_items(&self.tags, MAX_TAG_COUNT, "tags terlalu banyak (max 10)")?;
        validate_item_max_len(
            &self.tags,
            TAG_MAX_LEN,
            "tags mengandung item terlalu panjang (max 32)",
        )?;

        Ok(())
    }
}

impl Validatable for ProjectFilterQuery {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl From<ProjectFilterJson> for ProjectFilter {
    fn from(value: ProjectFilterJson) -> Self {
        Self::Json(value)
    }
}

impl From<ProjectFilterQuery> for ProjectFilter {
    fn from(value: ProjectFilterQuery) -> Self {
        Self::Query(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{ProjectFilterJson, ProjectFilterQuery};

    #[test]
    fn rejects_title_longer_than_limit() {
        let filter = ProjectFilterJson {
            title: Some("a".repeat(101)),
        };
        assert_eq!(
            filter.validate(),
            Err("title terlalu panjang (max 100)".to_string())
        );
    }

    #[test]
    fn rejects_tag_longer_than_limit() {
        let filter = ProjectFilterQuery {
            tag: Some("a".repeat(33)),
            ..Default::default()
        };
        assert_eq!(
            filter.validate(),
            Err("tag terlalu panjang (max 32)".to_string())
        );
    }

    #[test]
    fn rejects_q_longer_than_limit() {
        let filter = ProjectFilterQuery {
            q: Some("a".repeat(101)),
            ..Default::default()
        };
        assert_eq!(
            filter.validate(),
            Err("q terlalu panjang (max 100)".to_string())
        );
    }

    #[test]
    fn rejects_too_many_tags() {
        let filter = ProjectFilterQuery {
            tags: vec!["tag".to_string(); 11],
            ..Default::default()
        };
        assert_eq!(
            filter.validate(),
            Err("tags terlalu banyak (max 10)".to_string())
        );
    }

    #[test]
    fn rejects_single_tag_item_longer_than_limit() {
        let filter = ProjectFilterQuery {
            tags: vec!["a".repeat(33)],
            ..Default::default()
        };
        assert_eq!(
            filter.validate(),
            Err("tags mengandung item terlalu panjang (max 32)".to_string())
        );
    }
}
