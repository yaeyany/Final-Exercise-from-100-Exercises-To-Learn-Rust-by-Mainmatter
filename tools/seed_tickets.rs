use rand::prelude::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut handles = Vec::new();

    let client = reqwest::Client::new();

    for _ in 0..100 {
        let client = client.clone();

        let handle = tokio::spawn(async move {

            let title = random_title();
            let description = random_description();
            let ticket = Ticket{title,description};

            client
                .post("http://localhost:3000/ticket/create")
                .json(&ticket)
                .send()
                .await
        });

        handles.push(handle);
    }

    for handle in handles {
        let response = handle.await??;
        println!("{}", response.status());
    }
    
    Ok(())
}

fn random_title() -> String {
    let titles = [
        "Fix login bug",
        "Update documentation",
        "Improve dashboard",
        "Fix broken navigation",
        "Investigate slow query",
        "Add dark mode",
    ];

    titles
        .choose(&mut rand::rng())
        .unwrap()
        .to_string()
}

fn random_description() -> String {
    let descriptions = [
        "The problem occurs when the user submits the form.",
        "This needs to be investigated and fixed.",
        "The issue appears intermittently.",
        "This would improve the user experience.",
        "The current implementation is difficult to maintain.",
    ];

    descriptions
        .choose(&mut rand::rng())
        .unwrap()
        .to_string()
}