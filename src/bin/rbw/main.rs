mod actions;
mod commands;
mod sock;

fn main() {
    let matches = clap::App::new("rbw")
        .about("unofficial bitwarden cli")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .subcommand(
            clap::SubCommand::with_name("config")
                .subcommand(clap::SubCommand::with_name("show"))
                .subcommand(
                    clap::SubCommand::with_name("set")
                        .arg(clap::Arg::with_name("key").required(true))
                        .arg(clap::Arg::with_name("value").required(true)),
                ),
        )
        .subcommand(clap::SubCommand::with_name("login"))
        .subcommand(clap::SubCommand::with_name("unlock"))
        .subcommand(clap::SubCommand::with_name("sync"))
        .subcommand(clap::SubCommand::with_name("list"))
        .subcommand(
            clap::SubCommand::with_name("get")
                .arg(clap::Arg::with_name("name").required(true))
                .arg(clap::Arg::with_name("user")),
        )
        .subcommand(clap::SubCommand::with_name("add"))
        .subcommand(
            clap::SubCommand::with_name("generate")
                .arg(clap::Arg::with_name("len").required(true))
                .arg(clap::Arg::with_name("name"))
                .arg(clap::Arg::with_name("user"))
                .arg(clap::Arg::with_name("no-symbols").long("no-symbols"))
                .arg(
                    clap::Arg::with_name("only-numbers").long("only-numbers"),
                )
                .arg(
                    clap::Arg::with_name("nonconfusables")
                        .long("nonconfusables"),
                )
                .arg(clap::Arg::with_name("diceware").long("diceware"))
                .group(clap::ArgGroup::with_name("password-type").args(&[
                    "no-symbols",
                    "only-numbers",
                    "nonconfusables",
                    "diceware",
                ])),
        )
        .subcommand(clap::SubCommand::with_name("edit"))
        .subcommand(clap::SubCommand::with_name("remove"))
        .subcommand(clap::SubCommand::with_name("lock"))
        .subcommand(clap::SubCommand::with_name("purge"))
        .subcommand(clap::SubCommand::with_name("stop-agent"))
        .get_matches();

    match matches.subcommand() {
        ("config", Some(smatches)) => match smatches.subcommand() {
            ("show", Some(_)) => commands::config_show(),
            ("set", Some(ssmatches)) => commands::config_set(
                ssmatches.value_of("key").unwrap(),
                ssmatches.value_of("value").unwrap(),
            ),
            _ => {
                eprintln!("{}", smatches.usage());
                std::process::exit(1);
            }
        },
        ("login", Some(_)) => commands::login(),
        ("unlock", Some(_)) => commands::unlock(),
        ("sync", Some(_)) => commands::sync(),
        ("list", Some(_)) => commands::list(),
        ("get", Some(smatches)) => commands::get(
            smatches.value_of("name").unwrap(),
            smatches.value_of("user"),
        ),
        ("add", Some(_)) => commands::add(),
        ("generate", Some(smatches)) => {
            let ty = if smatches.is_present("no-symbols") {
                rbw::pwgen::Type::NoSymbols
            } else if smatches.is_present("only-numbers") {
                rbw::pwgen::Type::Numbers
            } else if smatches.is_present("nonconfusables") {
                rbw::pwgen::Type::NonConfusables
            } else if smatches.is_present("diceware") {
                rbw::pwgen::Type::Diceware
            } else {
                rbw::pwgen::Type::AllChars
            };
            commands::generate(
                smatches.value_of("name"),
                smatches.value_of("user"),
                smatches.value_of("len").unwrap().parse().unwrap(),
                ty,
            );
        }
        ("edit", Some(_)) => commands::edit(),
        ("remove", Some(_)) => commands::remove(),
        ("lock", Some(_)) => commands::lock(),
        ("purge", Some(_)) => commands::purge(),
        ("stop-agent", Some(_)) => commands::stop_agent(),
        _ => {
            eprintln!("{}", matches.usage());
            std::process::exit(1);
        }
    }
}