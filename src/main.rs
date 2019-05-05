#[macro_use]
pub mod error;

pub mod atdf;
pub mod chip;
pub mod elementext;
pub mod svd;
pub mod util;

pub use elementext::ElementExt;
pub use error::{Error, Result};

#[derive(Debug, structopt::StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    atdf_path: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    svd_path: Option<std::path::PathBuf>,

    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let args: Options = structopt::StructOpt::from_args();

    let atdf_file = std::fs::File::open(args.atdf_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let svd_file: Box<dyn std::io::Write> = if let Some(p) = args.svd_path {
        Box::new(std::fs::File::create(p).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }))
    } else {
        Box::new(std::io::stdout())
    };

    let chip = atdf::parse(atdf_file).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if args.debug {
        eprintln!("{:#?}", chip);
    }

    svd::generate(&chip, svd_file).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
