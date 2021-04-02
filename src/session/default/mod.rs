




use crate::prelude::*;
use super::AuthKey;
use chashmap::CHashMap;
use super::SessionManager;
#[rocket::async_trait]
impl SessionManager for CHashMap<i32, AuthKey> {

    // Unnecesary Result
    async fn insert(&self, id: i32, key: String) -> Result<()> {
        self.insert(id, key.into());
        Ok(())
    }
    // Unnecesary Result
    async fn remove(&self, id: i32) -> Result<()> {
        self.remove(&id);
        Ok(())
    }

    async fn get(&self, id: i32) -> Option<String> {
        let key = self.get(&id)?;
        Some(key.secret.clone())
    }

    fn clear_all(&self) -> Result<()> {
        self.clear();
        Ok(())
    }

    async fn insert_for(&self, id: i32, key: String, time: Duration) -> Result<()> {
        let key = AuthKey {
            expires: time.as_secs(),
            secret: key,
        };
        self.insert(id, key);
        Ok(())
    }
    fn clear_expired(&self) -> Result<()> {
        let time = now()? as u64;
        self.retain(| _, auth_key | {
            auth_key.expires > time
        });
        Ok(())
    }
}




use std::time::{SystemTime, UNIX_EPOCH};
fn now() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("Error computing SystemTime")?
        .as_secs())
}
