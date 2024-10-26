use std::path::PathBuf; 
use structopt::StructOpt; 

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to journal file
    Add {
        /// task desciption text
        #[structopt()]
        task: String
    }, 
    /// Remove an entry from journal file by position
    Done {
        #[structopt()]
        position: usize
    }, 
    /// list all tasks in journal file
    List, 
}   

#[derive(Debug, StructOpt)]
#[structopt(
    name="Rusty Journal (to-do-app)", 
    about="A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {

    #[structopt(subcommand)]
    pub action: Action, 

    /// use a diffferent journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>
}
