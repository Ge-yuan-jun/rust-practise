/**
 * Demo for transfrom a website to markdown by rust
 * 
 * how to run this,for example:
 * 
 * cargo run -- https://www.rust-lang.org rust.md
 */
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if let [_path, url, output, ..] = args.as_slice() {
        println!("url: {}, output: {}", url, output);

        println!("Fetching url: {}", url);
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();

        println!("Converting html to markdown...");
        let md = html2md::parse_html(&body);

        fs::write(output, md.as_bytes()).unwrap();
        println!("Converted markdown has been saved in {}.", output);
    } else {
        eprintln!("参数缺失");
    }
}