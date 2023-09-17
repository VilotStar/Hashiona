use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use clap::{Parser, Subcommand};
use bson::Document;
use hashiona::folder_hash_sha512;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(short = 'q', long = "folder")]
    folder_path: String,
    #[arg(short = 'f', long = "hash-file")]
    hash_file_path: String
}

#[derive(Subcommand, Debug)]
enum Command {
    Generate,
    Verify
}

fn main() -> () {
    let args = Args::parse();

    match args.command {
        Command::Generate => {
            let hash_map = folder_hash_sha512(&args.folder_path).unwrap();
            let mut doc = Document::new();

            for (file, hash) in hash_map {
                doc.insert(file, hash);
            }

            let mut file = File::create(&args.hash_file_path).unwrap();

            let bson_bytes = bson::to_vec(&doc).unwrap();
            file.write_all(&bson_bytes).unwrap();
        }
        Command::Verify => {
            let file = File::open(&args.hash_file_path).unwrap();
            let doc = Document::from_reader(file).unwrap();
            let mut old_hash_map = HashMap::new();

            for (key, value) in doc {
                old_hash_map.insert(key, value);
            }

            let cur_hash_map = folder_hash_sha512(&args.folder_path).unwrap();

            let mut changed_files = 0;
            let mut new_files = 0;

            for (file, hash) in cur_hash_map {
                match old_hash_map.get(&file) {
                    Some(old_hash_opt) => {
                        let old_hash = old_hash_opt.as_str().unwrap();
                        if old_hash != hash {
                            println!("File {file} has changed content");
                            changed_files = changed_files + 1;
                        }
                    }
                    None => {
                        println!("New file {file}");
                        new_files = new_files + 1;
                    }
                }
            }

            if changed_files == 0 && new_files == 0 {
                println!("== No changed or new files!");
                return;
            }

            println!("== {changed_files} Changed files");
            println!("== {new_files} New files");
        }
    }

    return;
}
