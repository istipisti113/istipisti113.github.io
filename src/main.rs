use std::{arch::naked_asm, cell::LazyCell, ffi::{OsStr, OsString},
    fs::{self, File, create_dir, write}, io::{Read, Write},
    path::{Component, Path, PathBuf}, thread::current};
use markdown::{CompileOptions, Options, to_html_with_options};

const TEMPLATE: LazyCell<String> = LazyCell::new(||{
    let mut file = File::open("templates/index.html").unwrap();
    let mut base = String::new();
    file.read_to_string(&mut base).unwrap();
    base
});

const BACK_LINK: LazyCell<String> = LazyCell::new(||{
    String::from("<a href=\"TARGET\" style=\"float: right;\">back</a>")
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
    //println!("current: {}, is_dir: {}", current.to_str().unwrap(), current.is_dir());
    if current.is_dir(){
        create_output_dir(current);
        for entry in current.read_dir().unwrap(){
            let valid = entry.unwrap().path();
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
    let name = path.file_prefix().unwrap(); //only the name, no extension
    let mut md_content = String::new();
    File::read_to_string(&mut File::open(&path).unwrap(), &mut md_content).unwrap();

    //let mut timestamp = md_content.split("$$$").nth(0).unwrap();

    let options = Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::default()
        },
        ..Options::default()
    };

    let mut back = path.components().collect::<Vec<_>>();
    back.pop();
    back.remove(0);
    if back.len()>0{
        back.pop();
    }

    let html_content = TEMPLATE.replace("CONTENT", 
        &(BACK_LINK.clone().replace("TARGET", 
            &format!("/{}", back.iter().map(|c|{
                c.as_os_str().to_str().unwrap()
            }).collect::<Vec<&str>>().join("/"))
        ) + &to_html_with_options(&md_content, &options).unwrap())
    );


    if !Path::new("output").exists(){
        create_dir("output").unwrap();
    }
    let output_path = change_to_html(&change_to_output(&path));
    write(output_path, html_content).unwrap();
}
