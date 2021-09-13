pub mod database;
pub mod command;
pub mod utils;
pub mod todo;
pub mod workspace;
pub mod cli;
pub mod sys;
mod ui {
    pub mod todolist;
    pub mod view_list;
}

use structopt::StructOpt;
use std::error::Error;
use ui::todolist::TodoList;
use ui::view_list::show_list;
use cli::{ Cli, WorkspaceCommand };

fn main() -> Result<(), Box<dyn Error>> {

    sys::initialize();

    let mut db_filename = match workspace::get_workspace() {
        Ok(workspace_name) => database::get_db_filename_from_workspace_name(workspace_name),
        _ => database::get_db_filename_from_workspace_name(String::from(""))
    };

    match StructOpt::from_args() {
        Cli::Workspace(ws_command) => {
            match ws_command {
                WorkspaceCommand::Set { workspace_name} => {
                    workspace::set_workspace(&workspace_name);
                    db_filename = database::get_db_filename_from_workspace_name(workspace_name);
                },
                WorkspaceCommand::Unset => {
                    workspace::unset_workspace();
                    db_filename = String::from("db.json");
                },
                WorkspaceCommand::List => {
                    let entries = workspace::list_workspaces().expect("Error listing workspaces");
                    entries
                        .into_iter()
                        .for_each(|e| {
                            println!("{}", e);
                        });
                    return Ok(());
                },
                WorkspaceCommand::Remove { workspace_name } => {
                    workspace::remove_workspace(&workspace_name);
                }
            }
        },
        Cli::Add {todo_name} => {
            command::add(todo_name, &db_filename);
            return Ok(());
        }
        _ => println!("Continuing to list")
    }

    let db = database::read(&db_filename);
    let mut todo_list = TodoList::new(db);
    todo_list.items.state.select(Some(0));

    return show_list(todo_list, db_filename);
}
