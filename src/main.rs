use std::io::{self, Write, stdout};

use clap::{Parser, Subcommand};
use keyring::Entry;
use pedidosya_courier_rs::{
    PedidosYaBlocking,
    models::{Urls, WebhookConfiguration, WebhooksConfigModel},
};

fn main() -> Result<(), String> {
    let arg = Cli::parse().commands;

    match arg {
        Command::Login => {
            let key = rpassword::read_password().unwrap();
            let entry = Entry::new("pedidos-cli", "default-user").unwrap();
            entry
                .set_password(&key)
                .map_err(|_| "Couldn't set api key data for")?;
        }
        Command::List => {
            let entry = Entry::new("pedidos-cli", "default-user").unwrap();

            let pass = entry
                .get_password()
                .map_err(|_| "There is no saved api key")?;

            let webhooks = PedidosYaBlocking::blocking_webhook_get_webhooks_configuration(pass)
                .map_err(|_| "Request Error")?;

            webhooks.webhooks_configuration.iter().for_each(|hook| {
                println!("{:#?}", hook);
            });
        }
        Command::Set => {
            let entry = Entry::new("pedidos-cli", "default-user").unwrap();

            let pass = entry
                .get_password()
                .map_err(|_| "There is no saved api key")?;

            print!("Test/Develop webhook? (Y/n): ");
            stdout().flush().unwrap();
            let mut is_test = String::new();
            io::stdin().read_line(&mut is_test).unwrap();
            let is_test = is_test.trim();
            let is_test = is_test.is_empty() || is_test.eq("Y");

            print!("Webhook url: ");
            stdout().flush().unwrap();
            let mut url = String::new();
            io::stdin().read_line(&mut url).unwrap();
            let url = url.trim().to_string();

            print!("Authorization key that will be set as `x-api-key` header: ");
            stdout().flush().unwrap();
            let mut authorization_key = String::new();
            io::stdin().read_line(&mut authorization_key).unwrap();
            let authorization_key = Some(authorization_key.trim()).and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            });

            let webhook_config = WebhookConfiguration {
                is_test: Some(is_test),
                notification_type:
                    pedidosya_courier_rs::models::webhook_configuration::NotificationType::default(),
                topic: pedidosya_courier_rs::models::webhook_configuration::Topic::default(),
                urls: vec![Urls {
                    url,
                    authorization_key,
                }],
            };
            let web_config_model = WebhooksConfigModel {
                webhooks_configuration: Some(vec![webhook_config]),
            };

            let webhooks = PedidosYaBlocking::blocking_webhook_set_webhooks_configuration(
                pass,
                web_config_model,
            )
            .map_err(|e| match e {
                pedidosya_courier_rs::models::Error::Reqwest(error) => error.to_string(),
                pedidosya_courier_rs::models::Error::Serde(error) => error.to_string(),
                pedidosya_courier_rs::models::Error::Io(error) => error.to_string(),
                pedidosya_courier_rs::models::Error::ResponseError(_) => {
                    "Another error".to_string()
                }
            })?;

            webhooks.webhooks_configuration.iter().for_each(|hook| {
                println!("{:#?}", hook);
            });
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(subcommand)]
    commands: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Authenticate via providing the api key
    Login,
    List,
    Set,
}
