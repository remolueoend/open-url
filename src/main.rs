use color_eyre::eyre::Result;
use eyre::ContextCompat;
use open_url::{cli, open_url};

extern crate loggerv;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = cli::get_interface().get_matches();
    loggerv::init_with_verbosity(args.occurrences_of("v"))?;
    let url = args.value_of("url").wrap_err("Missing argument <url>")?;
    open_url(&url.to_string())
}
