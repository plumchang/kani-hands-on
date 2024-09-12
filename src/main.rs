mod api;
mod models;

use std::sync::Arc;

use crossterm::{
    event::{self, KeyCode},
    terminal,
};
use models::{CommandResult, Side, WhatYouCanSee};
use reqwest::Client;
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?; // Rawモードを有効化

    let client = Client::new();
    let stop_flag = Arc::new(Mutex::new(false));

    // 引数判定
    let args: Vec<String> = std::env::args().collect();
    let kani_mode = args.contains(&"--kani".to_string());

    // キーボード入力待ちを別スレッドで行う
    let stop_flag_clone = Arc::clone(&stop_flag);
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(200)).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('q') {
                        println!("Quit");
                        let mut flag = stop_flag_clone.lock().await;
                        *flag = true;
                        break;
                    }
                }
            }
        }
    });

    let url = "https://kani-life-m6xtjae7da-an.a.run.app/api/command";

    // カニのスポーン
    let result = api::send_spawn(&client, url, "y-ume", 30.0).await?;
    // トークン取得
    let token = if let CommandResult::Spawn { token } = result {
        println!("Spawned successfully: token={}", token);
        token
    } else {
        return Ok(());
    };

    let mut current_turn_side = Side::Right;
    let mut current_walk_side = Side::Left;

    if kani_mode {
        // カニモード
        println!("Kani mode");

        let required_points = 20;
        let mut total_points = 0;

        // 必要なポイントを貯めるまでループ
        while total_points < required_points {
            // qが押されたら終了
            let stop_flag_value = stop_flag.lock().await;
            if *stop_flag_value {
                break;
            }
            drop(stop_flag_value);

            let scan_res = api::send_scan(&client, url, &token).await?;
            match scan_res {
                CommandResult::Scan { whatYouCanSee } => match whatYouCanSee {
                    WhatYouCanSee::Food => {
                        println!("Found food");

                        // ターン
                        api::send_turn(&client, url, &token, current_turn_side.clone()).await?;
                    }
                    WhatYouCanSee::Wall => {
                        println!("Found wall");

                        // ウォーク
                        let walk_res =
                            api::send_walk(&client, url, &token, current_walk_side.clone()).await?;

                        if let CommandResult::Walk {
                            success,
                            totalPoint,
                            ..
                        } = walk_res
                        {
                            if success {
                                total_points = totalPoint as usize;
                                println!("Walked successfully: total_point={}", totalPoint);
                            } else {
                                println!("Walk failed: total_point={}", totalPoint);

                                // 壁にぶつかったらターン
                                api::send_turn(&client, url, &token, current_turn_side.clone())
                                    .await?;
                            }
                        }
                    }
                    WhatYouCanSee::Crab => {
                        println!("Found crab");
                    }
                },
                _ => (),
            }
            sleep(Duration::from_millis(200)).await;
        }

        // 描画開始位置に移動
        println!("Preparing to draw 'カニ'.");
        // 左に9マス移動
        for _ in 0..9 {
            let walk_res = api::send_walk(&client, url, &token, Side::Left).await?;
            if let CommandResult::Walk { success, .. } = walk_res {
                if !success {
                    break;
                }
            }
            sleep(Duration::from_millis(200)).await;
        }
        // 右にターン
        api::send_turn(&client, url, &token, Side::Right).await?;
        // 左に10マス移動
        for _ in 0..10 {
            let walk_res = api::send_walk(&client, url, &token, Side::Left).await?;
            if let CommandResult::Walk { success, .. } = walk_res {
                if !success {
                    break;
                }
            }
            sleep(Duration::from_millis(200)).await;
        }
        // 左にターン
        api::send_turn(&client, url, &token, Side::Left).await?;

        // 「カニ」を5×5のドットで描く
        let ka_dots = [
            // カ
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 1],
            [0, 1, 0, 0, 1],
        ];
        let ni_dots = [
            // ニ
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        // 「カ」を描く
        for i in 0..5 {
            let is_even_row = i % 2 == 0;
            // 偶数列は右に、奇数列は左に移動
            if is_even_row {
                current_walk_side = Side::Right;
            } else {
                current_walk_side = Side::Left;
            }
            for j in 0..5 {
                // ペイント判定
                if is_even_row {
                    if ka_dots[i][j] == 1 {
                        api::send_paint(&client, url, &token).await?;
                    }
                } else {
                    if ka_dots[i][4 - j] == 1 {
                        api::send_paint(&client, url, &token).await?;
                    }
                }
                // 移動
                if j < 4 {
                    // 右に移動
                    api::send_walk(&client, url, &token, current_walk_side.clone()).await?;
                } else if i < 4 {
                    // 下に移動
                    api::send_turn(&client, url, &token, Side::Left).await?;
                    api::send_walk(&client, url, &token, Side::Left).await?;
                    api::send_turn(&client, url, &token, Side::Right).await?;
                }
                sleep(Duration::from_millis(200)).await;
            }
        }
        // 「ニ」を描く
        for i in 0..5 {
            let is_even_row = i % 2 == 0;
            // 偶数列は右に、奇数列は左に移動
            if is_even_row {
                current_walk_side = Side::Right;
            } else {
                current_walk_side = Side::Left;
            }
            for j in 0..5 {
                // ペイント判定
                if is_even_row {
                    if ni_dots[i][j] == 1 {
                        api::send_paint(&client, url, &token).await?;
                    }
                } else {
                    if ni_dots[i][4 - j] == 1 {
                        api::send_paint(&client, url, &token).await?;
                    }
                }
                // 移動
                if j < 4 {
                    // 左右に移動
                    api::send_walk(&client, url, &token, current_walk_side.clone()).await?;
                } else if i < 4 {
                    // 下に移動
                    api::send_turn(&client, url, &token, Side::Left).await?;
                    api::send_walk(&client, url, &token, Side::Left).await?;
                    api::send_turn(&client, url, &token, Side::Right).await?;
                }
                sleep(Duration::from_millis(200)).await;
            }
        }
    } else {
        // 通常モード
        println!("Normal mode");

        loop {
            // qが押されたら終了
            let stop_flag_value = stop_flag.lock().await;
            if *stop_flag_value {
                break;
            }
            drop(stop_flag_value);

            // Scan
            let scan_res = api::send_scan(&client, url, &token).await?;

            match scan_res {
                CommandResult::Scan { whatYouCanSee } => match whatYouCanSee {
                    WhatYouCanSee::Food => {
                        println!("Found food");

                        // ターン
                        api::send_turn(&client, url, &token, current_turn_side.clone()).await?;
                    }
                    WhatYouCanSee::Wall => {
                        println!("Found wall");

                        // ウォーク
                        let walk_res =
                            api::send_walk(&client, url, &token, current_walk_side.clone()).await?;

                        if let CommandResult::Walk {
                            success,
                            totalPoint,
                            ..
                        } = walk_res
                        {
                            if success {
                                println!("Walked successfully: total_point={}", totalPoint);

                                // ポイントが1.0以上ならペイント
                                if totalPoint >= 1.0 {
                                    let paint_res = api::send_paint(&client, url, &token).await?;

                                    if let CommandResult::Paint {
                                        success,
                                        totalPoint,
                                        ..
                                    } = paint_res
                                    {
                                        if success {
                                            println!(
                                                "Painted successfully: total_point={}",
                                                totalPoint
                                            );
                                        } else {
                                            println!("Paint failed: total_point={}", totalPoint);
                                        }
                                    }
                                } else {
                                    println!(
                                        "Not enough point to paint: total_point={}",
                                        totalPoint
                                    );
                                }
                            } else {
                                println!("Walk failed: total_point={}", totalPoint);

                                // 壁にぶつかったらターン
                                api::send_turn(&client, url, &token, current_turn_side.clone())
                                    .await?;
                            }
                        }
                    }
                    WhatYouCanSee::Crab => {
                        println!("Found crab");
                    }
                },
                _ => (),
            }

            sleep(Duration::from_millis(200)).await;
        }
    }

    // Rawモードを解除して通常の画面に戻す
    terminal::disable_raw_mode()?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_json_encoding() {
//         // r#" "#は生文字列リテラル
//         let expected = r#"{"type":"Ping"}"#;
//         let actual = serde_json::to_string(&crate::Command::Ping).unwrap();
//         assert_eq!(expected, actual);
//     }
// }
