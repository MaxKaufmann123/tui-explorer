use std::io::{self, Write};

pub enum Input {
    Choose(usize),
    DirName(String),
    NewDir(String),
    Rm(String),
    Quit,
}

impl Input {
    pub fn get_input() -> Result<Self, String> {
        print!("Wähle eine Option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input Error");

        match input.trim() {
            "q" => Ok(Self::Quit),
            "mkdir" => {
                input = String::from("");
                print!("Name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Input error");
                Ok(Self::NewDir(input.trim().to_string()))
            }
            "rm" => {
                input = String::from("");
                print!("Name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Input error");

                Ok(Self::Rm(input.trim().to_string()))
            }
            _ => {
                if let Ok(usize) = input.trim().parse() {
                    Ok(Self::Choose(usize))
                } else {
                    Ok(Self::DirName(input))
                }
            }
        }
    }
}
