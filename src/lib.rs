/// macros to create version manager CLIs.
/// depending on argument, it will create one of two variants:
/// 0. version manager for a package (mirror location, version, env vars, …)
/// 1. version manager for a service (daemon, port, hostname, …); also includes ↑

/*#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ROOT_DIR: std::path::PathBuf =
        std::path::Path::new(&std::env::home_dir).join("version-managers");
}*/

#[macro_export]
macro_rules! cli_struct_and_helpers {
    ($name:expr, $author:expr, $version:expr, $about:expr) => {
        use clap::{CommandFactory, Parser};
        use const_format::concatcp;

        const VM_ROOT_DEFAULT: &'static str =
            concatcp!("$HOME", std::path::MAIN_SEPARATOR_STR, "version-managers");
        const ROOT_DEFAULT: &'static str = concatcp!(
            VM_ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, $name);
        const VERSIONED_ROOT_DEFAULT: &'static str = concatcp!(
            ROOT_DEFAULT,
            std::path::MAIN_SEPARATOR_STR,
            $name,
            std::path::MAIN_SEPARATOR_STR,
            "$APP_VERSION"
        );

        #[derive(clap::Parser, serde::Serialize, serde::Deserialize, std::fmt::Debug)]
        #[command(name = $name)]
        #[command(author = $author)]
        #[command(version = $version)]
        #[command(about = $about, long_about = None)]
        struct Cli {
            /// Config file to read from. If provided used as new default (before env and argv res).
            #[serde(skip)]
            #[arg(long, env = "VMS_CONFIG",
                  default_value_os_t = std::ffi::OsString::from(
                      concatcp!(ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, "vms-config.json")))]
            vms_config: std::ffi::OsString,

            /// Whether to read from config file. If vms_config provided, this defaults to  `true` .
            #[arg(long, env = "VMS_CONFIG_READ", default_value_t = false)]
            config_read: bool,

            /// Whether to write to config file.
            #[arg(long, env = "VMS_CONFIG_WRITE", default_value_t = true)]
            config_write: bool,

            #[serde(skip, default = "_default_command")]
            #[command(subcommand)]
            command: Commands,

            /// Desired version of application.
            #[arg(long, env = "APP_VERSION", default_value_t = String::from("latest"))]
            app_version: String,

            /// root directory for all version-managers. For download cache and interdependencies.
            #[arg(long, env = "VM_ROOT",
                  default_value_os_t = std::ffi::OsString::from(VM_ROOT_DEFAULT))]
            vm_root: std::ffi::OsString,

            /// Root directory. By default all paths are relative to this one.
            #[arg(long, env = "ROOT", default_value_os_t = std::ffi::OsString::from(ROOT_DEFAULT))]
            root: std::ffi::OsString,

            /// Hostname of server.
            #[arg(long, env = "HOSTNAME", default_value_t = String::from("localhost"))]
            hostname: String,

            /// Port for server to listen on.
            #[arg(short, long, env = "PORT")]
            port: u16,

            /// Database name.
            #[arg(long, env = "DATABASE", default_value_t = String::from("database"))]
            database: String,

            /// Runtime path. This is where PID files and/or similar temporary files are stored.
            #[arg(long, env = "RUNTIME_PATH", default_value_os_t = std::ffi::OsString::from(
                      concatcp!(VERSIONED_ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, "run")))]
            runtime_path: std::ffi::OsString,

            /// Data path. This is where the actual data is stored, e.g., the .db and WAL files.
            #[arg(long, env = "DATA_PATH", default_value_os_t = std::ffi::OsString::from(
                      concatcp!(VERSIONED_ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, "data")))]
            data_path: std::ffi::OsString,

            /// Binary path. Where the executable binary are located. Sometimes called PREFIX.
            #[arg(long, env = "BIN_PATH", default_value_os_t = std::ffi::OsString::from(
                       concatcp!(VERSIONED_ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, "bin")))]
            bin_path: std::ffi::OsString,

            /// Logs path. Where the log files are to be stored.
            #[arg(long, env = "LOGS_PATH", default_value_os_t = std::ffi::OsString::from(
                       concatcp!(VERSIONED_ROOT_DEFAULT, std::path::MAIN_SEPARATOR_STR, "logs")))]
            logs_path: std::ffi::OsString,

            /// Locale to use.
            #[arg(long, env = "LC_ALL", default_value_t = String::from("en_US.UTF-8"))]
            locale: String,

            /// Markdown help generator. Only really used to generate replacement README.md files.
            #[arg(long, hide = true)]
            markdown_help: bool,
        }

        /// default command; needed for serde (even though serde ignores this field)
        fn _default_command() -> Commands {
            Commands::Uri {}
        }

        #[derive(clap::Subcommand, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
        enum Commands {
            /// Download specified version
            Download { version: Option<String> },

            /// Print out associated environment variables
            Env {},

            /// Install specified version
            Install { version: Option<String> },

            /// Install (only) dependencies for specified version
            InstallDependencies { version: Option<String> },

            /// Install service (daemon), e.g., systemd, OpenRC, windows-service
            InstallService(InstallService),

            /// List what versions are installed
            Ls {},

            /// List what versions are available
            LsRemote {},

            /// Reload specified version
            Reload { version: Option<String> },

            /// Start specified version
            Start { version: Option<String> },

            /// Stop specified version
            Stop { version: Option<String> },

            /// Print out database connection string
            Uri {},
        }

        #[derive(std::fmt::Debug, clap::Args, serde::Serialize, serde::Deserialize)]
        #[command(args_conflicts_with_subcommands = true)]
        struct InstallService {
            /// Install service (daemon), e.g., systemd, OpenRC, windows-service
            #[command(subcommand)]
            command: InstallServiceCommands,
        }

        #[derive(std::fmt::Debug, clap::Subcommand, serde::Serialize, serde::Deserialize)]
        enum InstallServiceCommands {
            /// Install OpenRC service
            OpenRc {
                /// user group to run service as
                #[arg(long, env = "GROUP", default_value_t = String::from($name))]
                group: String,

                /// where to install the config file
                #[arg(long, env = "CONFIG_INSTALL_PATH",
                      default_value_os_t = std::ffi::OsString::from(concat!("/etc/conf.d/", $name))
                     )]
                config_install_path: std::ffi::OsString,

                /// where to install the service file
                #[arg(long, env = "SERVICE_INSTALL_PATH",
                      default_value_os_t = std::ffi::OsString::from(concat!("/etc/init.d/", $name))
                     )]
                service_install_path: std::ffi::OsString,

                /// user to run service as
                #[arg(long, env = "USER", default_value_t = String::from($name))]
                user: String,
            },

            /// Install systemd service
            Systemd {
                /// user group to run service as
                #[arg(long, env = "GROUP", default_value_t = String::from($name))]
                group: String,

                /// where to install the service file
                #[arg(long, env = "SERVICE_INSTALL_PATH",
                      default_value_os_t = std::ffi::OsString::from(concat!("/etc/systemd/system/",
                                                                            $name, ".service")))]
                service_install_path: std::ffi::OsString,

                /// user to run service as
                #[arg(long, env = "USER", default_value_t = String::from($name))]
                user: String,
            },

            /// Install Windows Service
            WindowsService {
                /// name of service
                #[arg(long, env = "SERVICE_NAME", default_value_t = String::from($name))]
                service_name: String,

                /// description of service
                #[arg(long, env = "SERVICE_DESCRIPTION", default_value_t = String::from($about))]
                service_description: String,
            },
        }

        /// Common errors
        pub(crate) mod errors {
            pub(crate) enum IoOrJsonError {
                Io { source: std::io::Error },
                SerdeJson { source: serde_json::Error },
            }

            impl std::fmt::Display for IoOrJsonError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::SerdeJson { source } => {
                            write!(f, "Serialiser/deserialiser failed: {}", source)
                        }
                        Self::Io { source } => write!(f, "Could not load config: {}", source),
                    }
                }
            }

            impl std::fmt::Debug for IoOrJsonError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::SerdeJson { source } => source.fmt(f),
                        Self::Io { source } => source.fmt(f),
                    }
                }
            }

            impl std::error::Error for IoOrJsonError {
                fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                    match self {
                        Self::Io { source } => Some(source),
                        Self::SerdeJson { source } => Some(source),
                    }
                }
            }

            impl From<std::io::Error> for IoOrJsonError {
                fn from(source: std::io::Error) -> Self {
                    Self::Io { source }
                }
            }

            impl From<serde_json::Error> for IoOrJsonError {
                fn from(source: serde_json::Error) -> Self {
                    Self::SerdeJson { source }
                }
            }
        }

        /// Default commands to use or copy the interface of for ones own implementation
        pub(crate) mod command {
            use crate::Cli;

            pub(crate) fn default_ls_command(args: &Cli) -> Result<(), std::io::Error> {
                let entries: Vec<std::path::PathBuf> = {
                    let mut _entries = std::fs::read_dir(&args.root)?
                        .map(|res| res.map(|e| e.path()))
                        .collect::<Result<Vec<_>, std::io::Error>>()?;
                    _entries.sort();
                    _entries
                };
                print!("{:?}", entries);
                Ok(())
            }
        }

        /// Config helper functions
        pub(crate) mod config {
            use crate::errors::IoOrJsonError;
            use crate::{Cli, Commands};

            pub(crate) fn should_write_to_config(args: &Cli) -> bool {
                if !args.config_write {
                    return false;
                }
                match &args.command {
                    Commands::Ls {}
                    | Commands::LsRemote {}
                    | Commands::Uri {}
                    | Commands::Reload { version: _ }
                    | Commands::Stop { version: _ } => false,
                    _ => true,
                }
            }

            pub(crate) fn config_file_write(args: &Cli) -> Result<(), IoOrJsonError> {
                serde_json::to_writer(
                    std::fs::File::create(&args.vms_config).map_err(IoOrJsonError::from)?,
                    &args,
                )
                .map_err(IoOrJsonError::from)
            }

            pub(crate) fn maybe_config_file_write(args: &Cli) -> Result<(), IoOrJsonError> {
                if config::should_write_to_config(&args) {
                    config::config_file_write(&args)
                } else {
                    Ok(())
                }
            }

            pub(crate) fn maybe_config_from_file(args: &mut Cli) -> Result<Option<Cli>, IoOrJsonError> {
                let config_path = std::path::Path::new(&args.vms_config);
                if !args.config_read {
                    return Ok(None);
                } else if args.vms_config.is_empty()
                    || config_path.components().next()
                        == Some(std::path::Component::Normal(std::ffi::OsStr::new("$HOME")))
                {
                    args.config_read = false;
                    return Ok(None);
                } else if !config_path.is_file() {
                    return Ok(None);
                }
                println!("reading from config file {:?}", args.vms_config);
                return serde_json::from_reader(
                    std::fs::File::open(config_path).map_err(IoOrJsonError::from)?,
                )
                .map(Some)
                .map_err(IoOrJsonError::from);
            }
        }
    };
}
