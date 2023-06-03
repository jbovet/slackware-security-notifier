use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Advisor {
    pub _id: Option<ObjectId>,
    pub date: DateTime,
    pub package_name: String,
    pub url: String,
    pub year: i32,
}

impl Hash for Advisor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.package_name.hash(state);
    }
}

impl Eq for Advisor {}

impl PartialEq for Advisor {
    fn eq(&self, other: &Self) -> bool {
        self.package_name == other.package_name
    }
}
