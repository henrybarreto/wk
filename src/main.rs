use clap::{App, Arg};

mod configuration;

fn main() {
    let app = App::new("wk")
        .version("0.1.0")
        .about("WK is a CLI tool to create, manager and access workspaces")
        .author("Henry Barreto <me@henrybarreto.dev>")
    .arg(Arg::new("workspace")
        .help("Workspace's name")
        .index(1))
    .arg(
        Arg::new("save")
            .short('s')
            .long("save")
            .help("Save a workspace")
            .takes_value(true)
            .value_names(&["NAME", "PATH"])
            .exclusive(true)
    )
    .arg(
        Arg::new("list")
            .short('l')
            .long("list")
            .help("List workspaces")
            .exclusive(true)
    );

    let matches = app.get_matches();
    println!("{:?}", matches);

    unimplemented!();
}
