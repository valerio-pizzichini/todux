use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
    Add {
        todo_name: String
    },
    List, 
    Workspace(WorkspaceCommand)
}

#[derive(StructOpt, Debug)]
pub enum WorkspaceCommand {
    Set {
        name: String
    },
    Unset,
    List,
    Remove {
        name: String
    }
}
