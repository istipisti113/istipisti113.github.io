use std::{arch::naked_asm, cell::LazyCell, ffi::OsStr, fs::{self, File, create_dir, write}, io::{Read, Write}, path::{Component, Path, PathBuf}, thread::current};

const TEMPLATE: LazyCell<String> = LazyCell::new(||{
    let mut file = File::open("templates/index.html").unwrap();
    let mut base = String::new();
    file.read_to_string(&mut base).unwrap();
    base
});

fn main() {
    println!("building pages");

    let entries = std::fs::read_dir("input").unwrap();
    
    for entry in entries {
        let valid = entry.unwrap().path();
        recursive_convert(&valid);
    }
}

fn recursive_convert(current: &PathBuf) {
    println!("current: {}, is_dir: {}", current.to_str().unwrap(), current.is_dir());
    if current.is_dir(){
        create_output_dir(current);
        for entry in current.read_dir().unwrap(){
            let valid = entry.unwrap().path();
            //if valid.as_path().is_dir() {
            //    recursive_convert(&valid);
            //} else {
            //    //println!("{}", file.unwrap().file_name().to_str().unwrap());
            //    //let name = valid.file_name().unwrap();
            //    //let name = name.to_str().unwrap().split(".").nth(0).unwrap();
            //    //convert_file(name);
            //    convert_file_path(valid);
            //}
            recursive_convert(&valid.to_path_buf());
        }
    } else {
        convert_file_path(current.to_path_buf());
    }
}

fn create_output_dir(path: &PathBuf) {
    let newpath = change_to_output(path);
    fs::create_dir_all(newpath).unwrap();
}

fn change_to_output(path: &PathBuf) -> PathBuf{
    path.as_path().components().into_iter().map(|c|{
        if c == Component::Normal(OsStr::new("input")) {
            Component::Normal(OsStr::new("output"))
        } else {
            c
        }
    }).collect::<PathBuf>()
}

fn change_to_html(path: &PathBuf) -> PathBuf {
    let mut filename = String::from(path.file_prefix().unwrap().to_str().unwrap());
    filename += ".html";
    let mut vec = path.as_path().components().collect::<Vec<_>>();
    vec.pop();
    vec.push(Component::Normal(OsStr::new(&filename)));
    vec.iter().collect()
}

fn convert_file_path(path: PathBuf) {
    println!("{:?}", path);
    let name = path.file_prefix().unwrap(); //only the name, no extension
    println!("{:?}", name);
    let mut md_content = String::new();
    File::read_to_string(&mut File::open(&path).unwrap(), &mut md_content).unwrap();

    let html_content = TEMPLATE.replace("CONTENT", &markdown::to_html(&md_content));

    if !Path::new("output").exists(){
        create_dir("output").unwrap();
    }
    let output_path = change_to_html(&change_to_output(&path));
    write(output_path, html_content).unwrap();
}
