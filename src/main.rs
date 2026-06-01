use serde_json::{json, Value};
use reqwest;
use std::io;
use colored::*;
use dotenvy::dotenv;
use std::env;



#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let model = env::var("LLM_MODEL").expect("input a model in .env");
    let api_key = env::var("API_KEY").expect("Input openroute key in .env file");
    let client = reqwest::Client::new();
    let mut history = Vec::new();

    loop{

        // prompt inputgit push -u origin main

        println!("--Enter Your Prompt--");
        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).expect("expected string");
        println!("User asked : {}\n", prompt.red());
        
        // pushing prompt to history
        history.push(json!({"role": "user", "content": prompt}));

        //json request
        let payload = json!({
        "model": model,
        "messages": &history
        });
        
        let res = client.post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await;

        match res{
            Ok(response) => {
                let text = response.text().await.unwrap();
                let v: Value = serde_json::from_str(&text).expect("REASON");
                let content = v["choices"][0]["message"]["content"].as_str().unwrap();
                println!("Response: {}\n", content.green());

                // push history
                history.push(json!({"role": "assistant", "content": content,}));

            },
            Err(e) => println!("{}",e),
        }
    }
}