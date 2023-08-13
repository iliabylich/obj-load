use obj_load::{filepath_from_args, zip, Config};
use reqwest::blocking::multipart;
use reqwest::blocking::Client;

pub fn main() {
    Config::init();
    let url = format!("{}/obj-load/upload", Config::get().server);
    let mut filepath = filepath_from_args();

    if std::path::Path::new(&filepath).is_dir() {
        println!("It's a folder! Zipping..");
        filepath = zip(&filepath);
    }

    let filename = std::path::Path::new(&filepath)
        .file_name()
        .expect("Invalid filepath given")
        .to_str()
        .expect("Invalid filepath given")
        .to_string();
    println!(
        "Uploading file '{}' as {} to configured server",
        filepath, filename
    );

    let form = multipart::Form::new()
        .file(filename, filepath)
        .expect("Failed to build multipart form");

    let res = Client::new()
        .post(url)
        .query(&[("token", &Config::get().auth_token)])
        .multipart(form)
        .send()
        .unwrap();

    if res.status() == 200 {
        println!("OK");
    } else {
        eprintln!("Failed to upload file");
        dbg!(res);
        std::process::exit(1);
    }
}
