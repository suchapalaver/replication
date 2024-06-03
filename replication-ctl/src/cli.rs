use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("replication-ctl")
        .about("replication client")
        .arg(
            Arg::new("port")
                .long("port")
                .num_args(1)
                .default_value("50051")
                .help("Port on which to listen"),
        )
        .arg(
            Arg::new("intent")
                .long("intent")
                .num_args(1)
                .required(true)
                .help("Message to be passed to the model"),
        )
}
