
#[tokio::main]
async fn main() {
    let url = "https://kani-life-m6xtjae7da-an.a.run.app/api/command";

    let result = reqwest::Client::new()
        .post(url)
        .json(&Command::Spawn { name: "y-ume".to_string(), hue: 30.0 })
        .send()
        .await
        .unwrap()
        .json::<CommandResult>()
        .await
        .unwrap();

    println!("{:?}", result);

    // if let Ok(response) = result {
    //     println!("{:?}", response.text().await.unwrap());
    // } else {
    //     println!("Error: {:?}", result);
    // }
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
enum Command {
    Ping,
    Spawn {
        name: String,
        hue: f32,
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
enum CommandResult {
    Pong,
    Spawn {
        token: String,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_encoding() {
        // r#" "#は生文字列リテラル
        let expected = r#"{"type":"Ping"}"#;
        let actual = serde_json::to_string(&crate::Command::Ping).unwrap();
        assert_eq!(expected, actual);
    }
}