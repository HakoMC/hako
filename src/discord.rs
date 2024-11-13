use colored::*;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;

const CLOUDFLARE_ORANGE: u32 = 0xF6821F;

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
        "embeds": [{
            "title": "🚀 デプロイ完了",
            "description": message,
            "url": "https://xn--n8js9a0a.xn--q9jyb4c/",
            "color": CLOUDFLARE_ORANGE,
            "footer": {
                "text": "Powered by Cloudflare Pages"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }]
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
        "📝 **コミットメッセージ**\n{}\n\n⏱️ **所要時間**\n{:?}",
        commit_message, elapsed
    )
}
