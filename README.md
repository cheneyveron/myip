# MyIP

A simple and blazing-fast HTTP server to query My IP, written in Rust. It works well behind proxies like `nginx`.

The origin intention to write is that I want an HTTP server that is as small as possible to run on my VPS and get pubic IP from my NAS. However the existing `ifconfig.io`

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

