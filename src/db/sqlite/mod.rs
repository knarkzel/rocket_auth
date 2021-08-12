mod sql;

use crate::prelude::{Result, *};
use rusqlite::params;
use tokio::sync::Mutex;

#[rocket::async_trait]
impl DBConnection for Mutex<rusqlite::Connection> {
    async fn init(&self) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::CREATE_TABLE, [])?;
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::INSERT_USER, params![email, hash, is_admin])?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(
            sql::UPDATE_USER,
            params![user.id, user.email, user.password, user.is_admin],
        )?;
        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::REMOVE_BY_ID, params![user_id])?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::REMOVE_BY_EMAIL, params![email])?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let conn = self.lock().await;
        let user = conn.query_row(sql::SELECT_BY_ID, params![user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                is_admin: row.get(3)?,
            })
        })?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let conn = self.lock().await;
        let user = conn.query_row(sql::SELECT_BY_EMAIL, params![email], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                is_admin: row.get(3)?,
            })
        })?;
        Ok(user)
    }
}
