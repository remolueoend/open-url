use clap::{App, Arg};

pub const APP_NAME: &str = "open-url";

pub fn get_interface<'a, 'b>() -> App<'a, 'b> {
    App::new(APP_NAME)
        .version("1.0")
        .author("remolueoend")
        .about("open URLs with custom handlers")
        .arg(
            Arg::with_name("url")
                .index(1)
                .help("The URL to open")
                .required(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("sets the level of verbosity"),
        )
}
