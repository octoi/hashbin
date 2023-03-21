use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        process::exit(0);
    }

    let option = &args[1];

    if option == "--help" {
        print_help();
        process::exit(0);
    }

    if option == "-h" {
        if args.len() < 3 {
            println!("Error: usage: hashbin -h <file>\n");
            print_help();
            process::exit(0);
        }

        for file in &args[2..] {
            let hash = hash_file(file);
            println!("{} : {}", file, hash);
        }
    } else if option == "-c" {
        if args.len() < 4 {
            println!("Error: usage: hashbin -c <file> <hash>\n");
            print_help();
            process::exit(0);
        }

        let file = &args[2];
        let hash = &args[3];

        let file_hash = hash_file(file);

        if hash.to_string() == file_hash {
            println!("Hash matched");
        } else {
            println!("Hash doesn't match");
            println!("file hash: {}", file_hash);
        }
    } else {
        println!("Error: Invalid option \n");
        print_help();
    }
}

fn print_help() {
    println!("Hashbin cli 0.1.0\n");
    println!("usage: hashbin <option> <file>");
    println!("-- options");
    println!("-h : hash given file");
    println!("-c : compare given file with hash");
}

fn hash_file(file: &str) -> String {
    match fs::read(file) {
        Ok(bytes) => {
            let hash = sha256::digest(&*bytes); 
            return hash;
        },
        Err(err) => {
            println!("Error: {}", err);
            process::exit(0);
        }
    };
}

