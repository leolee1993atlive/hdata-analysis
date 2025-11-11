use rbdc::DateTime;
use serde::{Deserialize, Serialize};

// 公共字段结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub version: i64,
    pub created_by: i64,
    pub created_date: DateTime,
    pub last_modified_by: i64,
    pub last_modified_date: DateTime,
    pub deleted_by: Option<i64>,
    pub deleted_date: Option<DateTime>,
}

impl BaseEntity {
    pub fn new(created_by: i64) -> Self {
        Self {
            version: 1,
            created_by,
            created_date: DateTime::now(),
            last_modified_by: created_by,
            last_modified_date: DateTime::now(),
            deleted_by: None,
            deleted_date: None,
        }
    }

    pub fn update(&mut self, last_modified_by: i64) {
        self.version += 1;
        self.last_modified_by = last_modified_by;
        self.last_modified_date = DateTime::now();
    }

    pub fn delete(&mut self, deleted_by: i64) {
        self.deleted_by = Some(deleted_by);
        self.deleted_date = Some(DateTime::now());
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_by.is_some() && self.deleted_date.is_some()
    }
}
