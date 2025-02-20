fn main() {
    //  This is an example using raw cliw without using clap
    let args = cliw::args_os();
    let args: Vec<_> = args.collect();
    let msg = format!("{:?}", args);
    cliw::output::print(&msg);
}
