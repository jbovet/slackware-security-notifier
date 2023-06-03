use std::collections::HashSet;
use std::process::exit;

use chrono::Datelike;
use util::parser::get_advisories_from_site_by_year;

use crate::model::advisor::Advisor;
use crate::repository::db::DB;

mod model;
mod repository;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //initdb
    let db = DB::init().await;
    let year = chrono::Utc::now().year();
    let latest_advisory_list = match get_advisories_from_site_by_year(year).await {
        Ok(list) => list,
        Err(_) => exit(1),
    };

    //get from db
    let current_advisory_list = db.find_by_year(year).await.unwrap();
    if latest_advisory_list.len() <= current_advisory_list.len() {
        println!("No new advisories found");
        exit(0);
    }

    //get difference
    let hs_current = current_advisory_list.iter().collect::<HashSet<_>>();
    let hs_latest = latest_advisory_list.iter().collect::<HashSet<_>>();

    let differences_elements = &hs_current
        .symmetric_difference(&hs_latest)
        .collect::<Vec<_>>();

    //add advisories
    for advisor in differences_elements.iter() {
        println!("adding package {:?}", &advisor.package_name);
        db.insert(Advisor {
            _id: advisor._id,
            date: advisor.date,
            package_name: advisor.package_name.to_owned(),
            url: advisor.url.to_owned(),
            year: advisor.year,
        })
        .await
        .unwrap();
    }
    Ok(())
}
