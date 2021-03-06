use clap::{Arg, Command};

mod configuration;
mod workspace;

fn main() {
    let command = Command::new("wk")
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
            .takes_value(true)
            .exclusive(true)
    )
    .arg(
        Arg::new("list")
            .short('l')
            .long("list")
            .help("List workspaces")
            .exclusive(true)
    );

    // TODO: Deal with panic errors.
    let matches = command.get_matches();
    if matches.is_present("workspace") {
        let workspace = matches.value_of("workspace").unwrap();
        workspace::Workspace::go(workspace)
    } else if matches.is_present("save") {
        let save = matches.values_of("save").unwrap().into_iter().collect::<Vec<&str>>();
        workspace::Workspace::save(save[0], save[1])
    } else if matches.is_present("list") {
        workspace::Workspace::list()
    } else {
        println!("{:#?}", matches)
    }
}
