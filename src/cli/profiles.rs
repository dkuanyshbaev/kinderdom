// use kinderdom::db::{create_profile, establish_connection};
// use std::env;
//
// fn help() {
//     println!("subcommands:");
//     println!("    new<name>: create a new task");
// }
//
// fn new_profile(args: &[String]) {
//     if args.len() < 1 {
//         println!("new: missing <name>");
//         help();
//         return;
//     }
//
//     let conn = establish_connection();
//     create_profile(&conn, &args[0]);
// }
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() < 2 {
//         help();
//         return;
//     }
//
//     let subcommand = &args[1];
//     match subcommand.as_ref() {
//         "new" => new_profile(&args[2..]),
//         _ => help(),
//     }
// }
//
//fn show_tasks(args: &[String]) {
//     if args.len() > 0 {
//         println!("show: unexpected argument");
//         help();
//         return;
//     }
//
//     let conn = establish_connection();
//     println!("TASKS\n-----");
//     for task in query_task(&conn) {
//         println!("{}", task.title);
//     }
// }
