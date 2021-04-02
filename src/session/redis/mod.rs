use super::SessionManager;
use crate::prelude::*;


use redis::{AsyncCommands, Client};
// use redis::as
const YEAR_IN_SECS: usize = 365 * 60 * 60 * 24;
#[rocket::async_trait]
impl SessionManager for Client {
    async fn insert(&self, id: i32, key: String) -> Result<()> {
        let mut cnn = self.get_async_connection().await?;
        cnn.set_ex(id, key, YEAR_IN_SECS).await?;
        Ok(())
    }
    async fn insert_for(&self, id: i32, key: String, time: Duration) -> Result<()> {
        let mut cnn = self.get_async_connection().await?;
        cnn.set_ex(id, key, time.as_secs() as usize).await?;
        Ok(())
    }
    async fn remove(&self, id: i32) -> Result<()> {
        let mut cnn = self.get_async_connection().await?;
        cnn.del(id).await?;
        Ok(())
    }
    async fn get(&self, id: i32) -> Option<String> {
        let mut cnn = self.get_async_connection().await.ok()?;
        let key = cnn.get(id).await.ok()?;
        Some(key)
    }
    fn clear_all(&self) -> Result<()> {
        let mut cnn = self.get_connection()?;
        redis::Cmd::new().arg("FLUSHDB").execute(&mut cnn);
        Ok(())
    }
    fn clear_expired(&self) -> Result<()> {
        Ok(())
    }
}


