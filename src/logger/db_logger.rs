use crate::config::Config;
use crate::entities::chat::ChatMessage;
use crate::error::Error;
use sqlx::PgPool;

pub struct DbLogger {
    pool: PgPool,
    table_name: String,
}

impl DbLogger {
    pub fn new(config: &Config, pool: PgPool) -> Self {
        Self {
            pool,
            table_name: config.db_table.clone().unwrap(),
        }
    }

    pub async fn create_log(&mut self, message: &ChatMessage) -> Result<(), Error> {
        let query = format!(
            "INSERT INTO {} (username, message, channel, sent_at) VALUES ($1, $2, $3, $4)",
            self.table_name
        );
        sqlx::query(&query)
            .bind(&message.username)
            .bind(&message.message)
            .bind(&message.channel)
            .bind(message.sent_at)
            .execute(&self.pool)
            .await
            .unwrap();

        Ok(())
    }

    pub async fn create_log_batch(&mut self, messages: &[ChatMessage]) -> Result<(), Error> {
        if messages.is_empty() {
            return Ok(());
        }

        let query = format!(
            "INSERT INTO {} (username, message, channel, sent_at) VALUES ($1, $2, $3, $4)",
            self.table_name
        );
        let mut transaction = self.pool.begin().await.unwrap();
        for message in messages {
            sqlx::query(&query)
                .bind(&message.username)
                .bind(&message.message)
                .bind(&message.channel)
                .bind(message.sent_at)
                .execute(&mut transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();
        Ok(())
    }
}
