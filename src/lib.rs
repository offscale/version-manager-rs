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

            #[arg(long, env = "ROOT", default_value_t = String::from("$HOME/version-managers"))]
            root: String,

            #[arg(short, long, env = "PORT")]
            port: u16,
        }

        #[derive(Subcommand)]
        enum Commands {
            /// Download specified version
            Download {
                version: Option<String>,
            },

            /// Print out associated environment variables
            Env {},

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

            /// Install specified version
            Install {
                version: Option<String>,
            },

            /// List what versions are installed
            Ls {},

            /// List what versions are available
            LsRemote {},

            /// Print out database connection string
            Uri {},

            InstallService(InstallService),
        }

        #[derive(Debug, Args)]
        #[command(args_conflicts_with_subcommands = true)]
        struct InstallService {
            #[command(subcommand)]
            command: InstallCommands,
        }

        #[derive(Debug, Subcommand)]
        enum InstallCommands {
            /// Install OpenRC service
            OpenRc {},

            /// Install systemd service
            Systemd {},

            /// Install Windows Service
            WindowsService {},
        }
    };
}
