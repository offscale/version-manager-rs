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
  - [MySQL](https://www.mysql.com) ⊕ [$name](https://$name.org)
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

## Comparison to [Docker](https://www.docker.com), [Terraform](https://www.terraform.io) and [Packer](https://www.packer.io), [Puppet](https://www.puppet.com), &etc.

This is not a replacement for any of these technologies. It does simplify their usage, and also simplify usage without any of these tools (e.g., normal native deployment with 5 lines to deploy 5 systems).

In terms of simplifying their usage, your [Dockerfile](https://docs.docker.com/engine/reference/builder), [Chef](https://www.chef.io) recipe or what have you become one line (e.g., for a custom [$name](https://$name.org) image) or five lines (e.g., for a full application with no dependent images).

It also makes it easier to mix-and-match with hosted services, e.g., swap out self-hosted [PostgreSQL](https://www.postgresql.org) for [Google Cloud](https://cloud.google.com)'s DBaaS [AlloyDB](https://cloud.google.com/alloydb).

## This crate

This crate defines the CLI that is then used by all these version managers. All arguments are available in all version managers, however more can be added that are very specific to that version manager. Each of the above will have their own GitHub repository and crate, with binary releases available.

Also note that the CLI arguments are language agnostic, e.g., here is a go implementation for [PostgreSQL](https://www.postgresql.org):
https://github.com/offscale/postgres-version-manager-go

<hr/>

## Command Overview

* [`$name-version-manager-rs`↴](#$name-version-manager-rs)
* [`$name-version-manager-rs download`↴](#$name-version-manager-rs-download)
* [`$name-version-manager-rs env`↴](#$name-version-manager-rs-env)
* [`$name-version-manager-rs install`↴](#$name-version-manager-rs-install)
* [`$name-version-manager-rs ls`↴](#$name-version-manager-rs-ls)
* [`$name-version-manager-rs ls-remote`↴](#$name-version-manager-rs-ls-remote)
* [`$name-version-manager-rs reload`↴](#$name-version-manager-rs-reload)
* [`$name-version-manager-rs start`↴](#$name-version-manager-rs-start)
* [`$name-version-manager-rs stop`↴](#$name-version-manager-rs-stop)
* [`$name-version-manager-rs uri`↴](#$name-version-manager-rs-uri)
* [`$name-version-manager-rs install-service`↴](#$name-version-manager-rs-install-service)
* [`$name-version-manager-rs install-service open-rc`↴](#$name-version-manager-rs-install-service-open-rc)
* [`$name-version-manager-rs install-service systemd`↴](#$name-version-manager-rs-install-service-systemd)
* [`$name-version-manager-rs install-service windows-service`↴](#$name-version-manager-rs-install-service-windows-service)

## `$name-version-manager-rs`



**Usage:** `$name-version-manager-rs [OPTIONS] --port <PORT> <COMMAND>`

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

###### **Options:**

* `--app-version <APP_VERSION>`

  Default value: `latest`
* `--root <ROOT>`

  Default value: `$HOME/version-managers/$name-version-manager-rs`
* `--hostname <HOSTNAME>`

  Default value: `localhost`
* `-p`, `--port <PORT>`
* `--markdown-help`



## `$name-version-manager-rs download`

Download specified version

**Usage:** `$name-version-manager-rs download [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `$name-version-manager-rs env`

Print out associated environment variables

**Usage:** `$name-version-manager-rs env`



## `$name-version-manager-rs install`

Install specified version

**Usage:** `$name-version-manager-rs install [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `$name-version-manager-rs ls`

List what versions are installed

**Usage:** `$name-version-manager-rs ls`



## `$name-version-manager-rs ls-remote`

List what versions are available

**Usage:** `$name-version-manager-rs ls-remote`



## `$name-version-manager-rs reload`

Reload specified version

**Usage:** `$name-version-manager-rs reload [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `$name-version-manager-rs start`

Start specified version

**Usage:** `$name-version-manager-rs start [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `$name-version-manager-rs stop`

Stop specified version

**Usage:** `$name-version-manager-rs stop [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `$name-version-manager-rs uri`

Print out database connection string

**Usage:** `$name-version-manager-rs uri`



## `$name-version-manager-rs install-service`

Install service (daemon), e.g., systemd, OpenRC, windows-service

**Usage:** `$name-version-manager-rs install-service
install-service <COMMAND>`

###### **Subcommands:**

* `open-rc` — Install OpenRC service
* `systemd` — Install systemd service
* `windows-service` — Install Windows Service



## `$name-version-manager-rs install-service open-rc`

Install OpenRC service

**Usage:** `$name-version-manager-rs install-service open-rc [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `$name-version-manager-rs`
* `--config-install-path <CONFIG_INSTALL_PATH>`

  Default value: `/etc/conf.d/$name-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/init.d/$name-version-manager-rs`
* `--user <USER>`

  Default value: `$name-version-manager-rs`



## `$name-version-manager-rs install-service systemd`

Install systemd service

**Usage:** `$name-version-manager-rs install-service systemd [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `$name-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/systemd/system/$name-version-manager-rs.service`
* `--user <USER>`

  Default value: `$name-version-manager-rs`



## `$name-version-manager-rs install-service windows-service`

Install Windows Service

**Usage:** `$name-version-manager-rs install-service windows-service [OPTIONS]`

###### **Options:**

* `--service-name <SERVICE_NAME>`

  Default value: `$name-version-manager-rs`
* `--service-description <SERVICE_DESCRIPTION>`

  Default value: ``

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
