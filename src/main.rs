use std::{fs::File, io::{Read, Write}};

static INJECTED:  &str = "
<h1>this is the new header</h1>
<p>this is the new paragraph</p>
";

fn main() {
    println!("building pages");
    let mut file = File::open("templates/index.html").unwrap();
    let mut contents = String::new();
    File::read_to_string(&mut file, &mut contents).unwrap();
    drop(file);

    contents = contents.replace("CONTENT", INJECTED);
    let mut newfile = File::create("output/index.html").unwrap();
    newfile.write(contents.as_bytes()).unwrap();
    println!("{contents}");
    drop(newfile);
}
