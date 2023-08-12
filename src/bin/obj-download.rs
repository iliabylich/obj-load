use obj_load::{filepath_from_args, Config};
use reqwest::blocking::Client;

pub fn main() {
    Config::init();
    let url = format!("{}/obj-load/download", Config::get().server);

    let filename = filepath_from_args();

    println!("Downloading file '{}' to the current directory", filename);

    let res = Client::new()
        .get(url)
        .query(&[
            ("filename", &filename),
            ("token", &Config::get().auth_token),
        ])
        .send()
        .unwrap();

    if res.status() == 200 {
        let bytes = res.bytes().unwrap();
        std::fs::write(filename, bytes).unwrap();
        println!("OK");
    } else {
        eprintln!("Failed to download file");
        dbg!(res);
        std::process::exit(1);
    }
}
