# File Upload/Download utilities

### Server

```sh
$ cargo run --p obj-down-up-load-server
Reading config from config.server.toml...
Running server on http://127.0.0.1:3000 ...
```

```sh
$ curl -F "name-of-the-file=@$PWD/Cargo.toml" http://localhost:3000/obj-load/upload?token=12345
OK

$ curl http://localhost:3000/obj-load/download?token=12345&filename=name-of-the-file
```

### Client

```sh
$ cargo run --p obj-upload README.md
Uploading file 'README.md' as README.md to configured server
OK

$ cargo run -p obj-download README.md
Downloading file 'README.md' to the current directory
OK
```

# Installing pre-compiled binaries

1. Install server package from [the latest release](https://github.com/iliabylich/obj-load/releases/tag/latest)

```sh
$ wget <URL_OF_DEV>
$ dpkg -i obj-down-up-load-server_<VERSION>_amd64.deb
```

2. copy `/etc/obj-down-up-load/config.example.toml` to `/etc/obj-down-up-load/config.toml` and edit it as you need
3. enable systemctl server
4. setup Nginx proxy

To get client on x86_64-apple-darwin (Mac on Intel):

```sh
sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-upload-x86_64-apple-darwin -O /usr/local/bin/obj-upload
sudo chmod +x /usr/local/bin/obj-upload

sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-download-x86_64-apple-darwin -O /usr/local/bin/obj-download
sudo chmod +x /usr/local/bin/obj-download
```

To get client on aarch64-apple-darwin (Mac M1):

```sh
sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-upload-aarch64-apple-darwin -O /usr/local/bin/obj-upload
sudo chmod +x /usr/local/bin/obj-upload

sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-download-aarch64-apple-darwin -O /usr/local/bin/obj-download
sudo chmod +x /usr/local/bin/obj-download
```

To get client on Linux with glibc (x86_64-unknown-linux-gnu):

```sh
sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-upload-x86_64-unknown-linux-gnu -O /usr/local/bin/obj-upload
sudo chmod +x /usr/local/bin/obj-upload

sudo wget https://github.com/iliabylich/obj-load/releases/download/latest-client/obj-download-x86_64-unknown-linux-gnu -O /usr/local/bin/obj-download
sudo chmod +x /usr/local/bin/obj-download
```

and run

```sh
$ obj-upload <filename>
# to upload a file on one machine

$ obj-download <filename>
# to dowload it on other machine
```
