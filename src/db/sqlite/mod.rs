mod sql;

use crate::prelude::*;

use rusqlite::{params, Connection};
use sql::*;
use std::sync::Mutex;
#[async_trait::async_trait]
impl DBConnection for Mutex<Connection> {
    async fn init(&self) -> Result<()> {
        let db = self.lock()?;
        db.execute(CREATE_TABLE, params![])?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let db = self.lock()?;
        db.execute(INSERT_USER, params![email, hash, is_admin])?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        let db = self.lock()?;
        db.execute(UPDATE_USER, params![user.id, user.email, user.password, user.is_admin])?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        let db = self.lock()?;
        db.execute(REMOVE_BY_ID, params![user_id])?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        let db = self.lock()?;
        db.execute(REMOVE_BY_EMAIL, params![email])?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let db = self.lock()?;
        let user = db
            .query_row(
                SELECT_BY_ID,
                params![user_id],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        password: row.get(2)?,
                        is_admin: row.get(3)?,
                    })
                },
            )
            .msg("User not found.")?;
        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let db = self.lock()?;
        let user = db
            .query_row(
                SELECT_BY_EMAIL,
                params![email],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        password: row.get(2)?,
                        is_admin: row.get(3)?,
                    })
                },
            )
            .msg("User not found.")?;
        Ok(user)
    }
}
