use std::env;

mod event;
mod parser;
mod report;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Insufficient arguments")
    } else {
        let Ok(report) = report::group_game_data(&args[1]) else {
            return Err("Error parsing file");
        };
        println!("{}", report);
        Ok(())
    }
}
