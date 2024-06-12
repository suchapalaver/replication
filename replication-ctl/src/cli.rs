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
            Arg::new("models")
                .long("models")
                .num_args(2)
                .default_values(["./sdxl.yaml", "./llava.yaml"])
                .help("Input configurations, including prompts, models, and other parameters. See sdxl.yaml and llava.yaml for examples"),
        )
}
