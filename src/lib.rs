/*#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ROOT_DIR: std::path::PathBuf =
        std::path::Path::new(&std::env::home_dir).join("version-managers");
}*/

#[macro_export]
macro_rules! cli_struct {
    ($name:expr, $author:expr, $version:expr, $about:expr) => {
        use clap::Parser;
        use std::path::PathBuf;

        #[derive(Parser)]
        #[command(name = $name)]
        #[command(author = $author)]
        #[command(version = $version)]
        #[command(about = $about, long_about = None)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,

            #[arg(long, env = "APP_VERSION", default_value_t = String::from("latest"))]
            appVersion: String,

            #[arg(long, env = "ROOT", default_value_t = String::from("$HOME/version-managers"))]
            root: String,
        }

        #[derive(Subcommand)]
        enum Commands {
            /// Download specified version
            Download { version: Option<String> },

            /// Print out associated environment variables
            Env { },

            /// Reload specified version
            Reload { version: Option<String> },

            /// Start specified version
            Start { version: Option<String> },

            /// Stop specified version
            Stop { version: Option<String> },

            /// Install specified version
            Install { version: Option<String> },

            /// List what versions are installed
            Ls { },

            /// List what versions are available
            LsRemote { },
        }
    };
}

/*

Storage layer specific CLI args:

/* global args */
#[arg(short, long, env = "PORT")]

port: u16,

/* subcommands */

/// Print out database connection string
uri {},

installService {
    /// Install OpenRC service
    openrc {},

    /// Install systemd service
    systemd  {},

    /// Install Windows Service
    windows-service {}
}
*/
