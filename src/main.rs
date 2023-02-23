
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// adds a task
    Add {
        /// what you're doing
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        duration: Option<u16>
        
    },
    /// shows tasks today including current
    Today {

    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Add {name, duration}) => {
            let mut elapsed : u16 = 25;
            if let Some(duration) = duration {
                elapsed = *duration;
            }
            
            println!("Adding a task {name} with duration {elapsed} minutes.");        
        },
        Some(Commands::Today {}) => {
            println!("Today you're doing great!");
        }
        None => {
            println!("No command found!");
        }
    }

    // Continued program logic goes here...
}