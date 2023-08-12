# File Upload/Download utilities

### Server

```sh
$ cargo run --bin server --features=bin-server
Reading config from config.dev.toml...
Running server on http://127.0.0.1:3000 ...
```

```sh
$ curl -F "name-of-the-file=@$PWD/Cargo.toml" http://localhost:3000/obj-load/upload?token=12345
OK

$ curl http://localhost:3000/obj-load/download?token=12345&filename=name-of-the-file
```

### Client

```sh
$ cargo run --bin obj-upload --features=bin-obj-upload -- README.md
Uploading file 'README.md' as README.md to configured server
OK

$ cargo run --bin obj-download --features=bin-obj-download -- README.md
Downloading file 'README.md' to the current directory
OK
```
