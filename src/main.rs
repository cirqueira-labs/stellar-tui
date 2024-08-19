mod server;
mod ui;

use reqwest::Client;
use server::get_public_key;
use std::io::{self, Write};
use std::time::Duration;
use stellar_rs::accounts::{accounts_request, prelude::*};
use stellar_rs::horizon_client::*;
use tokio::time::sleep;
use webbrowser::{self};

const AUTH_URL: &str = "http://localhost:50009";
enum StellarNetwork {
    Mainnet,
    Testnet,
    Futurenet,
}

impl StellarNetwork {
    fn rpc_url(&self) -> &str {
        match self {
            StellarNetwork::Mainnet => "https://horizon.stellar.org",
            StellarNetwork::Testnet => "https://horizon-testnet.stellar.org",
            StellarNetwork::Futurenet => "https://horizon-futurenet.stellar.org",
        }
    }
}

#[tokio::main]
async fn main() {
    let mut pub_key: String = String::new();
    ui::clear_terminal();
    ui::display_ascii_art();

    loop {
        println!("Please, choose a network:");
        ui::choose_network();

        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading");

        let input = input.trim();

        match input {
            "1" => {
                println!("Connecting to Mainnet...");
                break;
            }
            "2" => {
                println!("Connecting to Testnet...");
                break;
            }
            "3" => {
                println!("Connecting to Futurenet...");
                break;
            }
            "X" => {
                println!("Exiting...");
                return;
            }
            _ => {
                println!("Invalid option. Please choose again.");
            }
        }
    }

    pub_key = connect_wallet().await;
    loop {
        println!("Please, choose an option:");
        ui::show_menu();

        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading");

        let input = input.trim();

        if input == "X" {
            println!("Exiting...");
            return;
        }

        let result = execute_menu(&input, &pub_key).await;
    }
}

async fn execute_menu(input: &str, pub_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    match input {
        "1" => get_health(input).await,
        "2" => account_list(pub_key, input).await,
        "X" => Ok(()),
        _ => {
            println!("Choose again...");
            Ok(())
        }
    }
}

async fn get_health(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let horizon_url = match input {
        "1" => "https://horizon.stellar.org",
        "2" => "https://horizon-testnet.stellar.org",
        "3" => "https://horizon-futurenet.stellar.org",
        _ => return Err("Invalid network choice".into()),
    };
    let url = format!("{}/", horizon_url);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("Metrics: {}", json);
        return Ok(());
    }
    Ok(())
}

async fn account_list(
    pub_key: &str,
    choosen_input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("account_list");
    let mut vec_account_list: Vec<String> = Vec::new();

    let horizon_url = match choosen_input {
        "1" => "https://horizon.stellar.org",
        "2" => "https://horizon-testnet.stellar.org",
        "3" => "https://horizon-futurenet.stellar.org",
        _ => return Err("Invalid network choice".into()),
    };

    let horizon_client =
        HorizonClient::new(horizon_url.to_string()).expect("Failed to create Horizon Client");

    let request = AccountsRequest::new().set_signer_filter(pub_key).unwrap();

    let response: Result<AccountsResponse, String> =
        horizon_client.get_account_list(&request).await;

    for record in response?.embedded().records() {
        vec_account_list.push(record.account_id().to_string());
        vec_account_list.push(record.last_modified_time().to_string());
        vec_account_list.push(record.balances()[0].balance().to_string());
        println!("{:?}", vec_account_list);
    }

    Ok(())
}

fn open_browser(url: &str) {
    webbrowser::open(url);
}

async fn connect_wallet() -> String {
    let mut pub_key: String = String::new();
    let server_handle = tokio::spawn(async move {
        let _server_task = tokio::spawn(server::start_server());
    });
    sleep(Duration::from_secs(2)).await;
    open_browser(AUTH_URL);
    while server::get_public_key().is_empty() {}
    pub_key.push_str(get_public_key());
    println!(
        "Public key: {}...{}",
        &pub_key[0..4],
        &pub_key[&pub_key.len() - 4..],
    );
    server_handle.await.unwrap();
    pub_key
}
