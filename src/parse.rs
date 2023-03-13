use clap::Parser;

pub struct ArgStruct {
    pub source_file: String,
    pub destination_file: String,
    pub from: String,
    pub to: String,
    pub remove_source: bool,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// source file name
    #[arg(short, long)]
    source_file: Option<String>,

    /// destination file name
    #[arg(short, long)]
    destination_file: Option<String>,

    /// from directory
    #[arg(short, long)]
    from: Option<String>,

    /// to directory
    #[arg(short, long)]
    to: Option<String>,

    /// remove source file
    #[arg(short, long)]
    remove_source: bool,
}

pub fn parser() -> ArgStruct {
    let args = Args::parse();
    let mut arg_struct = ArgStruct {
        source_file: String::new(),
        destination_file: String::new(),
        from: String::new(),
        to: String::new(),
        remove_source: false,
    };

    if let Some(source_file) = args.source_file {
        arg_struct.source_file = source_file;
    }

    if let Some(destination_file) = args.destination_file {
        arg_struct.destination_file = destination_file;
    }

    if let Some(from) = args.from {
        arg_struct.from = from;
    }

    if let Some(to) = args.to {
        arg_struct.to = to;
    }

    arg_struct.remove_source = args.remove_source;

    arg_struct
}
