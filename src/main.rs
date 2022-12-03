use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Chobs",
    about = "Chobs (Changes Observer) is a tool that automatically restarting your process when file changes in the selected directory."
)]
struct Opt {
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(Debug, StructOpt)]
enum Mode {
    /// Start watching for changes
    Watch {
        /// Sets a command to execute
        #[structopt(short = "e", long = "exec")]
        exec: String,

        /// Sets a root folder to watch
        #[structopt(parse(from_os_str), short = "r", long = "root-foler")]
        root_folder: Option<PathBuf>,
    },
    /// Creates chobs.json config file
    Init,
}

fn main() {
    let opt = Opt::from_args();
    match opt.mode {
        Mode::Init => {
            chobs::init_config();
        }
        Mode::Watch { exec, root_folder } => {
            chobs::watch(root_folder, exec);
        }
    }
}
