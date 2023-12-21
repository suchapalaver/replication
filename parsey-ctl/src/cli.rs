use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("parsey-ctl")
        .about("parsey-ctl: rust-powered parsey client")
        .arg(
            Arg::new("port")
                .long("port")
                .num_args(1)
                .default_value("50051")
                .help("which port to listen on"),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .num_args(1)
                .default_value("table.pdf")
                .help("which file to use as data source"),
        )
}
