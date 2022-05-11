use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    recursive: bool,

    #[clap(short, long)]
    force: bool,

    #[clap()]
    files: Vec<String>
}

fn rm(args: Args) -> std::io::Result<()> {
    for file in args.files {
        if args.recursive {
            if args.force {
                std::fs::remove_dir_all(file)?;
            } else {
                std::fs::remove_dir(file)?;
            }
        } else {
            std::fs::remove_file(file)?;
        }
    }

    return std::io::Result::Ok(())
}

fn main() {
    let args = Args::parse();
    Result::unwrap(rm(args)) 
}
