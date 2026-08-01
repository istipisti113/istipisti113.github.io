use std::{cell::LazyCell, fs::File, io::{Read, Write}};

const TEMPLATE: LazyCell<String> = LazyCell::new(||{
    let mut file = File::open("templates/index.html").unwrap();
    let mut base = String::new();
    file.read_to_string(&mut base).unwrap();
    base
});

fn main() {
    println!("building pages");

    let files = std::fs::read_dir("input").unwrap();
    for file in files {
        //println!("{}", file.unwrap().file_name().to_str().unwrap());
        let name = file.unwrap().file_name();
        let name = name.to_str().unwrap().split(".").nth(0).unwrap();
        convert_file(name);
    }

}

fn convert_file(name: &str) {
    //let index = base.replace("CONTENT", INJECTED);
    println!("name: {name}");
    let mut md_content = String::new();
    let mut md_file = File::open(format!("input/{}.md", name)).unwrap();

    md_file.read_to_string(&mut md_content).unwrap();

    let html_content = TEMPLATE.replace("CONTENT", &markdown::to_html(&md_content));

    println!("output/{}.html", name);
    let mut newfile = File::create(format!("output/{}.html", name)).unwrap();
    newfile.write(html_content.as_bytes()).unwrap();
    //println!("{contents}");
}
