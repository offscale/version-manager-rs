version-manager-rs
==================
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The future base of configuration management tooling. Use this set of macros/functions/design-patterns to write independent version managers.

## Novelty

This idea isn't new, e.g., see:
- [`rvm`](https://rvm.io) for [Ruby](https://www.ruby-lang.org/en);
- [`nvm`](https://github.com/nvm-sh/nvm) for [Node.js](https://nodejs.org/en);
- [`rustup`](https://rustup.rs) for [Rust](https://www.rust-lang.org);
- [`pyenv`](https://github.com/pyenv/pyenv) for [Python](https://www.python.org).

What is new is taking this to its logical extreme, ending up with independent cross-platform version managers for:

- Databases
  - [PostgreSQL](https://www.postgresql.org)
  - [MySQL](https://www.mysql.com) ⊕ [MariaDB](https://mariadb.org)
  - [RabbitMQ](https://www.rabbitmq.com/)
  - [Memcached](https://memcached.org)
  - [MongoDB](https://www.mongodb.com)
  - [Redis](https://redis.io)
  - …
- Web servers
  - [Apache HTTP (httpd)](https://httpd.apache.org)
  - [nginx](https://nginx.org/en/)
  - …
- Languages
  - [Java](https://www.java.com/en/download/help/whatis_java.html) (limit to [free software implementations](https://en.wikipedia.org/wiki/Free_Java_implementations)?)
  - [Python](https://www.python.org)
  - [go](https://go.dev)
  - [zig](https://ziglang.org)
  - [Rust](https://www.rust-lang.org)
  - [Ruby](https://www.ruby-lang.org/en)
  - [JavaScript](https://en.wikipedia.org/wiki/JavaScript) runtime
    - [bun](https://bun.sh)
    - [Node.js](https://nodejs.org/en)
    - [deno](https://deno.com)
  - [PHP](https://www.php.net)
  - …
- Search engines
  - [Elastic (ElasticSearch)](https://github.com/elastic/elasticsearch)
  - …
- Operations
  - Dashboarding
    - [Grafana](https://grafana.com)
    - …
  - Metrics and monitoring
    - [Prometheus](https://prometheus.io)
    - [InfluxDB](https://github.com/influxdata/influxdb)
    - …
  - Alerting
    - [Nagios](https://www.nagios.org)
    - …
- Applications (dependencies from—some of—↑)
  - [WordPress](https://wordpress.org)
  - [Odoo](https://www.odoo.com)
  - [OpenEdX](https://openedx.org)
  - …

## Comparison to: [Docker](https://www.docker.com); [Terraform](https://www.terraform.io) and [Packer](https://www.packer.io); [Puppet](https://www.puppet.com); &etc.

This is not a replacement for any of these technologies. It does simplify their usage, and also simplify usage without any of these tools (e.g., normal native deployment with 5 lines to deploy 5 systems).

In terms of simplifying their usage, your [Dockerfile](https://docs.docker.com/engine/reference/builder), [Chef](https://www.chef.io) recipe or what have you become one line (e.g., for a custom [MariaDB](https://mariadb.org) image) or five lines (e.g., for a full application with no dependent images).

It also makes it easier to mix-and-match with hosted services, e.g., swap out self-hosted [PostgreSQL](https://www.postgresql.org) for [Google Cloud](https://cloud.google.com)'s DBaaS [AlloyDB](https://cloud.google.com/alloydb).

## This crate

This crate defines the CLI that is then used by all these version managers. All arguments are available in all version managers, however more can be added that are very specific to that version manager. Each of the above will have their own GitHub repository and crate, with binary releases available.

Also note that the CLI arguments are language agnostic, e.g., here is a go implementation for [PostgreSQL](https://www.postgresql.org):
https://github.com/offscale/postgres-version-manager-go

### Work in progress

  - MariaDB - https://github.com/offscale/mariadb-version-manager-rs

# Command-Line Help for `NAME-manager-rs`

This document contains the help content for the `NAME-manager-rs` command-line program.

**Command Overview:**

* [`NAME-manager-rs`↴](#NAME-manager-rs)
* [`NAME-manager-rs download`↴](#NAME-manager-rs-download)
* [`NAME-manager-rs env`↴](#NAME-manager-rs-env)
* [`NAME-manager-rs install`↴](#NAME-manager-rs-install)
* [`NAME-manager-rs ls`↴](#NAME-manager-rs-ls)
* [`NAME-manager-rs ls-remote`↴](#NAME-manager-rs-ls-remote)
* [`NAME-manager-rs reload`↴](#NAME-manager-rs-reload)
* [`NAME-manager-rs start`↴](#NAME-manager-rs-start)
* [`NAME-manager-rs stop`↴](#NAME-manager-rs-stop)
* [`NAME-manager-rs uri`↴](#NAME-manager-rs-uri)
* [`NAME-manager-rs install-service`↴](#NAME-manager-rs-install-service)
* [`NAME-manager-rs install-service open-rc`↴](#NAME-manager-rs-install-service-open-rc)
* [`NAME-manager-rs install-service systemd`↴](#NAME-manager-rs-install-service-systemd)
* [`NAME-manager-rs install-service windows-service`↴](#NAME-manager-rs-install-service-windows-service)

## `NAME-manager-rs`



**Usage:** `NAME-manager-rs [OPTIONS] --port <PORT> <COMMAND>`

###### **Subcommands:**

* `download` — Download specified version
* `env` — Print out associated environment variables
* `install` — Install specified version
* `ls` — List what versions are installed
* `ls-remote` — List what versions are available
* `reload` — Reload specified version
* `start` — Start specified version
* `stop` — Stop specified version
* `uri` — Print out database connection string
* `install-service` — Install service (daemon), e.g., systemd, OpenRC, windows-service
* `unknown` —

###### **Options:**

* `--vms-config <VMS_CONFIG>` — Config file to read from. If provided used as new default (before env and argv res)

  Default value: `$HOME/version-managers/NAME-manager-rs/vms-config.json`
* `--config-read` — Whether to read from config file. If vms_config provided, this defaults to `true`

  Default value: `false`
* `--config-write` — Whether to write to config file

  Default value: `true`
* `--app-version <APP_VERSION>` — Desired version of application

  Default value: `latest`
* `--root <ROOT>` — Root directory. By default all paths are relative to this one

  Default value: `$HOME/version-managers/NAME-manager-rs`
* `--hostname <HOSTNAME>` — Hostname of server

  Default value: `localhost`
* `-p`, `--port <PORT>` — Port for server to listen on
* `--database <DATABASE>` — Database name

  Default value: `database`
* `--runtime-path <RUNTIME_PATH>` — Runtime path. This is where PID files and/or similar temporary files are stored.

  Default value: `$HOME/version-managers/NAME-manager-rs/NAME-manager-rs/$APP_VERSION/run`
* `--data-path <DATA_PATH>` — Data path. This is where the actual data is stored, e.g., the .db and WAL files

  Default value: `$HOME/version-managers/NAME-manager-rs/NAME-manager-rs/$APP_VERSION/data`
* `--bin-path <BIN_PATH>` — Binary path. Where the executable binary are located. Sometimes called PREFIX

  Default value: `$HOME/version-managers/NAME-manager-rs/NAME-manager-rs/$APP_VERSION/bin`
* `--logs-path <LOGS_PATH>` — Logs path. Where the log files are to be stored

  Default value: `$HOME/version-managers/NAME-manager-rs/NAME-manager-rs/$APP_VERSION/logs`
* `--locale <LOCALE>` — Locale to use

  Default value: `en_US.UTF-8`
* `--markdown-help` — Markdown help generator. Only really used to generate replacement README.md files



## `NAME-manager-rs download`

Download specified version

**Usage:** `NAME-manager-rs download [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `NAME-manager-rs env`

Print out associated environment variables

**Usage:** `NAME-manager-rs env`



## `NAME-manager-rs install`

Install specified version

**Usage:** `NAME-manager-rs install [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `NAME-manager-rs ls`

List what versions are installed

**Usage:** `NAME-manager-rs ls`



## `NAME-manager-rs ls-remote`

List what versions are available

**Usage:** `NAME-manager-rs ls-remote`



## `NAME-manager-rs reload`

Reload specified version

**Usage:** `NAME-manager-rs reload [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `NAME-manager-rs start`

Start specified version

**Usage:** `NAME-manager-rs start [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `NAME-manager-rs stop`

Stop specified version

**Usage:** `NAME-manager-rs stop [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `NAME-manager-rs uri`

Print out database connection string

**Usage:** `NAME-manager-rs uri`



## `NAME-manager-rs install-service`

Install service (daemon), e.g., systemd, OpenRC, windows-service

**Usage:** `NAME-manager-rs install-service
install-service <COMMAND>`

###### **Subcommands:**

* `open-rc` — Install OpenRC service
* `systemd` — Install systemd service
* `windows-service` — Install Windows Service



## `NAME-manager-rs install-service open-rc`

Install OpenRC service

**Usage:** `NAME-manager-rs install-service open-rc [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `NAME-manager-rs`
* `--config-install-path <CONFIG_INSTALL_PATH>`

  Default value: `/etc/conf.d/NAME-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/init.d/NAME-manager-rs`
* `--user <USER>`

  Default value: `NAME-manager-rs`



## `NAME-manager-rs install-service systemd`

Install systemd service

**Usage:** `NAME-manager-rs install-service systemd [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `NAME-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/systemd/system/NAME-manager-rs.service`
* `--user <USER>`

  Default value: `NAME-manager-rs`



## `NAME-manager-rs install-service windows-service`

Install Windows Service

**Usage:** `NAME-manager-rs install-service windows-service [OPTIONS]`

###### **Options:**

* `--service-name <SERVICE_NAME>`

  Default value: `NAME-manager-rs`
* `--service-description <SERVICE_DESCRIPTION>`

  Default value: ``



## `NAME-manager-rs unknown`

**Usage:** `NAME-manager-rs unknown`



<hr/>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
