<p align="center">
  <img src="https://user-images.githubusercontent.com/26155267/179846683-a5e09b59-7344-4b88-be20-4b2a0fa0f610.png" alt="ecg"></img>
</p>

[![Integration](https://github.com/setten-io/ecg/actions/workflows/integration.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/integration.yaml)
[![Delivery](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml)
[![Crate](https://img.shields.io/crates/v/ecg)](https://crates.io/crates/ecg)
[![License](https://img.shields.io/github/license/setten-io/ecg?no-cache)](https://github.com/setten-io/ecg/blob/main/LICENSE)

`ecg` is a cosmos validator heartbeat monitoring tool.

---

> **Warning**
>
> Monitoring is essential.
> Ecg and heartbeat style monitoring are complementary to other monitoring tools and methods.
> We recommend ecg as an aditional failsafe.

Ecg is made to work with a [heartbeat](https://betterstack.com/community/guides/monitoring/what-is-cron-monitoring/) (or "cron") style monitoring. Available on SaaS platforms like (Better Uptime)[https://betteruptime.com] or [Dead Man's Snitch](https://deadmanssnitch.com).

In short, it works like a dead man switch.
As long as the heartbeat monitor is receiving requests from ecg at a predifined interval, alerting will not be triggered.
It makes this kind of monitoring resilient to failures that might otherwise prevent you from receiving any alert:

- Your whole infra loses connectivity
- Monitoring services is down at the same time as the validator
- Monitoring service can't access the validator
- Monitoring service lose access to its data source (ie: LCD)

Currently ecg will send heartbeat if all those conditions are met:

- block height increased
- validator block misses didn't increase
- validator isn't tombstoned

## Getting Started

This tool is installable as a crate.

```bash
cargo install ecg
```

Or available as a Docker image.

```bash
docker pull ghcr.io/setten-io/ecg
```

## Usage

```md
ecg
Heartbeats for cosmos validators

USAGE:
    ecg [OPTIONS] <VALCONS_ADDR> <LCD_URL> <HEARTBEAT_URL>

ARGS:
    <VALCONS_ADDR>     Validator tendermint consensus address [env: VALCONS_ADDR=]
    <LCD_URL>          LCD url used to communicate with the blockchain [env: LCD_URL=]
    <HEARTBEAT_URL>    Url to send heartbeats to [env: HEARTBEAT_URL=]

OPTIONS:
    -h, --help                   Print help information
    -i, --interval <INTERVAL>    Beat interval in seconds [env: INTERVAL=] [default: 10]
    -V, --version                Print version information
```

It is possible to change ecg logging level through the `ECG_LOG` env var, or for all/any module through the `RUST_LOG` env var.

### Examples

With the binary.

```bash
# configured through arguments
ecg \
  terravalcons1abcdef1234567890 \
  https://phoenix-lcd.terra.dev \
  https://your-heartbeat-monitor-url.com

# with debug logging and custom interval
export ECG_LOG=debug
ecg \
  terravalcons1abcdef1234567890 \
  https://phoenix-lcd.terra.dev \
  https://your-heartbeat-monitor-url.com \
  --interval 20

# or configured through environment
export VALCONS_ADDR="terravalcons1abcdef1234567890"
export LCD_URL="https://phoenix-lcd.terra.dev"
export HEARTBEAT_URL="https://your-heartbeat-monitor-url.com"
ecg
```

With the docker image.

```bash
docker run \
  -e VALCONS_ADDR="terravalcons1abcdef1234567890" \
  -e LCD_URL="https://phoenix-lcd.terra.dev" \
  -e HEARTBEAT_URL="https://your-heartbeat-monitor-url.com" \
  ghcr.io/setten-io/ecg
```
