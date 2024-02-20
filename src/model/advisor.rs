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

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_advisor() {
        use super::*;
        use mongodb::bson::DateTime;
        use std::collections::HashSet;

        let advisor = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let mut set = HashSet::new();
        set.insert(advisor);

        assert_eq!(set.len(), 1);
    }

    #[tokio::test]
    async fn test_advisor_eq() {
        use super::*;
        use mongodb::bson::DateTime;

        let advisor1 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let advisor2 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        assert_eq!(advisor1, advisor2);
    }

    #[tokio::test]
    async fn test_advisor_hash() {
        use super::*;
        use mongodb::bson::DateTime;
        use std::collections::HashSet;

        let advisor1 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let advisor2 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let mut set = HashSet::new();
        set.insert(advisor1);
        set.insert(advisor2);

        assert_eq!(set.len(), 1);
    }

    #[tokio::test]
    async fn test_advisor_hash_diff() {
        use super::*;
        use mongodb::bson::DateTime;
        use std::collections::HashSet;

        let advisor1 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let advisor2 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test2".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let mut set = HashSet::new();
        set.insert(advisor1);
        set.insert(advisor2);

        assert_eq!(set.len(), 2);
    }

    #[tokio::test]
    async fn test_advisor_not_eq() {
        use super::*;
        use mongodb::bson::DateTime;

        let advisor1 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        let advisor2 = Advisor {
            _id: None,
            date: DateTime::now(),
            package_name: "Test".to_string(),
            url: "https://test.com".to_string(),
            year: 2021,
        };

        assert_ne!(advisor1, advisor2);
    }
}
