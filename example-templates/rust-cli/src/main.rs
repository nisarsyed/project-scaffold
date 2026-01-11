use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "{{project_name}}")]
#[command(about = "{{description}}")]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }

    Ok(())
}
