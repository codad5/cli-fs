use std::{path::Path, fs, io::Write};

use fli::Fli;
fn main() {
    let mut app = Fli::init("file Helper", "A simple cli app for common file command");
    let ls = app.command("ls", "List Current Dir content");
    ls.default(list_dir);
    ls.option("-b --brief", "List the dir content in brief", list_dir);

    let move_command = app.command("move", "move file path");
    move_command.option("-f --force", "force move", move_path);
    move_command.default(move_path);

    let create_command = app.command("create", "To create a file");
    create_command.option("-p --put, <>", "To put content to the file", create_file);
    create_command.default(create_file);

    let speak_command = app.command("speak", "To speak a word");
    speak_command.default(|x : &Fli| {
        let word = x.get_arg_at(1).unwrap();
        println!("speak: {}", word);
    });
    //say the content of a file
    speak_command.option("-f --file, <>", "To speak the content of a file", speak_file_content);

    let write_command = app.command("write", "To write a word to a file");
    write_command.option("-a --append", "Append the file instead", write_file);
    write_command.option("-f --force", "force", |_x|{}); // do nothing
    write_command.default(write_file);

    app.run();
}


fn list_dir(x : &Fli){
    let mut path = x.get_arg_at(1).expect("path is required");
    if path == "." {
        path = "./".to_string();
    }
    let path = Path::new(&path);
    if !path.exists() {
        println!("{} not exists", path.display());
        return;
    }
    if !path.is_dir() {
        println!("{} is not a dir", path.display());
        return;
    }
    let is_brief = x.is_passed("brief".to_string());
    println!("is_brief: {}", is_brief);
    // if brief is passed, list the dir content in brief else list the dir content in detail wirh file size, date, etc.
    if is_brief {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            println!("{}", file_name);
        }
    } else {
        println!("{0: <20} | {1: <10} | {2: <10} | {3: <10}", "Name", "Type", "Size", "Date");
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let file_type = if path.is_dir() {
                "dir"
            } else {
                "file"
            };
            let file_size = fs::metadata(&path).unwrap().len();
            let file_size = if file_size > 1024 {
                format!("{}K", file_size / 1024)
            } else {
                format!("{}B", file_size)
            };
            let file_date = fs::metadata(&path).unwrap().modified().unwrap();
            // let file_date = file_date.
            println!("{0: <20} | {1: <10} | {2: <10} | {3: <10}", file_name, file_type, file_size, format!("{:?}", file_date));
        }
    }
}

fn move_path(x : &Fli) {
    let from = x.get_arg_at(1).unwrap();
    let is_forced = x.is_passed("force".to_string());
    let to = x.get_arg_at(2).unwrap();
    println!("from: {}, to: {}", from, to);
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);
    if !from_path.exists() {
        println!("{} not exists", from);
        return;
    }
    if to_path.exists() && !is_forced {
        println!("{} already exists", to);
        return;
    }
    if !to_path.is_dir() && !is_forced {
        println!("{} is not a dir", to);
        return;
    }
    if !to_path.is_dir() {
        fs::create_dir_all(to_path).unwrap();
    }
    fs::rename(from_path, to_path).unwrap();
    println!("{} moved to {}", from, to);
}

fn create_file(x : &Fli) {
    let file_name = x.get_arg_at(1).unwrap();
    let file_path = Path::new(&file_name);
    let content = match x.get_values("put".to_string()) {
        Ok(values) => values[0].clone(),
        Err(_) => "".to_string()
    };
    if file_path.exists() {
        println!("{} already exists", file_name);
        return;
    }
    fs::File::create(file_path).unwrap();
    if content != "" {
        fs::write(file_path, content).unwrap();
    }
    println!("{} created", file_name);
}

fn speak_file_content(x : &Fli) {
    let file_name = match x.get_values("file".to_string()) {
        Ok(values) => values[0].clone(),
        Err(_) => "".to_string()
    };
    if file_name == "" {
        x.print_help("Invalid file name");
        return;
    }
    let file_path = Path::new(&file_name);
    if !file_path.exists() {
        println!("{} not exists", file_name);
        return;
    }
    if !file_path.is_file() {
        println!("{} is not a file", file_name);
        return;
    }
    let content = fs::read_to_string(file_path).unwrap();
    println!("file says: {}", content);
}

fn write_file(x : &Fli){
    let word = x.get_arg_at(2).unwrap();
    let file_name = x.get_arg_at(1).unwrap();
    let file_path = Path::new(&file_name);
    if !file_path.exists() && !x.is_passed("force".to_string()) {
        println!("{} not exists", file_name);
        return;
    }
    else if !file_path.exists() {
        fs::File::create(file_path).unwrap();
    }
    if !file_path.is_file() {
        println!("{} is not a file", file_name);
        return;
    }
    let mut file = fs::OpenOptions::new();
    if x.is_passed("append".to_string()) {
        file.append(true);
    }
    let mut file = file.open(file_path).unwrap();
    file.write_all(word.as_bytes()).unwrap();
    println!("{} written to {}", word, file_name);
}
