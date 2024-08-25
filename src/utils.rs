use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io::Write};
use std::process::Command;

pub const RUST_COMPILER : &str = "rustc";
pub const FILE_NAME : &str = "child.rs";
pub const FILE_EXECUTION_COMMAND : &str = "./child";

pub fn user_input(mut file: File, curr_dir: &PathBuf) {

    loop {
        println!("\n# Please give your input: (PS Enter: exit to exit or clear to clear file)\n"); 
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input) ;

        match input.trim() {
            "exit" => {
                println!("\nExiting the program ...\n");
                clear_already_existing_files();
                break;
            },
            "clear" => {
                println!("\n # clearing the input file ...");
                
                clear_already_existing_files();
                file = create_file(curr_dir);

                input.clear();
            },
            _ => {
                println!("*************Your File************\n");
                file.write_all(input.as_bytes()).unwrap();
                read_file(&curr_dir);
                println!("\n**********************************");

                //let's try to work like an interpretor
                compile();
            },
        }
    }
}

pub fn clear_already_existing_files() {
    //edge case where the file already exists delete the files
    if std::path::Path::new("child.rs").exists() {
        let _ = fs::remove_file("child.rs").expect("File does not exist");
    }
    if std::path::Path::new("child").exists() {
        let _ = fs::remove_file("child").expect("file does not exist");
    }
}

pub fn create_file(curr_dir: &PathBuf) -> File {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(curr_dir)
        .unwrap()
}

pub fn compile() {
    
    let compile_result = Command::new(RUST_COMPILER)
                            .arg(FILE_NAME)
                            .output()
                            .unwrap();
    if compile_result.status.success() {
        println!("\n# sucessfully compiled");

        // lets execute the ./child
        let execution_result = Command::new(FILE_EXECUTION_COMMAND)
                            .output()
                            .unwrap();

        // println!("Status {}",execution_result.status.success());
        let res = String::from_utf8(execution_result.stdout).unwrap();
        println!("\n*************Output***************\n");
        println!("\n {}", res);
        println!("\n**********************************\n");
    } else {
        println!("\n# compilation error: please check your input\n");
    }
}

pub fn read_file(path : &PathBuf) {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();
    println!("{}",buf);
}

