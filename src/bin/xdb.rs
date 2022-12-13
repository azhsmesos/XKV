use std::env::current_dir;
use clap::{Parser, Subcommand};
use xlkv::kv;


#[derive(Debug, Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Set {key: String, val: String},

    #[clap(arg_required_else_help = true)]
    Get { key: String },

    #[clap(arg_required_else_help = true)]
    #[clap(name = "rm")]
    Remove { key: String },
}

fn main() {
    let args = Cli::parse();


    let mut store = kv::kv_store::KVStore::open(current_dir().unwrap().as_path()).unwrap();

    match args.command {
        Command::Get { key } => {
            println!("key: {}", key);
        },
        Command::Set { key, val } => {
            if let Err(err) = store.set(key, val) {
                eprintln!("{:?}", err);
            }
        }
        _ => unimplemented!("Unsupported syntax"),
    }
}