use axum::{
    body::Body,
    extract::{Multipart, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use obj_load::Config;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() {
    Config::init();

    let app = Router::new()
        .route("/obj-load/upload", post(upload))
        .route("/obj-load/download", get(download));

    let port = Config::get().port;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
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
    let body = Body::from_stream(ReaderStream::new(file));
    let mut res = Response::new(body);
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
