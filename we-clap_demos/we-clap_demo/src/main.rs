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

fn main() {
    let opts = Opts::parse();

    // Note : we use cliw for output in this example.
    let msg = format!("{opts:?}");
    cliw::output::print(&msg);
}
