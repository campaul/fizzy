use clap::{App, AppSettings, Arg, SubCommand};

mod command;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Fizzy")
        .version(VERSION)
        .about("Read and modify old disk images")
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("info")
                .about("controls testing features")
                .arg(
                    Arg::with_name("image")
                        .required(true)
                        .help("print debug information verbosely"),
                )
                .arg(Arg::with_name("partition").required(false).help("foo")),
        )
        .get_matches();

    println!("{:?}", matches);
}
