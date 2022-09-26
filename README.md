# switchbot-exporter

The [exporter](https://github.com/prometheus/node_exporter) for [SwitchBot](https://www.switchbot.jp/) made in [Rust](https://www.rust-lang.org/).

## Build

```sh
# start app
$ docker compose up -d --build
# stop app
$ docker compose down --rmi local

# you can use cargo-make to start app
$ cargo make up
# or
$ makers up

# and stop app
$ cargo make down
# or
$ makers down
```
