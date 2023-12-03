use clap::{Parser, Subcommand};
use dot::{create_package, DotError};

#[derive(Parser)]
#[command(name = "Dot")]
#[command(author = "Jack <htnjack@proton.me>")]
#[command(version = "1.0")]
#[command(about = "Kinda like stow", long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New { package: String },
    List,
    Revert { package: String },
}

fn main() -> Result<(), DotError> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose is on");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::New { package }) => {
            create_package(package)?;
            return Ok(());
        }
        Some(Commands::List) => {}
        Some(Commands::Revert { .. }) => {}
        None => {}
    };
    Ok(())
}
