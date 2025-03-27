use clap::{Parser, Subcommand};

mod constellation;

#[derive(Parser)]
#[command(name = "gmat")]
#[command(about = "gmat cli for creating constellations", long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Constellation {
        #[arg(short = 'n', long = "name", required = true)]
        script_name: String,
        #[arg(short, long)]
        i: i32,
        #[arg(short, long)]
        t: i32,
        #[arg(short, long)]
        p: i32,
        #[arg(short, long)]
        f: i32,
        #[arg(short, long)]
        ecc: f64,
        #[arg(short, long)]
        sma: f64,
        #[arg(short, long)]
        aop: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Constellation {script_name, i, t, p, f, ecc, sma, aop} => {
            match constellation::create_constellation(i, t, p, f, ecc, sma, aop, script_name.clone()) {
                Ok(_) => {
                    println!("constellation script saved at {}", script_name);
                }
                Err(e) => {
                    eprintln!("error creating constellation: {}", e);
                }
            }
        }
    }
}
