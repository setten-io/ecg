<p align="center">
  <img src="https://user-images.githubusercontent.com/26155267/183662242-e463b043-c58f-449b-a9fb-0130ee8bb57d.png" alt="ecg"></img>
</p>

[![Integration](https://github.com/setten-io/ecg/actions/workflows/integration.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/integration.yaml)
[![Delivery](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml/badge.svg)](https://github.com/setten-io/ecg/actions/workflows/delivery.yaml)
[![Crate](https://img.shields.io/crates/v/ecg)](https://crates.io/crates/ecg)
[![License](https://img.shields.io/github/license/setten-io/ecg?no-cache)](https://github.com/setten-io/ecg/blob/main/LICENSE)

`ecg` is a cosmos validators [Dead Man's Switch](https://en.wikipedia.org/wiki/Dead_man%27s_switch).

---

As long as the monitor is receiving heartbeats from ecg, alerting will not be triggered.
This "failure as default" approach makes this kind of monitoring resilient to failures that might otherwise prevent you from receiving any alert.

Examples:

- Your whole infra loses connectivity
- Monitoring services is down at the same time as the validator
- Monitoring service can't access the validator
- Monitoring service lose access to its data source (ie: LCD)

> **Warning**
>
> Monitoring is essential.
> Ecg and heartbeat style monitoring are complementary to other monitoring tools and methods.
> We recommend ecg as an aditional failsafe alongside more proactive tools.

There are multiple SaaS platforms like [Better Uptime](https://betteruptime.com) or [Dead Man's Snitch](https://deadmanssnitch.com) that implement heartbeat monitoring.

## Features

`ecg` will:

1. For each target, fetch a state from all clients
2. Keep the freshest one (highest block height)
3. Run checks on it 
4. If they all pass, send heartbeat

Checks include:

- block height increased
- validator block misses didn't increase
- validator isn't jailed
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
    ecg [OPTIONS]

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    Path to yaml config [env: ECG_CONFIG_PATH=] [default: ecg.yaml]
    -V, --version        Print version information
```

It is possible to change ecg logging level through the `ECG_LOG` env var, or for all/any module through the `RUST_LOG` env var.

## Configuration

`ecg` is configured through a yaml file.

The default path is `./ecg.yaml` but can be specified through the `-p/--path` flag or the `ECG_CONFIG_PATH` env var.

### Example

<details>
<summary><code>ecg.yaml</code></summary>
<br>

```yaml
targets:
  phoenix:
    url: https://betteruptime.com/api/v1/heartbeat/fFKHCd3YNkayv8Fr6MJAFE3w
    valcons_address: terravalcons1qqyfhs9oacvteimwdpbt77fis88mie5gx6gxf2
    interval: 10
    clients:
      - type: lcd
        url: https://phoenix-lcd.terra.dev
      - type: lcd
        url: https://terra-api.polkachu.com
      - type: setten-lcd
        project_id: ea08855653b64998bb47b2c03bf66de7
        key: 02215b36969446c28b22059e63b4301b
        network: phoenix
        blockchain: terra
  kaiyo:
    url: https://betteruptime.com/api/v1/heartbeat/t6xm2P7Ujfjz3ph5TNBFti8X
    valcons_address: kujiravalcons14rt55jpahf4giiupxrxivy85ecog2onb29a2ev
    interval: 2
    clients:
      - type: lcd
        url: https://lcd.kaiyo.kujira.setten.io
      - type: lcd
        url: https://kujira-api.polkachu.com
```

</details>


### Specification

<details>
<summary><code>targets.&lt;name&gt;</code></summary>
<br>

The name of your target;
Should be self-explanatory.

</details>


<details>
<summary><code>targets.&lt;name&gt;.url</code></summary>
<br>

The url of the monitor to send `GET` http heartbeat requests to.

</details>


<details>
<summary><code>targets.&lt;name&gt;.valcons_address</code></summary>
<br>

The validator valcons address.

It can be found using the cosmos sdk chain binary cli:

```bash
terrad tendermint show-address
```

</details>


<details>
<summary><code>targets.&lt;name&gt;.interval</code></summary>
<br>

> Optional, default to 30

Interval in seconds between each cycle (run checks + send heartbeat).

You must set this in accordance to the heartbeat frequency your monitor is expecting to receive and the chain block time.

</details>


<details>
<summary><code>targets.&lt;name&gt;.clients</code></summary>
<br>

Array of redundant clients to querry in parallel.

</details>


<details>
<summary><code>targets.&lt;name&gt;.clients[*].type</code></summary>
<br>

Type defines the kind of clients and the configuration keys that will be available.

Available client types and their configurations:

<details>
<summary><code>lcd</code></summary>
<br>

* `url` - LCD endpoint to query (ex: `https://kujira-api.polkachu.com`)

</details>

<details>
<summary><code>setten-lcd</code></summary>
<br>

* `project_id` - Setten project id (ex: `ea08855653b64998bb47b2c03bf66de7`)
* `key` - Setten project key (ex: `02215b36969446c28b22059e63b4301b`)
* `network` - Setten network slug (ex: `phoenix`)
* `blockchain` - Setten blockchain slug (ex: `terra`)

For network and blockchain slugs, please see [Setten's docs](https://docs.setten.io/concepts/products-and-networks#supported-networks)

</details>

</details>
