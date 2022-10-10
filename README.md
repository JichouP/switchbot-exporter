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

## How to use

1. Fork this repository
1. Set the following environment variables
   - `SWITCH_BOT_TOKEN`
   - `SWITCH_BOT_SECRET`
   - `SWITCH_BOT_METER_PLUS_DEVICE_ID`
   - `SWITCH_BOT_PLUG_MINI_PC_DEVICE_ID`
   - `SWITCH_BOT_PLUG_MINI_DESK_DEVICE_ID`
1. Run CI
