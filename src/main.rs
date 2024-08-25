use std::env;

use utils::*;

pub mod utils;

fn main() {
    // check if bin and files already exists and remove them
    clear_already_existing_files();
    
    //  create the file
    let mut curr_dir = env::current_dir().unwrap();
    curr_dir.push(FILE_NAME);

    let file = create_file(&curr_dir);

    //take user input
    user_input(file, &curr_dir);
}

