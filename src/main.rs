use std::process::exit;

use clap::{Arg, Command, ValueHint};

mod configuration;
mod workspace;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Henry Barreto <me@henrybarreto.dev>")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("go").about("Go to a workspace").arg(
                Arg::new("name")
                    .help("The name of the workspace")
                    .required(true)
                    .value_name("NAME")
                    .value_hint(ValueHint::Other)
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
                        .value_name("NAME")
                        .value_hint(ValueHint::Other)
                        .index(1),
                )
                .arg(
                    Arg::new("path")
                        .help("The path of the workspace")
                        .required(true)
                        .value_name("PATH")
                        .value_hint(ValueHint::DirPath)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("remove").about("Remove a workspace").arg(
                Arg::new("name")
                    .help("The name of the workspace")
                    .required(true)
                    .value_name("NAME")
                    .value_hint(ValueHint::Other)
                    .index(1),
            ),
        )
        .subcommand(Command::new("list").about("List workspaces"))
        .get_matches();

    match matches.subcommand() {
        Some(("go", args)) => {
            let name = args.value_of("name").unwrap();

            if let Ok(workspace) = workspace::get(name) {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("cd {} && exec $SHELL", workspace.path))
                    .spawn()
                    .expect("Failed to execute process")
                    .wait_with_output()
                    .unwrap();

                println!("Exited from {}", workspace.name);

                exit(0);
            } else {
                println!("Workspace not found");

                exit(1);
            };
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
            if let Err(err) = workspace::list() {
                println!("{}", err);

                exit(1);
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
