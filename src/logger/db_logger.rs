use crate::config::Config;
use crate::entities::chat::ChatMessage;
use crate::error::Error;
use tokio_postgres::types::ToSql;
use tokio_postgres::{connect, types::Type, Client, NoTls, Statement};

pub struct DbLogger {
    client: Client,
    table_name: String,
    statement_create_log: Option<Statement>,
}

impl DbLogger {
    pub async fn new(client: Client, table_name: String) -> Self {
        let mut this = Self {
            client,
            table_name,
            statement_create_log: None,
        };

        this.prepare_create_log()
            .await
            .expect("Could not prepare create log statement");
        this
    }

    async fn prepare_create_log(&mut self) -> Result<(), Error> {
        let sql = format!(
            "\
            INSERT INTO {} (channel, username, message, sent_at) \
            VALUES ($1, $2, $3, $4) \
        ",
            self.table_name
        );
        let types = [Type::TEXT, Type::TEXT, Type::TEXT, Type::TIMESTAMPTZ];

        let statement = self.client.prepare_typed(&sql, &types).await?;
        self.statement_create_log = Some(statement);

        Ok(())
    }

    pub async fn create_log(&mut self, message: &ChatMessage) -> Result<u64, Error> {
        if self.statement_create_log.is_none() {
            self.prepare_create_log().await?;
        }

        let statement = self.statement_create_log.as_ref().unwrap();
        let params: &[&(dyn ToSql + Sync)] = &[
            &message.channel,
            &message.username,
            &message.message,
            &message.sent_at,
        ];

        let rows = self.client.execute(statement, params).await?;
        Ok(rows)
    }

    pub async fn try_from(config: &Config) -> Result<Self, Error> {
        let (client, connection) = connect(config.db_url.clone().unwrap().as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let table_name = config.db_table.clone().unwrap();

        Ok(Self::new(client, table_name).await)
    }
}
