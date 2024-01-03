use anyhow::Context;

use crate::macro_helper;
use crate::paint::Paintable;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;
pub const ID_SIZE: usize = macro_helper::field_size!(Row::id);
pub const USERNAME_SIZE: usize = macro_helper::field_size!(Row::username);
pub const EMAIL_SIZE: usize = macro_helper::field_size!(Row::email);
pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: [u8; COLUMN_USERNAME_SIZE],
    pub email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    pub fn serialize(&self, destination: [u8; ROW_SIZE]) -> [u8; ROW_SIZE] {
        let mut serialized_row = destination;
        serialized_row[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&self.id.to_le_bytes());

        serialized_row[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]
            .copy_from_slice(&self.username);

        serialized_row[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&self.email);

        serialized_row
    }

    pub fn deserialize(&self, source: [u8; ROW_SIZE]) -> anyhow::Result<Row> {
        let mut row = Row {
            id: 0,
            username: [0; COLUMN_USERNAME_SIZE],
            email: [0; COLUMN_EMAIL_SIZE],
        };

        row.id = u32::from_le_bytes(source[ID_OFFSET..ID_OFFSET + ID_SIZE].try_into().context(
            format!(
                "{}",
                "Failed to convert the id bytes into a u32 value.".error()
            ),
        )?);
        row.username
            .copy_from_slice(&source[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]);
        row.email
            .copy_from_slice(&source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE]);

        Ok(row)
    }

    pub fn username(&self) -> String {
        let mut username = String::new();
        for byte in &self.username {
            if *byte == 0 {
                break;
            }
            username.push(*byte as char);
        }
        username
    }

    pub fn email(&self) -> String {
        let mut email = String::new();
        for byte in &self.email {
            if *byte == 0 {
                break;
            }
            email.push(*byte as char);
        }
        email
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Row {{ id: {}, username: {}, email: {} }}",
            self.id,
            self.username(),
            self.email()
        )
    }
}
