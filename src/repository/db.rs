use crate::model::advisor::Advisor;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub collection: Collection<Advisor>,
}

impl DB {
    pub async fn init() -> Self {
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();

        client_options.server_api =
            Some(ServerApi::builder().version(ServerApiVersion::V1).build());
        let client = Client::with_options(client_options).unwrap();

        let database = client.database("advisors");
        Self {
            collection: database.collection::<Advisor>("advisor"),
        }
    }

    pub async fn find_by_year(
        &self,
        year: i32,
    ) -> Result<Vec<Advisor>, Box<dyn std::error::Error>> {
        let mut advisors: Vec<Advisor> = Vec::new();
        let filter = doc! { "year": year };
        let mut cursor = self.collection.find(filter, None).await.unwrap();

        while let Some(advisor) = cursor.try_next().await.unwrap() {
            advisors.push(Advisor {
                _id: advisor._id,
                date: advisor.date,
                package_name: advisor.package_name,
                url: advisor.url,
                year: advisor.year,
            });
        }
        Ok(advisors)
    }

    pub async fn insert(
        &self,
        advisor: Advisor,
    ) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        let result = self
            .collection
            .insert_one(&advisor, None)
            .await
            .expect("failed to insert");
        Ok(result)
    }
    
}
