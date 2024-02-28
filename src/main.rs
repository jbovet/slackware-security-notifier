use std::process::exit;

use chrono::Datelike;
use util::parser::get_advisories_from_site_by_year;

use crate::model::advisor::Advisor;
use crate::repository::db::DB;
use crate::social::twitter::TwitterClient;
use log::{error, info};

mod model;
mod repository;
mod social;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting slackware security advisory bot");

    let db = DB::init().await;
    let twitter_client = TwitterClient::new(
        std::env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY must be set"),
        std::env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET must be set"),
        std::env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set"),
        std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .expect("TWITTER_ACCESS_TOKEN_SECRET must be set"),
    );

    info!("Getting latest advisories");
    let year = chrono::Utc::now().year();
    let mut latest_advisory_list = match get_advisories_from_site_by_year(year).await {
        Ok(list) => list,
        Err(_) => exit(1),
    };

    info!("Getting current advisories from db");
    let current_advisory_list = db.find_by_year(year).await.unwrap();
    if latest_advisory_list.len() <= current_advisory_list.len() {
        info!("No new advisories found");
        exit(0);
    }

    //remove commons elements
    latest_advisory_list.retain(|x| !current_advisory_list.contains(x));

    //add advisories
    for advisor in latest_advisory_list.iter().rev() {
        match db
            .insert(Advisor {
                _id: advisor._id,
                date: advisor.date,
                package_name: advisor.package_name.to_owned(),
                url: advisor.url.to_owned(),
                year: advisor.year,
            })
            .await
        {
            Ok(_) => {
                info!("Added advisory: {:?}", advisor.package_name);
                match twitter_client
                    .post_tweet(format!(
                        "#slackware security advisory for {} {}",
                        advisor.package_name, advisor.url
                    ))
                    .await
                {
                    Ok(_) => {
                        info!("Tweeted advisory: {:?}", advisor.package_name);
                    }
                    Err(e) => {
                        error!("Error tweeting advisory: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error adding advisory: {:?}", e);
            }
        }
    }
    Ok(())
}
