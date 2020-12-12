// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

#[derive(Debug, PartialEq)]
pub enum Type {
    Set,
    Get,
    Update,
    Delete,
    Ping,
    Exit,
    Unknown,
}

#[derive(Debug)]
pub struct Command {
    key: String,
    value: String,
    expire: i64,
    name: Type,
}

impl Command {
    pub fn new<S: Into<String>>(key: S, value: S, expire: i64, name: Type) -> Command {
        Command {
            key: key.into(),
            value: value.into(),
            expire: expire,
            name: name,
        }
    }

    pub fn from_str<S: Into<String>>(cmd: S) -> Result<Command, String> {
        let name_val: Type;

        let cmd_str = cmd.into();

        let command = cmd_str.trim().to_string();
        let mut items: Vec<&str> = command.split(' ').collect();

        // Match the command
        match items[0].to_uppercase().as_str() {
            "SET" => {
                name_val = Type::Set;
            }
            "GET" => {
                name_val = Type::Get;
            }
            "UPDATE" => {
                name_val = Type::Update;
            }
            "DELETE" => {
                name_val = Type::Delete;
            }
            "PING" => {
                name_val = Type::Ping;
            }
            "EXIT" => {
                name_val = Type::Exit;
            }
            _ => {
                name_val = Type::Unknown;
            }
        }

        // If an invalid command, raise an error
        if name_val == Type::Unknown {
            return Err(format!(
                "Invalid command name `{name_val}`",
                name_val = items[0]
            ));
        }

        // Fix items
        while items.len() < 4 {
            items.push("");
        }

        if items[3] == "" {
            items[3] = "0"
        }

        if ((name_val == Type::Get) || (name_val == Type::Delete)) && (items[1] == "") {
            return Err(format!("Invalid command {cmd}", cmd = cmd_str));
        }

        if ((name_val == Type::Set) || (name_val == Type::Update))
            && ((items[1] == "") || (items[2] == ""))
        {
            return Err(format!("Invalid command {cmd}", cmd = cmd_str));
        }

        Ok(Command::new(
            items[1],
            items[2],
            items[3].to_string().parse::<i64>().unwrap(),
            name_val,
        ))
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_expire(&self) -> &i64 {
        &self.expire
    }

    pub fn get_name(&self) -> &Type {
        &self.name
    }

    pub fn set_key<S: Into<String>>(&mut self, key: S) {
        self.key = key.into()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = value.into()
    }

    pub fn set_expire(&mut self, expire: i64) {
        self.expire = expire
    }

    pub fn set_name(&mut self, name: Type) {
        self.name = name
    }
}

#[test]
fn test_set_command() {
    let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

    cmd.set_key("item1");
    cmd.set_value("value1");
    cmd.set_expire(0);
    cmd.set_name(Type::Set);

    assert_eq!(*cmd.get_key(), "item1".to_string());
    assert_eq!(*cmd.get_value(), "value1".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Set);

    // Test `SET $key $value $expire` command
    match Command::from_str("SET item2 value2") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "item2".to_string());
    assert_eq!(*cmd.get_value(), "value2".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Set);

    // Test `SET $key $value $expire` command
    match Command::from_str("SET item2 value2 160") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "item2".to_string());
    assert_eq!(*cmd.get_value(), "value2".to_string());
    assert_eq!(*cmd.get_expire(), 160);
    assert_eq!(*cmd.get_name(), Type::Set);
}

#[test]
fn test_get_command() {
    let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

    // Test `GET $key` command
    match Command::from_str("GET item2") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "item2".to_string());
    assert_eq!(*cmd.get_value(), "".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Get);
}

#[test]
fn test_delete_command() {
    let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

    // Test `DELETE $key` command
    match Command::from_str("DELETE item2") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "item2".to_string());
    assert_eq!(*cmd.get_value(), "".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Delete);
}

#[test]
fn test_ping_command() {
    let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

    // Test `PING` command
    match Command::from_str("PING") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "".to_string());
    assert_eq!(*cmd.get_value(), "".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Ping);
}

#[test]
fn test_exit_command() {
    let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

    // Test `EXIT` command
    match Command::from_str("EXIT") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "".to_string());
    assert_eq!(*cmd.get_value(), "".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Exit);

    // Test `exit` command
    match Command::from_str("exit") {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "".to_string());
    assert_eq!(*cmd.get_value(), "".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Exit);
}

#[test]
fn test_error1_command() {
    let mut err: String = String::from("");

    match Command::from_str("inver") {
        Ok(_) => {}
        Err(e) => err = e,
    }

    assert_eq!(err, "Invalid command name `inver`".to_string());
}

#[test]
fn test_error2_command() {
    let mut err: String = String::from("");

    // Test `EXIT` command
    match Command::from_str("get ") {
        Ok(_) => {}
        Err(e) => err = e,
    }

    assert_eq!(err, "Invalid command get ".to_string());
}

#[test]
fn test_error3_command() {
    let mut err: String = String::from("");

    // Test `EXIT` command
    match Command::from_str("update gs") {
        Ok(_) => {}
        Err(e) => err = e,
    }

    assert_eq!(err, "Invalid command update gs".to_string());
}
