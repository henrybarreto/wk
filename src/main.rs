use clap::{Arg, Command, ValueHint};

mod configuration;
mod workspace;

fn main() {
    let matches = Command::new("wk")
        .version("0.3.0")
        .about("WK is a CLI tool to create, manager and access workspaces")
        .author("Henry Barreto <me@henrybarreto.dev>")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("go").about("Go to a workspace").arg(
                Arg::new("name")
                    .help("The name of the workspace")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("save")
                .about("Save a workspace")
                .arg(
                    Arg::new("name")
                        .help("The name of the workspace")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("path")
                        .help("The path of the workspace")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("remove").about("Remove a workspace").arg(
                Arg::new("name")
                    .help("The name of the workspace")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("list").about("List workspaces"))
        .get_matches();

    match matches.subcommand() {
        Some(("go", args)) => {
            let name = args.value_of("name").unwrap();
            workspace::go(name);
        }
        Some(("save", args)) => {
            let name = args.value_of("name").unwrap();
            let path = args.value_of("path").unwrap();
            workspace::save(name, path);
        }
        Some(("remove", args)) => {
            let name = args.value_of("name").unwrap();
            workspace::remove(name);
        }
        Some(("list", _)) => {
            workspace::list();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
