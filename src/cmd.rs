use clap::Parser;
use regex::Regex;

/// Convert Windows path to WSL path and vice versa
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Convert WSL path to Windows path
    #[arg(short, long)]
    pub reverse: bool,

    /// Paths to convert
    #[arg(value_name = "paths")]
    pub paths: Vec<String>,
}

pub fn run() {
    let args = Args::parse();
    if args.reverse {
        // /mnt/x/path/to -> x:\path\to
        let re = Regex::new(r"/mnt/([a-z])").unwrap();
        for path in args.paths {
            let path = re.replace_all(&path, "$1:").to_string();
            let path = path.replace("/", r"\");
            println!("{}", path);
        }
    } else {
        // x:\path\to -> /mnt/x/path/to
        for path in args.paths {
            let path = path.replace(r"\", "/");
            if let Some((drive, path_to)) = path.split_once(":") {
                println!("/mnt/{}{}", drive.to_lowercase(), path_to)
            } else {
                println!("{}", path)
            }
        }
    }
}
