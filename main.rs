use reqwest;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio;

async fn is_valid_url(url: &str) -> bool {
    match reqwest::Client::new().head(url).send().await {
        Ok(response) => response.status().as_u16() < 400,
        Err(_) => false,
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: program_name <URL> <WORDLIST_FILE> [-fl FILTER_CONTENT_LENGTH] [-sc FILTER_STATUS_CODES]");
        return;
    }

    let url = &args[1];
    let wordlist_file = &args[2];

    if !is_valid_url(url).await {
        println!("Invalid URL or unreachable host.");
        return;
    }

    let mut filter_content_length: Option<u64> = None;
    let filter_status_codes: Arc<Mutex<HashSet<u16>>> = Arc::new(Mutex::new(HashSet::new()));

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "-fl" => {
                filter_content_length = args.get(i + 1).and_then(|arg| arg.parse().ok());
                i += 2;
            }
            "-sc" => {
                if let Some(status_codes) = args.get(i + 1) {
                    for code in status_codes.split(',') {
                        if let Ok(status_code) = code.parse::<u16>() {
                            filter_status_codes.lock().unwrap().insert(status_code);
                        }
                    }
                }
                i += 2;
            }
            _ => {
                println!("Invalid flag: {}", args[i]);
                return;
            }
        }
    }

    if let Ok(lines) = fs::read_to_string(wordlist_file) {
        let client = reqwest::Client::new();
        let mut tasks = vec![];

        for line in lines.lines() {
            let target_url = format!("{}{}", url, line);
            let client_clone = client.clone();
            let filter_status_codes_clone = filter_status_codes.clone(); // Clone the Arc for each task
            let task = tokio::spawn(async move {
                if let Ok(response) = client_clone.get(&target_url).send().await {
                    let status_code = response.status().as_u16();
                    if let Some(length) = response.content_length() {
                        let filter_status_codes = filter_status_codes_clone.lock().unwrap();
                        if filter_content_length.map_or(true, |filter| length != filter) &&
                            !filter_status_codes.contains(&status_code) {
                            println!("URL: {}", target_url);
                            println!("Response status code: {}", status_code);
                            println!("Response content length: {}", length);
                            println!("---------------------------------------");
                        }
                    }
                }
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }
    } else {
        println!("Failed to read wordlist file.");
    }
}
