use clap::{arg, command, Command};
use obws::Client;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            Command::new("savereplay")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("savereplay") => {
            let client = Client::connect("localhost", 4455, Some("")).await.unwrap();
            let _ = client.replay_buffer().save().await.unwrap();
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    Ok(())
}