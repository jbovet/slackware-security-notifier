use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use mongodb::bson::{oid::ObjectId, DateTime};

use regex::Regex;

use crate::model::advisor::Advisor;

const URL: &str = "http://www.slackware.com/security/";
const REGX_ADVISORIES: &str =
    r"[0-9]{4}-[0-9]{2}-[0-9]{2} - <A\s([A-Za-z0-9]+(.*[A-Za-z0-9]+)+)\s\([^)]*\)";

pub async fn get_advisories_from_site_by_year(
    year: i32,
) -> Result<Vec<Advisor>, Box<dyn std::error::Error>> {
    let mut advisories: Vec<Advisor> = Vec::new();
    let body = reqwest::get(format!("{}list.php?l=slackware-security&y={}", URL, year))
        .await?
        .text()
        .await
        .unwrap();

    let re = Regex::new(REGX_ADVISORIES).unwrap();
    for cap in re.captures_iter(&body) {
        let temp = &cap[0];
        let temp = &temp.replace(" - ", "\n");
        let temp = &temp.replace("<A HREF=\"viewer.php?", "viewer.php?");
        let temp = &temp.replace("\">[slackware-security]", "\n");

        let elements = &temp.split('\n').map(|e| e.trim()).collect::<Vec<_>>();

        let naive_date = NaiveDate::parse_from_str(elements[0], "%Y-%m-%d").unwrap();
        // Add some default time to convert it into a NaiveDateTime
        let naive_datetime: NaiveDateTime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        // Add a timezone to the object to convert it into a DateTime<UTC>
        let datetime_utc = TimeZone::from_utc_datetime(&Utc, &naive_datetime);

        advisories.push(Advisor {
            _id: Some(ObjectId::new()),
            date: DateTime::from_chrono(datetime_utc),
            package_name: String::from(elements[2]),
            url: format!("{}{}", URL, elements[1]),
            year,
        });
    }

    Ok(advisories)
}

#[cfg(test)]
mod tests {
    use crate::model::advisor::Advisor;
    use crate::util::parser::get_advisories_from_site_by_year;
    use mongodb::bson;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_get_advisories_from_site_by_year() {
        let advisory_list = get_advisories_from_site_by_year(2023).await.unwrap();
        assert!(advisory_list.len() > 52);
    }

    #[tokio::test]
    async fn test_compare() {
        let av1 = [
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "sudo (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "git (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
        ];

        let av2 = [
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "sudo (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "git (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "xss (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
            Advisor {
                _id: None,
                date: bson::DateTime::now(),
                package_name: "ssl (SSA:2023-018-01)".to_string(),
                url: "".to_string(),
                year: 2023,
            },
        ];

        let hs_current = av1.iter().collect::<HashSet<_>>();
        let hs_latest = av2.iter().collect::<HashSet<_>>();
        let hs = &hs_current
            .symmetric_difference(&hs_latest)
            .collect::<Vec<_>>();

        assert!(hs.len() == 2);
    }
}
