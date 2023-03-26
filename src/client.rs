use crate::config::Config;
use crate::error::Error;
use crate::utils;

use twitch_irc::login::RefreshingLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use utils::env::EnvStorage;

pub struct Client {
    pub config: Config,
    env: EnvStorage,
    irc: Option<TwitchIRCClient<SecureTCPTransport, RefreshingLoginCredentials<EnvStorage>>>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        let env = EnvStorage::from(&config);
        Self {
            config,
            env,
            irc: None,
        }
    }

    pub async fn start(
        &mut self,
        sender: tokio::sync::mpsc::Sender<ServerMessage>,
    ) -> Result<(), Error> {
        let config = build_irc_config(self.env.clone())?;
        let (mut incoming_messages, irc) = TwitchIRCClient::<
            SecureTCPTransport,
            RefreshingLoginCredentials<EnvStorage>,
        >::new(config);
        self.irc = Some(irc);

        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                sender.send(message).await.unwrap();
            }
        });

        let irc = self.irc.as_mut().unwrap();
        let channels = self.config.channels.as_ref().unwrap();
        for channel in channels.keys() {
            irc.join(channel.to_string()).unwrap();
        }
        join_handle.await.unwrap();

        Ok(())
    }
}

fn build_irc_config(
    env: EnvStorage,
) -> Result<ClientConfig<RefreshingLoginCredentials<EnvStorage>>, Error> {
    let username: Option<String> = env
        .get_env_opt("USERNAME")?
        .filter(|s: &String| !s.is_empty());

    let client_id = env.get_env("CLIENT_ID")?;
    let client_secret = env.get_env("CLIENT_SECRET")?;

    Ok(ClientConfig::new_simple(
        RefreshingLoginCredentials::init_with_username(username, client_id, client_secret, env),
    ))
}
