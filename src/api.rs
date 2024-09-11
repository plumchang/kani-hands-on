use crate::models::{Command, CommandResult};
use reqwest::Client;

pub async fn send_spawn(
    client: &Client,
    url: &str,
    name: &str,
    hue: f32,
) -> Result<CommandResult, reqwest::Error> {
    let spawn_command = Command::Spawn {
        name: name.to_string(),
        hue,
    };

    let result = client
        .post(url)
        .json(&spawn_command)
        .send()
        .await?
        .json::<CommandResult>()
        .await?;

    Ok(result)
}

pub async fn send_scan(
    client: &Client,
    url: &str,
    token: &str,
) -> Result<CommandResult, reqwest::Error> {
    let scan_command = Command::Scan {
        token: token.to_string(),
    };

    let result = client
        .post(url)
        .json(&scan_command)
        .send()
        .await?
        .json::<CommandResult>()
        .await?;

    Ok(result)
}

pub async fn send_turn(
    client: &Client,
    url: &str,
    token: &str,
    side: crate::models::Side,
) -> Result<CommandResult, reqwest::Error> {
    let turn_command = Command::Turn {
        token: token.to_string(),
        side,
    };

    let result = client
        .post(url)
        .json(&turn_command)
        .send()
        .await?
        .json::<CommandResult>()
        .await?;

    Ok(result)
}

pub async fn send_walk(
    client: &Client,
    url: &str,
    token: &str,
    side: crate::models::Side,
) -> Result<CommandResult, reqwest::Error> {
    let walk_command = Command::Walk {
        token: token.to_string(),
        side,
    };

    let result = client
        .post(url)
        .json(&walk_command)
        .send()
        .await?
        .json::<CommandResult>()
        .await?;

    Ok(result)
}

pub async fn send_paint(
    client: &Client,
    url: &str,
    token: &str,
) -> Result<CommandResult, reqwest::Error> {
    let paint_command = Command::Paint {
        token: token.to_string(),
    };

    let result = client
        .post(url)
        .json(&paint_command)
        .send()
        .await?
        .json::<CommandResult>()
        .await?;

    Ok(result)
}
