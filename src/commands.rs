use inquire::Confirm;

pub struct InitialCommand {}

impl InitialCommand {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        println!("Welcome to the modslink CLI tool!");
        println!(
            "In the next steps, you will enter several informations for your modslink configuration."
        );
        let start_ans = Confirm::new("Do you want to start the configuration process?").prompt();

        match start_ans {
            Ok(true) => {
                println!("Starting configuration process...");
                // TODO: Implement configuration process
            }
            Ok(false) => {
                println!("Configuration process aborted.");
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        Ok(())
    }
}
