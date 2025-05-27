extern crate redis;
use redis::{Commands, FromRedisValue, RedisResult, ToRedisArgs};

pub struct Cache {
    client: redis::Client,
}

impl Cache {
    pub fn open(url: &String) -> RedisResult<Self> {
        let client = redis::Client::open(url.clone())?;
        Ok(Cache { client })
    }

    pub fn set<T: ToRedisArgs>(&self, key: &String, value: &T) -> redis::RedisResult<()> {
        let mut con = self.client.get_connection()?;
        con.set(key, value)
    }

    pub fn get<T: FromRedisValue>(&self, path: &String) -> redis::RedisResult<T> {
        let mut con = self.client.get_connection()?;
        con.get(path)
    }
}
