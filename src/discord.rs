use colored::*;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;

pub fn send_notification(message: &str) {
    // .envファイルを読み込み
    dotenv().ok();

    // Discord WebhookのURLを環境変数から取得
    let webhook_url = match env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("{}", "Discord webhook URLが設定されていません".yellow());
            return;
        }
    };

    let client = Client::new();
    let payload = json!({
        "content": message,
    });

    match client.post(&webhook_url).json(&payload).send() {
        Ok(_) => println!("{}", "Discordへの通知を送信しました".green()),
        Err(e) => println!(
            "{}",
            format!("Discordへの通知の送信に失敗しました: {}", e).red()
        ),
    }
}

pub fn create_deploy_message(commit_message: &str, elapsed: std::time::Duration) -> String {
    format!(
        "🚀 デプロイが完了しました!\n\
         📝 コミットメッセージ: {}\n\
         ⏱️ 所要時間: {:?}",
        commit_message, elapsed
    )
}
