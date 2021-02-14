use std::collections::HashMap;


async fn httpget() -> Result<HashMap<String, String>, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}

async fn httppost() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    Ok(())

}


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    if let Ok(res) = httpget().await {
        println!("{:#?}", res)
    }

    if let Ok(res) = httppost().await {
        println!("{:#?}", res)
    }
}
