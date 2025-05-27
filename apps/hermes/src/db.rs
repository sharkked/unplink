use mongodb::{
    Client,
    bson::{Document, doc},
    error::Result,
};

pub struct MongoConnection {
    client: Client,
}

impl MongoConnection {
    pub async fn open(uri: &String) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }

    pub async fn set_url(&self, shortcode: &String, url: &String) -> Result<()> {
        let shorturls = &self.client.database("unplink").collection("shorturls");
        let insert_result = shorturls
            .insert_one(doc! {
                "shortcode": shortcode,
                "url": url
            })
            .await?;
        println!("New document: {:?}", insert_result);
        Ok(())
    }

    pub async fn find_url(&self, shortcode: &String) -> Result<String> {
        let shorturls = &self.client.database("unplink").collection("shorturls");
        let find_result: Document = shorturls
            .find_one(doc! {
                "shortcode": shortcode
            })
            .await?
            .unwrap();
        println!("FOUND document ID: {}", find_result);
        Ok("YAY".into())
    }
}
