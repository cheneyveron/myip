# MyIP

A simple and blazing-fast HTTP server to query My IP, written in Rust. It works well behind proxies like `nginx`.

It uses 0.3%-0.7% CPU, and 436KB memory on a single-core VPS with `E5-2690 v2` under high load. And almost no CPU usage under light load.

The origin intention to write is that I want an HTTP server that is as small as possible to run on my VPS and get pubic IP from my NAS. However the existing open-sourced `ifconfig.io` contains a large number of HTML/CSS/JS files, and retrieves too many unneeded information. Also, golang's runtime environment has too many "overheads" for a task as simple as this.


## Secure Path

Modify `/MySecretPath` to any string to avoid being abused by some random guy on the internet.

Modify `0.0.0.0:8080` to listen to some random high port for even more peace of mind.

## How to build for x86_64 Linux

Make sure you have docker installed. Then run:

```bash
make build
```

The generated file will be in `target/x86_64-unknown-linux-musl/release/myip`

### Example service file for systemd

```
[Unit]
Description=My IP Service
After=network.target

[Service]
ExecStart=/opt/myip/myip
Restart=always
User=nobody
Group=nogroup
Environment="RUST_BACKTRACE=1"
# You can add more environment variables if needed

[Install]
WantedBy=multi-user.target
```

