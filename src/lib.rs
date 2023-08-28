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
macro_rules! cli_struct {
    ($name:expr, $author:expr, $version:expr, $about:expr) => {
        use clap::{Args, Parser, Subcommand};
        use const_format::concatcp;

        #[derive(Parser)]
        #[command(name = $name)]
        #[command(author = $author)]
        #[command(version = $version)]
        #[command(about = $about, long_about = None)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,

            #[arg(long, env = "APP_VERSION", default_value_t = String::from("latest"))]
            app_version: String,

            #[arg(long, env = "ROOT", default_value_os_t = String::from(concatcp!(
                "$HOME", std::path::MAIN_SEPARATOR_STR, "version-managers",
                std::path::MAIN_SEPARATOR_STR, $name)))]
            root: String,

            #[arg(long, env = "HOSTNAME", default_value_t = String::from("localhost"))]
            hostname: String,

            #[arg(short, long, env = "PORT")]
            port: u16,

            #[arg(long, hide = true)]
            markdown_help: bool,
        }

        #[derive(Subcommand)]
        enum Commands {
            /// Download specified version
            Download {
                version: Option<String>,
            },

            /// Print out associated environment variables
            Env {},

            /// Install specified version
            Install {
                version: Option<String>,
            },

            /// List what versions are installed
            Ls {},

            /// List what versions are available
            LsRemote {},

            /// Reload specified version
            Reload {
                version: Option<String>,
            },

            /// Start specified version
            Start {
                version: Option<String>,
            },

            /// Stop specified version
            Stop {
                version: Option<String>,
            },

            /// Print out database connection string
            Uri {},

            /// Install service (daemon), e.g., systemd, OpenRC, windows-service
            InstallService(InstallService),
        }

        #[derive(Debug, Args)]
        #[command(args_conflicts_with_subcommands = true)]
        struct InstallService {
            #[command(subcommand)]
            command: InstallServiceCommands,
        }

        #[derive(Debug, Subcommand)]
        enum InstallServiceCommands {
            /// Install OpenRC service
            OpenRc {
                #[arg(long, env = "GROUP", default_value_t = String::from($name))]
                group: String,

                #[arg(long, env = "CONFIG_INSTALL_PATH", default_value_t = String::from(concat!("/etc/conf.d/", $name)))]
                configInstallPath: String,

                #[arg(long, env = "SERVICE_INSTALL_PATH", default_value_t = String::from(concat!("/etc/init.d/", $name)))]
                serviceInstallPath: String,

                #[arg(long, env = "USER", default_value_t = String::from($name))]
                user: String,
            },

            /// Install systemd service
            Systemd {
                #[arg(long, env = "GROUP", default_value_t = String::from($name))]
                group: String,

                #[arg(long, env = "SERVICE_INSTALL_PATH", default_value_t = String::from(concat!("/etc/systemd/system/", $name, ".service")))]
                serviceInstallPath: String,

                #[arg(long, env = "USER", default_value_t = String::from($name))]
                user: String,
            },

            /// Install Windows Service
            WindowsService {
                #[arg(long, env = "SERVICE_NAME", default_value_t = String::from($name))]
                serviceName: String,

                #[arg(long, env = "SERVICE_DESCRIPTION", default_value_t = String::from($about))]
                serviceDescription: String,
            },
        }
    };
}
