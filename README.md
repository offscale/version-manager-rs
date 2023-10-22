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

Imagine the [CNCF landscape](https://landscape.cncf.io) or any arbitrary stack, and being about to swap out components like:

    |------------|--------
    | Web server | Proxy |
    -----------------------------------------
    | Database | Cache | Queue | Filesystem |
    -----------------------------------------------------
    | AI/ML stack | Search engine | Recommender systems |
    -----------------------------------------------------
    | Language runtime | Function as a Service |
    -------------------------------------------
    | Logging | Alerting | Security scanning |
    ------------------------------------------
    | De/provisionioning infrastructure |
    ------------------------------------

## This crate

This crate defines the CLI that is then used by all these version managers. All arguments are available in all version managers, however more can be added that are very specific to that version manager. Each of the above will have their own GitHub repository and crate, with binary releases available.

Also note that the CLI arguments are language agnostic, e.g., here is a go implementation for [PostgreSQL](https://www.postgresql.org):
https://github.com/offscale/postgres-version-manager-go

### Work in progress

  - MariaDB - https://github.com/offscale/mariadb-version-manager-rs

# Command-Line Help for `NAME-version-manager-rs`

This document contains the help content for the `NAME-version-manager-rs` command-line program.

**Command Overview:**

* [`NAME-version-manager-rs`↴](#NAME-version-manager-rs)
* [`NAME-version-manager-rs download`↴](#NAME-version-manager-rs-download)
* [`NAME-version-manager-rs download-plan`↴](#NAME-version-manager-rs-download-plan)
* [`NAME-version-manager-rs env`↴](#NAME-version-manager-rs-env)
* [`NAME-version-manager-rs install`↴](#NAME-version-manager-rs-install)
* [`NAME-version-manager-rs install-dependencies`↴](#NAME-version-manager-rs-install-dependencies)
* [`NAME-version-manager-rs ls`↴](#NAME-version-manager-rs-ls)
* [`NAME-version-manager-rs ls-remote`↴](#NAME-version-manager-rs-ls-remote)
* [`NAME-version-manager-rs uri`↴](#NAME-version-manager-rs-uri)
* [`NAME-version-manager-rs service`↴](#NAME-version-manager-rs-service)
* [`NAME-version-manager-rs service install`↴](#NAME-version-manager-rs-service-install)
* [`NAME-version-manager-rs service install open-rc`↴](#NAME-version-manager-rs-service-install-open-rc)
* [`NAME-version-manager-rs service install systemd`↴](#NAME-version-manager-rs-service-install-systemd)
* [`NAME-version-manager-rs service install windows-service`↴](#NAME-version-manager-rs-service-install-windows-service)
* [`NAME-version-manager-rs service reload`↴](#NAME-version-manager-rs-service-reload)
* [`NAME-version-manager-rs service start`↴](#NAME-version-manager-rs-service-start)
* [`NAME-version-manager-rs service stop`↴](#NAME-version-manager-rs-service-stop)

## `NAME-version-manager-rs`



**Usage:** `NAME-version-manager-rs [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `download` — Download specified version
* `download-plan` — Resolve download URL and hash/checksum. Useful for security and concurrency.
* `env` — Print out associated environment variables
* `install` — Install specified version
* `install-dependencies` — Install (only) dependencies for specified version
* `ls` — List what versions are installed
* `ls-remote` — List what versions are available
* `uri` — Print out database connection string
* `service` — Service management

###### **Options:**

* `--vms-config <VMS_CONFIG>` — Config file to read from. If provided used as new default (before env and argv res)

  Default value: `$HOME/version-managers/NAME-version-manager-rs/vms-config.json`
* `--config-read` — Whether to read from config file. If vms_config provided, this defaults to  `true`

  Default value: `false`
* `--config-write` — Whether to write to config file

  Default value: `true`
* `--app-version <APP_VERSION>` — Desired version of application

  Default value: `latest`
* `--vm-root <VM_ROOT>` — root directory for all version-managers. For download cache and interdependencies

  Default value: `$HOME/version-managers`
* `--root <ROOT>` — Root directory. By default all paths are relative to this one

  Default value: `$HOME/version-managers/NAME-version-manager-rs`
* `--hostname <HOSTNAME>` — Hostname of server

  Default value: `localhost`
* `-p`, `--port <PORT>` — Port for server to listen on

* `--database <DATABASE>` — Database name

  Default value: `database`
* `--runtime-path <RUNTIME_PATH>` — Runtime path. This is where PID files and/or similar temporary files are stored

  Default value: `$HOME/version-managers/NAME-version-manager-rs/NAME-version-manager-rs/$APP_VERSION/run`
* `--data-path <DATA_PATH>` — Data path. This is where the actual data is stored, e.g., the .db and WAL files

  Default value: `$HOME/version-managers/NAME-version-manager-rs/NAME-version-manager-rs/$APP_VERSION/data`
* `--bin-path <BIN_PATH>` — Binary path. Where the executable binary are located. Sometimes called PREFIX

  Default value: `$HOME/version-managers/NAME-version-manager-rs/NAME-version-manager-rs/$APP_VERSION/bin`
* `--logs-path <LOGS_PATH>` — Logs path. Where the log files are to be stored

  Default value: `$HOME/version-managers/NAME-version-manager-rs/NAME-version-manager-rs/$APP_VERSION/logs`
* `--locale <LOCALE>` — Locale to use

  Default value: `en_US.UTF-8`
* `--markdown-help` — Markdown help generator. Only really used to generate replacement README.md files



## `NAME-version-manager-rs download`

Download specified version

**Usage:** `NAME-version-manager-rs download [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



## `NAME-version-manager-rs download-plan`

Resolve download URL and hash/checksum. Useful for security and concurrency.

**Usage:** `NAME-version-manager-rs download-plan [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



## `NAME-version-manager-rs env`

Print out associated environment variables

**Usage:** `NAME-version-manager-rs env`



## `NAME-version-manager-rs install`

Install specified version

**Usage:** `NAME-version-manager-rs install [VERSION] [SKIP_DEPENDENCIES]...`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var
* `<SKIP_DEPENDENCIES>` — dependencies to skip installation of, defaults to install all. Skip all with *



## `NAME-version-manager-rs install-dependencies`

Install (only) dependencies for specified version

**Usage:** `NAME-version-manager-rs install-dependencies [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



## `NAME-version-manager-rs ls`

List what versions are installed

**Usage:** `NAME-version-manager-rs ls`



## `NAME-version-manager-rs ls-remote`

List what versions are available

**Usage:** `NAME-version-manager-rs ls-remote`



## `NAME-version-manager-rs uri`

Print out database connection string

**Usage:** `NAME-version-manager-rs uri`



## `NAME-version-manager-rs service`

Service management

**Usage:** `NAME-version-manager-rs service <COMMAND>`

###### **Subcommands:**

* `install` — Install service (daemon), e.g., systemd, OpenRC, windows-service
* `reload` — Reload specified version
* `start` — Start specified version
* `stop` — Stop specified version



## `NAME-version-manager-rs service install`

Install service (daemon), e.g., systemd, OpenRC, windows-service

**Usage:** `NAME-version-manager-rs service install
install <COMMAND>`

###### **Subcommands:**

* `open-rc` — Install OpenRC service
* `systemd` — Install systemd service
* `windows-service` — Install Windows Service



## `NAME-version-manager-rs service install open-rc`

Install OpenRC service

**Usage:** `NAME-version-manager-rs service install open-rc [OPTIONS]`

###### **Options:**

* `--group <GROUP>` — user group to run service as

  Default value: `NAME-version-manager-rs`
* `--config-install-path <CONFIG_INSTALL_PATH>` — where to install the config file

  Default value: `/etc/conf.d/NAME-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>` — where to install the service file

  Default value: `/etc/init.d/NAME-version-manager-rs`
* `--user <USER>` — user to run service as

  Default value: `NAME-version-manager-rs`



## `NAME-version-manager-rs service install systemd`

Install systemd service

**Usage:** `NAME-version-manager-rs service install systemd [OPTIONS]`

###### **Options:**

* `--group <GROUP>` — user group to run service as

  Default value: `NAME-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>` — where to install the service file

  Default value: `/etc/systemd/system/NAME-version-manager-rs.service`
* `--user <USER>` — user to run service as

  Default value: `NAME-version-manager-rs`



## `NAME-version-manager-rs service install windows-service`

Install Windows Service

**Usage:** `NAME-version-manager-rs service install windows-service [OPTIONS]`

###### **Options:**

* `--service-name <SERVICE_NAME>` — name of service

  Default value: `NAME-version-manager-rs`
* `--service-description <SERVICE_DESCRIPTION>` — description of service

  Default value: ``



## `NAME-version-manager-rs service reload`

Reload specified version

**Usage:** `NAME-version-manager-rs service reload [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



## `NAME-version-manager-rs service start`

Start specified version

**Usage:** `NAME-version-manager-rs service start [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



## `NAME-version-manager-rs service stop`

Stop specified version

**Usage:** `NAME-version-manager-rs service stop [VERSION]`

###### **Arguments:**

* `<VERSION>` — version to install, defaults to global arg if provided otherwise env var



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
