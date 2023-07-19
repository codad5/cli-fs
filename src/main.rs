use std::{path::Path, fs};

use fli::Fli;
fn main() {
    println!("Hello, world!");
    let mut app = Fli::init("file Helper", "A simple cli app for common file command");
    let ls = app.command("ls", "List Current Dir content");
    ls.option("-d --dir, []", "List Current Dir content", list_dir);
    ls.option("-b --brief", "List in brief", list_dir);

    let move_command = app.command("move", "move file path");
    move_command.option("-f --from, <>", "File/path to move", move_path);
    move_command.option("-to --to, <>", "Where to move file to", move_path);
    app.run();
}


fn list_dir(x : &Fli){
    let dir: String = match x.get_values("dir".to_string()) {
        Ok(dir) => dir[0].to_string(),
        Err(_) => "./".to_string()
    };

    let path = Path::new(&dir);
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if x.is_passed("brief".to_string()) {
            println!("{:?}", entry.file_name());
            continue;
        }
        // let metadata = entry.metadata().unwrap();
        // let size = metadata.len();
        // let created = metadata.created();
        // let modified = metadata.modified();
    }
}

fn move_path(x : &Fli) {
    let from = match x.get_values("from".to_string()){
        Ok(path) => path[0].to_string(),
        Err(_) => panic!()
    };
    let to_path = match x.get_values("to".to_string()) {
        Ok(path) => path[0].to_string(),
        Err(_) => panic!()
    };
    let to = Path::new(&to_path);
    if !to.is_dir() {
        fs::create_dir_all(to_path.clone()).expect(&format!("Error creating directory: {}", to_path.clone()))
    }
    fs::rename(from, to).expect("Something went wrong");
}