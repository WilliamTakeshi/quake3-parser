mod parser;

fn main() {
    let input = " 25:52 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
";
    match parser::parse_kill_log(input) {
        Ok((_, log)) => {
            println!("Killer: {}", log.killer);
            println!("Killed: {}", log.killed);
            println!("Cause: {}", log.cause);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}