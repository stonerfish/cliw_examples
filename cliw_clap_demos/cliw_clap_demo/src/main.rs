use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about)]
pub struct Opts {
    /// A required string
    #[arg(short, long)]
    pub words: String,

    /// An optional value
    #[arg(short, long)]
    pub value: Option<f32>,
}

// Using cliw with regular clap.  To get error and help output for the web,  you need to use
// try_parse_from() and handle the result.
//
// If we were to use a patch web-enabled clap the output for the web is automagic and you can use
// parse().   Note the simpler main that would be used with we-clap.
// fn main() {
//    // we-clap parse() will handle help and error output on native and web.
//    let opts = Opts::parse();
//
//    let  msg = format!("{opts:?}");
//    // we will use cliw for output to show the args
//    cliw::output::print(&msg);
// }
fn main() {
    let opts = Opts::try_parse_from(cliw::args_os());

    match opts {
        Ok(opts) => {
            let msg = format!("{opts:?}");
            println!("{msg}");
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
    };
}
