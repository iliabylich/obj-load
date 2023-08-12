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

TODO
