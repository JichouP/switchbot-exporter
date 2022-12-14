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

Build for arm64

```sh
docker buildx create --name mybuilder
docker buildx use mybuilder
docker buildx build --platform linux/amd64,linux/arm64 -t jichoup/switchbot-exporter --push .
```

## How to use

See the [example](./example)

1. Set the following environment variables
   - `PORT`
   - `SWITCHBOT_TOKEN`
   - `SWITCHBOT_SECRET`
   - `SWITCHBOT_METER_PLUS_DEVICE_ID`
   - `SWITCHBOT_PLUG_MINI_PC_DEVICE_ID`
   - `SWITCHBOT_PLUG_MINI_DESK_DEVICE_ID`
1. Run `docker compose up`
