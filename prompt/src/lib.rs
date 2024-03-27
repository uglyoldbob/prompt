//! A module for prompting a user to fill out a struct

pub use prompt_derive::Prompting;

pub trait Prompter<T> {
    fn prompt(&mut self) -> Result<T, Error>;
}

/// The types of errors that can occur when prompting for user input.
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
            let mut v: String = <String as Prompting>::prompt(name)?;
            v.pop();
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

impl<T> Prompting for Option<T>
where
    T: Prompting + core::str::FromStr,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        loop {
            let mut v: String = <String as Prompting>::prompt(name)?;
            v.pop();
            if v.is_empty() {
                return Ok(None);
            } else {
                let v: Result<T, Error> = v.parse().map_err(|_| Error::ConversionError);
                if let Ok(v) = v {
                    return Ok(Some(v));
                }
            }
            println!("Invalid input");
        }
    }
}
