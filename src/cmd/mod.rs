use heapless::Vec;

use crate::println;
const MAX_COMMAND: usize = 20;

struct Cmd<'a> {
    name: &'a str,
    description: &'a str,
    callback: fn(&[&str]),
}

pub struct CommandLine<'a> {
    list: Vec<Cmd<'a>, MAX_COMMAND>,
}

impl<'a> CommandLine<'a> {
    pub fn new() -> Self {
        CommandLine { list: Vec::new() }
    }

    pub fn add_command(&mut self, name: &'a str, description: &'a str, callback: fn(&[&str])) {
        let cmd = Cmd {
            name,
            callback,
            description,
        };
        self.list.push(cmd).ok();
    }

    // Execute the command if it matches the input string and pass the arguments
    pub fn execute(&mut self, index: usize, input: &str) {
        let mut parts = input.split_whitespace();
        // The first part is the command name
        parts.next();
        // collect all arguments (maximum is 10)
        let args: Vec<&str, 10> = parts.collect();

        if let Some(cmd) = self.list.get_mut(index) {
            (cmd.callback)(&args); // Call the function pointer with arguments
        }
    }

    // Check if the input string matches any command in the list
    pub fn check_command(&mut self, input: &str) -> Option<usize> {
        // Split the input into command and arguments
        let mut parts = input.split_whitespace();
        let command_name = parts.next(); // The first part is the command name
        if let Some(cmd_name) = command_name {
            // Iterate over the command list and check for a match
            for (index, cmd) in self.list.iter().enumerate() {
                if cmd.name == cmd_name {
                    return Some(index); // Return the index and arguments
                }
            }
        }
        None // Return None if no command matches
    }

    // Print all available commands and their descriptions
    pub fn help_command(&self) {
        println!("List of available commands:");
        for cmd in self.list.iter() {
            println!("\t{} - {}", cmd.name, cmd.description);
        }
    }
}
