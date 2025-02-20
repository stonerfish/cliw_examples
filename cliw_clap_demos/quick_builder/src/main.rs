use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    match command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .try_get_matches_from(cliw::args_os())
    {
        Ok(matches) => {
            let mut msg = String::new();
            // You can check the value provided by positional arguments, or option arguments
            if let Some(name) = matches.get_one::<String>("name") {
                msg.push_str(&format!("Value for name: {name}\n"));
            }

            if let Some(config_path) = matches.get_one::<PathBuf>("config") {
                msg.push_str(&format!("Value for config: {}\n", config_path.display()));
            }

            // You can see how many times a particular flag or argument occurred
            // Note, only flags can have multiple occurrences
            match matches
                .get_one::<u8>("debug")
                .expect("Counts are defaulted")
            {
                0 => msg.push_str(&format!("Debug mode is off\n")),
                1 => msg.push_str(&format!("Debug mode is kind of on\n")),
                2 => msg.push_str(&format!("Debug mode is on\n")),
                _ => msg.push_str(&format!("Don't be crazy\n")),
            }

            // You can check for the existence of subcommands, and if found use their
            // matches just as you would the top level cmd
            if let Some(matches) = matches.subcommand_matches("test") {
                // "$ myapp test" was run
                if matches.get_flag("list") {
                    // "$ myapp test -l" was run
                    msg.push_str(&format!("Printing testing lists...\n"));
                } else {
                    msg.push_str(&format!("Not printing testing lists...\n"));
                }
            }

            cliw::output::print(&msg);
        }
        Err(err) => {
            // jump through some hoops to get proper color and error direction on native
            #[cfg(not(target_arch = "wasm32"))]
            {
                err.print().unwrap();
            }
            // and this hoop to get output for web/wasm
            #[cfg(target_arch = "wasm32")]
            {
                let msg = format!("{err}");
                cliw::output::eprint(&msg);
            }
        }
    }

    // Continued program logic goes here...
}
