<p align="center">
  <img src="https://user-images.githubusercontent.com/26155267/179846683-a5e09b59-7344-4b88-be20-4b2a0fa0f610.png" alt="ecg"></img>
</p>

[![Integration](https://github.com/setten-io/ecg/actions/workflows/integration.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/integration.yaml)
[![Delivery](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml)
![Crates.io](https://img.shields.io/crates/v/ecg)
![GitHub](https://img.shields.io/github/license/setten-io/ecg)

`ecg` is a cosmos validator heartbeat monitoring tool.

---

## Getting Started

This tool is installable as a crate.

```bash
cargo install ecg
```

Or available as a Docker image.

```bash
docker pull ghcr.io/setten-io/ecg:$VERSION
```

## Usage

Ecg is made to work with a [heartbeat](https://betterstack.com/community/guides/monitoring/what-is-cron-monitoring/) (or "cron") monitor available on services like betteruptime.

In short, it works like a dead man switch.
As long as ecg is able to send a request to the monitor at a regular interval, alerting will not be triggered.
The default state being alerting, you will get notified in many edge scenarios where traditional monitoring would fail.

Some example failures that might not get you notified:

- Your whole infra loses connectivity
- Monitoring services is down at the same time as the validator
- Monitoring service can't access the validator
- Monitoring service lose access to its data source (ie: LCD)

> **Warning**
>
> Monitoring is essential, ecg and heartbeat type monitoring are complementary to other tools and methods.
>
> They should be used together in conjuction.

### Checks

Ecg will not send heartbeat if, since last checked:

- block height didn't increase
- validator block misses increased
- validator is tombstoned

### Configuration

```md
ecg 0.1.0
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

### Running

Example with the binary.

```bash
# configured through arguments
ecg terravalcons1abcdef1234567890 https://phoenix-lcd.terra.dev https://your-heartbeat-monitor-url.com
# with debug logging and custom interval
ECG_LOG=debug ecg --interval 20 terravalcons1abcdef1234567890 https://phoenix-lcd.terra.dev https://your-heartbeat-monitor-url.com

# or

# configured through env
export VALCONS_ADDR="terravalcons1abcdef1234567890"
export LCD_URL="https://phoenix-lcd.terra.dev"
export HEARTBEAT_URL="https://your-heartbeat-monitor-url.com"
ecg
```

Example via docker.

```bash
docker run \
  -e VALCONS_ADDR="terravalcons1abcdef1234567890" \
  -e LCD_URL="https://phoenix-lcd.terra.dev" \
  -e HEARTBEAT_URL="https://your-heartbeat-monitor-url.com" \
  ghcr.io/setten-io/ecg:$VERSION
```
