use crate::paint::Paintable;

// the only data we are storing in a row is an id, a username and an email
// this is a total of 288 bytes (1 + 32 + 255)
pub const ID_SIZE: usize = 1;
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;
pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug)]
pub struct Row {
    // for now, we are cheating and only allowing to create a row with
    // an id, a username and an email
    // the row size is 288 bytes
    pub data: [u8; ROW_SIZE],
}

impl Row {
    pub fn empty() -> Self {
        Self {
            data: [0; ROW_SIZE],
        }
    }

    pub fn new(id: &str, username: &str, email: &str) -> anyhow::Result<Self> {
        let mut data = [0; ROW_SIZE];
        data[ID_OFFSET] = id.parse::<u8>().unwrap();

        if username.len() > USERNAME_SIZE {
            return Err(anyhow::Error::msg(format!("Username is too long.")));
        }
        data[USERNAME_OFFSET..USERNAME_OFFSET + username.len()]
            .copy_from_slice(username.as_bytes());

        if email.len() > EMAIL_SIZE {
            return Err(anyhow::Error::msg(format!("Email is too long.")));
        }
        data[EMAIL_OFFSET..EMAIL_OFFSET + email.len()].copy_from_slice(email.as_bytes());

        Ok(Self { data })
    }

    pub fn id(&self) -> u8 {
        self.data[0]
    }

    pub fn username(&self) -> String {
        let mut username = String::new();
        for byte in &self.data[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE] {
            if *byte == 0 {
                break;
            }
            username.push(*byte as char);
        }
        username
    }

    pub fn email(&self) -> String {
        let mut email = String::new();
        for byte in &self.data[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE] {
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
                .paint(yansi::Color::Yellow)
        );
    }

    pub fn print(&self) {
        println!(
            "{}",
            format!(
                "| {:<6} | {:<15} | {:<20} |",
                self.id(),
                self.username(),
                self.email()
            )
            .paint(yansi::Color::Yellow)
        );
    }
}
