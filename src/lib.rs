use std::env::{self};

pub fn read_env() -> Result<Parser, String> {
    let mut args = env::args();
    let path = args.next().unwrap();
    let mut parser = Parser::new(path);
    let mut current_command: Option<Command> = None;
    for arg in args {
        if arg.starts_with("--") {
            if current_command.is_none() {
                return Err(format!("子命令不能独立存在: {}", arg));
            }
            let command = current_command.clone().unwrap();
            let command = parser.get_command_by_name(command.command).unwrap();
            let sub = arg.strip_prefix("--").unwrap().to_string();
            let split = sub.split_once(&[':', '=']);
            match split {
                Some((name, args)) => {
                    let sub_command_option = command.get_sub_command_by_name(name.to_string());
                    match sub_command_option {
                        Some(sub_command) => {
                            sub_command.add_arg(args.to_string());
                        }
                        None => {
                            let mut sub_command = SubCommand::new(name.to_string());
                            sub_command.add_arg(args.to_string());
                            command.add_sub_command(sub_command);
                        }
                    }
                }
                None => {
                    // 无参数的子命令
                    let sub_command_option = command.get_sub_command_by_name(sub.to_string());
                    match sub_command_option {
                        Some(_sub_command) => {}
                        None => {
                            let sub_command = SubCommand::new(sub.to_string());
                            command.add_sub_command(sub_command);
                        }
                    }
                }
            }
        } else if arg.starts_with("-") {
            let name = arg.strip_prefix("-").unwrap().to_string();
            let command = Command::new(name);
            current_command = Some(command.clone());
            parser.add_command(command);
        } else {
            match current_command.clone() {
                Some(command) => {
                    let command = parser.get_command_by_name(command.command).unwrap();
                    command.add_arg(arg);
                }
                None => {
                    parser.add_arg(arg);
                }
            }
        }
    }
    Ok(parser)
}

#[derive(Debug)]
pub struct Parser {
    path: String,
    command: Vec<Command>,
    args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Command {
    command: String,
    sub_command: Vec<SubCommand>,
    args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SubCommand {
    command: String,
    args: Vec<String>,
}

impl Parser {
    fn new(path: String) -> Self {
        Parser {
            path: path,
            command: Vec::new(),
            args: Vec::new(),
        }
    }

    fn add_command(&mut self, command: Command) {
        self.command.push(command);
    }

    fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    fn get_command_by_name(&mut self, name: String) -> Option<&mut Command> {
        self.command
            .iter_mut()
            .find(|command| command.command == name)
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_command(&self) -> &Vec<Command> {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

impl Command {
    fn new(command: String) -> Self {
        Command {
            command,
            sub_command: Vec::new(),
            args: Vec::new(),
        }
    }

    fn add_sub_command(&mut self, sub_command: SubCommand) {
        self.sub_command.push(sub_command);
    }

    fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    fn get_sub_command_by_name(&mut self, name: String) -> Option<&mut SubCommand> {
        self.sub_command.iter_mut().find(|sub| sub.command == name)
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_sub_command(&self) -> &Vec<SubCommand> {
        &self.sub_command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

impl SubCommand {
    fn new(command: String) -> Self {
        SubCommand {
            command,
            args: Vec::new(),
        }
    }

    fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}
