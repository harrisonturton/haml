use std::path::PathBuf;

use clap::command;
use clap::Parser;

use inc::db::Database;
use inc::diagnostics::Diagnostics;
use inc::queries::{parse_file, read_file, Path};

#[derive(Parser)]
#[command(bin_name = "haml", author = "Harrison Turton", version)]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    import_root: Option<String>,
}

pub fn main() {
    let Args { path, .. } = Args::parse();

    let db = Database::default();
    let path = Path::new(&db, path);

    let file = match read_file(&db, path) {
        Some(file) => file,
        None => {
            println!("Could not read file");
            return;
        }
    };

    let ast = match parse_file(&db, file) {
        Some(ast) => ast,
        None => {
            let parse_file_errs = parse_file::accumulated::<Diagnostics>(&db, file);
            for err in parse_file_errs {
                println!("{}", err.to_user_message(&db, &file));
            }
            return;
        }
    };

    println!("{ast:?}");
}
