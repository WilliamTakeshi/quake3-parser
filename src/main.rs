use std::env;

mod event;
mod parser;
mod report;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Insufficient arguments")
    } else {
        let file = std::fs::read_to_string(&args[1]).map_err(|_| "Cannot read file")?;
        let Ok(report) = report::group_game_data(file) else {
            return Err("Error parsing file");
        };
        let Ok(json) = serde_json::to_string_pretty(&report) else {
            return Err("Error serializing report");
        };
        println!("{}", json);
        Ok(())
    }
}
