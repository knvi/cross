use functions::project;
use std::env;

// Modules
pub mod functions;
pub mod prompt;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        // Print help menu
    } else {
        let command = args[1].to_string();

        match command.as_str() {
            "create" => {
                if args.len() > 2 {
                    let subcommand = args[2].to_string();

                    match subcommand.as_str() {
                        "project" => project::handle_create_project(),
                        &_ => {
                            // Print help menu for subcommands
                        }
                    }
                } else {
                }
            }
            "init" => {}
            &_ => {
                // Command not found
            }
        }
    }
}
