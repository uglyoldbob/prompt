//! A module for prompting a user to fill out a struct

pub use prompt_derive::Prompting;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// This is a type that allows a user to enter a passowrd without revealing that password onscreen.
pub struct Password(String);

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::ops::Deref for Password {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Password {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Prompting for Password {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        use std::io::Write;
        if let Some(n) = name {
            print!("{}: ", n);
            std::io::stdout().flush().unwrap();
        }
        let buffer = rpassword::read_password().unwrap();
        Ok(Password(buffer))
    }
}

impl Password {
    pub fn new(s: String) -> Self {
        Password(s)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// This is a type that allows a user to create a password, prompting them once for the password, and a second time to verify the password.
pub struct Password2(String);

impl From<Password2> for String {
    fn from(value: Password2) -> Self {
        value.0
    }
}

impl std::fmt::Display for Password2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::ops::Deref for Password2 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Password2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Prompting for Password2 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        use std::io::Write;
        let mut buffer;
        loop {
            if let Some(n) = name {
                print!("{}:Enter password:", n);
            } else {
                print!("Enter password:");
            }
            std::io::stdout().flush().unwrap();
            buffer = rpassword::read_password().unwrap();
            if let Some(n) = name {
                print!("{}: Enter password again:", n);
            } else {
                print!("Enter password again: ");
            }
            std::io::stdout().flush().unwrap();
            let buf2 = rpassword::read_password().unwrap();
            if buffer == buf2 {
                break;
            }
            println!("Passwords do not match, try again");
        }
        Ok(Password2(buffer))
    }
}

impl Password2 {
    pub fn new(s: String) -> Self {
        Password2(s)
    }
}

/// This trait is for types that can be created with user input
pub trait Prompter<T> {
    fn prompt(&mut self) -> Result<T, Error>;
}

/// The types of errors that can occur when prompting for user input.
#[derive(Debug)]
pub enum Error {
    /// Error in standard input
    InputError(std::io::Error),
    /// Error converting from string to the desired type
    ConversionError,
}

/// This trait is responsible for doing the work of prompting the user for input.
pub trait Prompting: Sized {
    /// prompt for input of the specified type.
    /// # Arguments
    /// * name - The optional name to display for the type
    fn prompt(name: Option<&str>) -> Result<Self, Error>;

    fn prompt_generic<T>(name: Option<&str>) -> Result<T, Error>
    where
        T: Prompting + core::str::FromStr,
    {
        loop {
            let v: String = <String as Prompting>::prompt(name)?;
            let v2: Result<T, Error> = v.parse().map_err(|_| Error::ConversionError);
            if v2.is_ok() {
                return v2;
            }
            println!("Invalid input");
        }
    }
}

impl Prompting for String {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        use std::io::Write;
        let mut buffer = String::new();
        if let Some(n) = name {
            print!("{}: ", n);
            std::io::stdout().flush().unwrap();
        }
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| Error::InputError(e))?;
        buffer.pop();
        loop {
            if buffer.ends_with('\n') {
                buffer.pop();
                continue;
            }
            if buffer.ends_with('\r') {
                buffer.pop();
                continue;
            }
            break;
        }
        Ok(buffer)
    }
}

impl Prompting for u8 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for i8 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for u16 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for i16 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for u32 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for i32 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for u64 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for i64 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for usize {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for f32 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for f64 {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for bool {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl Prompting for std::path::PathBuf {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        Self::prompt_generic::<Self>(name)
    }
}

impl<T> Prompting for std::collections::HashMap<String, T>
where
    T: Prompting,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        use std::io::Write;
        if let Some(n) = name {
            print!("{}: ", n);
            std::io::stdout().flush().unwrap();
        }
        let mut hm = std::collections::HashMap::new();
        loop {
            print!("Enter key name (blank to end):");
            std::io::stdout().flush().unwrap();
            let key = String::prompt(None).unwrap();
            if key.is_empty() {
                println!("Done");
                break;
            }
            let t = T::prompt(None).unwrap();
            hm.insert(key, t);
        }
        Ok(hm)
    }
}

impl<T> Prompting for Vec<T>
where
    T: core::str::FromStr,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        loop {
            let mut v: String = <String as Prompting>::prompt(name)?;
            v.pop();
            if v.is_empty() {
                return Ok(Vec::new());
            } else {
                let mut vtotal: Vec<T> = Vec::new();
                let els: Vec<&str> = v.split(',').collect();
                let mut invalid = false;
                for el in els.iter() {
                    let v: Result<T, Error> = el.parse().map_err(|_| Error::ConversionError);
                    if let Ok(v) = v {
                        vtotal.push(v);
                    } else {
                        println!("Invalid input");
                        invalid = true;
                        break;
                    }
                }
                if !invalid {
                    return Ok(vtotal);
                }
            }
        }
    }
}

impl<T> Prompting for Option<T>
where
    T: Prompting,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        if let Some(name) = name {
            println!("[{} is optional, provide? (true/false)]", name);
        }
        let v = bool::prompt(name)?;
        if v {
            match T::prompt(name) {
                Ok(t) => {
                    return Ok(Some(t));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            Ok(None)
        }
    }
}
