use crate::paint::Paintable;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;
// pub const ID_SIZE: usize = macro_helper::field_size!(Row::id);
// pub const USERNAME_SIZE: usize = macro_helper::field_size!(Row::username);
// pub const EMAIL_SIZE: usize = macro_helper::field_size!(Row::email);
// pub const ID_OFFSET: usize = 0;
// pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
// pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = 288; // ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug)]
pub struct Row {
    // for now, we are cheating and only allowing to create a row with
    // an id, a username and an email
    // the row size is 288 bytes
    pub data: [u8; ROW_SIZE],
}

impl Row {
    pub fn new() -> Self {
        Self {
            data: [0; ROW_SIZE],
        }
    }

    pub fn username(&self) -> String {
        let mut username = String::new();
        for byte in &self.data[1..33] {
            if *byte == 0 {
                break;
            }
            username.push(*byte as char);
        }
        username
    }

    pub fn email(&self) -> String {
        let mut email = String::new();
        for byte in &self.data[33..] {
            if *byte == 0 {
                break;
            }
            email.push(*byte as char);
        }
        email
    }

    pub fn print_header() {
        println!(
            "{}",
            format!("| {:<6} | {:<15} | {:<20} |", "id", "username", "email")
                .paint(yansi::Color::Cyan)
        );
    }

    pub fn print(&self) {
        println!(
            "{}",
            format!(
                "| {:<6} | {:<15} | {:<20} |",
                self.data[0],
                self.username(),
                self.email()
            )
            .paint(yansi::Color::Cyan)
        );
    }
}
