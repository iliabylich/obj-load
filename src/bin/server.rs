use axum::{
    body::StreamBody,
    extract::{Multipart, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::OnceCell;
use tokio_util::io::ReaderStream;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    auth_token: String,
    data_dir: String,
}
static CONFIG: OnceCell<Config> = OnceCell::const_new();
impl Config {
    #[cfg(debug_assertions)]
    const CONFIG_PATH: &str = "config.dev.toml";
    #[cfg(not(debug_assertions))]
    const CONFIG_PATH: &str = "/etc/obj-down-up-load/config.toml";

    fn init() {
        println!("Reading config from {}...", Self::CONFIG_PATH);
        let config = toml::from_str(
            &std::fs::read_to_string(Self::CONFIG_PATH).expect("Unable to read config"),
        )
        .expect("Invalid TOML config");
        CONFIG.set(config).unwrap();
    }

    fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }
}

#[tokio::main]
async fn main() {
    Config::init();

    let app = Router::new()
        .route("/obj-down-up-load/upload", post(upload))
        .route("/obj-down-up-load/download", get(download));

    let addr = SocketAddr::from(([127, 0, 0, 1], Config::get().port));

    println!(
        "Running server on http://127.0.0.1:{} ...",
        Config::get().port
    );

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to spawn web server");
}

fn auth(given_token: &str) -> Result<(), (StatusCode, &'static str)> {
    if given_token == Config::get().auth_token {
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}

#[derive(Deserialize, Debug)]
struct UploadQuery {
    token: String,
}
async fn upload(
    query: Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    auth(&query.token)?;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Got file `{}` ({} bytes)", name, data.len());

        let filepath = format!("{}/{}", Config::get().data_dir, name);
        tokio::fs::write(filepath, data).await.unwrap();
    }

    Ok((StatusCode::OK, "OK"))
}

#[derive(Deserialize, Debug)]
struct DownloadQuery {
    token: String,
    filename: String,
}
async fn download(
    query: Query<DownloadQuery>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    auth(&query.token)?;

    let filepath = format!("{}/{}", Config::get().data_dir, query.filename);

    let file = match tokio::fs::File::open(filepath).await {
        Ok(file) => file,
        Err(_err) => return Err((StatusCode::NOT_FOUND, "File not found")),
    };
    let mut res = Response::new(StreamBody::new(ReaderStream::new(file)));
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    res.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename={}", query.filename)
            .parse()
            .unwrap(),
    );
    Ok(res)
}
