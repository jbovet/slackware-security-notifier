use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::TwitterApi;

#[derive(Clone, Debug)]
pub struct TwitterClient {
    pub auth: Oauth1aToken,
}

impl TwitterClient {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Self {
            auth: Oauth1aToken::new(
                consumer_key,
                consumer_secret,
                access_token,
                access_token_secret,
            ),
        }
    }
    pub async fn post_tweet(&self, tweet: String) -> Result<(), Box<dyn std::error::Error>> {
        let auth = self.auth.clone();
        TwitterApi::new(auth)
            .post_tweet()
            .text(tweet)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_post_tweet() {
        use super::*;
        let client = TwitterClient::new(
            std::env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY must be set"),
            std::env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET must be set"),
            std::env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set"),
            std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
                .expect("TWITTER_ACCESS_TOKEN_SECRET must be set"),
        );
        let result = client.post_tweet("test".to_string()).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
