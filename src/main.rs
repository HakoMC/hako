use colored::*;
use std::env;
use std::process::Command;
use std::time::Instant;

mod discord;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が不足している場合はヘルプを表示
    if args.len() < 2 {
        print_help();
        return;
    }

    // 第1引数に基づいてコマンドを実行
    match args[1].as_str() {
        "deploy" | "d" => run_deploy_command(&args),
        "algolia" | "a" => run_algolia_command(),
        "help" | "-h" | "--help" => print_help(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  hako d, deploy - HexoプロジェクトをWranglerでデプロイします");
    println!("  hako a, algolia - Algoliaインデックスを更新します");
    println!("  hako help  - ヘルプを表示します");
}

fn run_deploy_command(args: &[String]) {
    if args.len() < 3 {
        println!("{}", "エラー!: コミットメッセージが必要です".red());
        println!("使い方: hako commit \"コミットメッセージ\"");
        return;
    }

    let start = Instant::now();

    println!("変更をステージングしています...");
    let git_add_status = Command::new("git").args(["add", "."]).status();
    match git_add_status {
        Ok(status) => {
            if !status.success() {
                println!(
                    "{}",
                    format!("ステージングに失敗しました! ステータス: {}", status).red()
                );
                return;
            } else {
                println!("{}", "ステージングしました!".green());
            }
        }
        Err(e) => {
            println!(
                "{}",
                format!("ステージングの実行に失敗しました: {}", e).red()
            );
            return;
        }
    }

    println!("コミットしています...");
    let git_status = Command::new("git")
        .args(["commit", "-m", &args[2]])
        .status();
    match git_status {
        Ok(status) => {
            if status.success() {
                println!("{}", "コミットしました!".green());
            } else {
                println!(
                    "{}",
                    format!("コミットに失敗しました! ステータス: {}", status).red()
                );
            }
        }
        Err(e) => {
            println!("{}", format!("コミットの実行に失敗しました: {}", e).red());
            return;
        }
    }

    println!("プッシュしています...");
    let push_status = Command::new("git")
        .args(["push", "origin", "main"])
        .status();
    match push_status {
        Ok(status) => {
            if status.success() {
                println!("{}", "プッシュしました!".green());
            } else {
                println!(
                    "{}",
                    format!("プッシュに失敗しました! ステータス: {}", status).red()
                );
            }
        }
        Err(e) => {
            println!("{}", format!("プッシュの実行に失敗しました: {}", e).red());
            return;
        }
    }

    println!("ビルドしています...");
    let hexo_status = Command::new("hexo").arg("g").status();
    match hexo_status {
        Ok(status) => {
            if status.success() {
                println!("{}", "ビルドしました!".green());
            } else {
                println!(
                    "{}",
                    format!("ビルドに失敗しました! ステータス: {}", status).red()
                );
            }
        }
        Err(e) => {
            println!("{}", format!("ビルドの実行に失敗しました: {}", e).red());
            return;
        }
    }

    println!("デプロイしています...");
    let wrangler_status = Command::new("wrangler")
        .args(["pages", "deploy", "public"])
        .status();
    match wrangler_status {
        Ok(status) => {
            if status.success() {
                let elapsed = start.elapsed();
                let success_message = format!("デプロイしました! 所要時間: {:?}", elapsed);
                println!("{}", success_message.green());

                // Discordへ通知
                let discord_message = discord::create_deploy_message(&args[2], elapsed);
                discord::send_notification(&discord_message);
            } else {
                println!(
                    "{}",
                    format!("デプロイに失敗しました! ステータス: {}", status).red()
                );
            }
        }
        Err(e) => println!("{}", format!("デプロイの実行に失敗しました: {}", e).red()),
    }
}

fn run_algolia_command() {
    println!("Algoliaインデックスを更新しています...");

    // git commit -m "メッセージ" を実行
    let algolia_status = Command::new("hexo").arg("algolia").status();

    match algolia_status {
        Ok(status) => {
            if status.success() {
                println!("{}", "Algoliaインデックスを更新しました!".green());
            } else {
                println!(
                    "{}",
                    format!(
                        "Algoliaインデックスの更新に失敗しました! ステータス: {}",
                        status
                    )
                    .red()
                );
            }
        }
        Err(e) => {
            println!(
                "{}",
                format!("hexo algoliaの実行に失敗しました: {}", e).red()
            );
        }
    }
}
