# rust-ftp-server

## ⚙️Configuration

Set the .env file

```conf
RUST_LOG=info
FTP_HOME=/path/to/your/home
FTP_USER=username
FTP_PWD=password
FTP_PORT=8023
```

## 🚀Run

```log
> cargo run
[2024-11-11T20:21:00Z INFO  ftp_server_manager] FTP server: 192.168.1.1:8023 at dir: "/home/user/path"
```
