
pub struct Config {
    pub pattern: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        
        Ok(
            Config{
                pattern: args[1].to_string(),
                filename: args[2].to_string()
            }
        )
    }
}