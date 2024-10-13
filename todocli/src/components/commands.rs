// use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// #[structopt(
//     name = "todocli",
//     about = "A simple CLI todo list manager",
//     author = "zhuima <zhuima314@gmail.com>",
//     version = "1.0",
//     long_about = "Todo CLI is a powerful command-line tool for managing your tasks efficiently."
// )]
// pub enum Command {
//     #[structopt(name = "add", about = "Add a new todo item")]
//     Add {
//         #[structopt(short, long, help = "Title of the todo item")]
//         title: String,
//         #[structopt(short, long, help = "Description of the todo item")]
//         description: Option<String>,
//     },
//     #[structopt(name = "list", about = "List all todo items")]
//     List,
//     #[structopt(name = "complete", about = "Mark a todo item as completed")]
//     Complete {
//         #[structopt(short, long, help = "ID of the todo item to mark as completed")]
//         id: u64,
//     },
//     #[structopt(name = "delete", about = "Delete a todo item")]
//     Delete {
//         #[structopt(short, long, help = "ID of the todo item to delete")]
//         id: u64,
//     },
// }
