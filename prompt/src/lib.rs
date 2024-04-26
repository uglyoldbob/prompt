//! A module for prompting a user to fill out a struct

pub use prompt_derive::Prompting;

#[cfg(feature = "egui")]
pub use prompt_derive::EguiPrompting;

#[cfg(feature = "egui")]
pub use egui;

#[cfg(feature = "egui")]
/// The trait involved in building a input form for egui
pub trait EguiPrompting: Sized {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String>;
}

#[cfg(feature = "egui")]
impl EguiPrompting for String {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        ui.text_edit_singleline(self);
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for Password {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        let p: &mut String = &mut self.0;
        let pe = egui::TextEdit::singleline(p).password(true);
        ui.add(pe);
        if self.0.is_empty() {
            return Err(format!("{} password is blank", name.unwrap_or("")));
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for Password2 {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        let p: &mut String = &mut self.0;
        let pe = egui::TextEdit::singleline(p).password(true);
        ui.add(pe);
        let p: &mut String = &mut self.1;
        let pe = egui::TextEdit::singleline(p).password(true);
        ui.add(pe);
        if !self.0.is_empty() && self.0 == self.1 {
            Ok(())
        }
        else {
            Err(format!("{} password does not match", name.unwrap_or("")))
        }
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for u8 {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let mut s = self.to_string();
        s.build_gui(ui, name)?;
        if let Ok(val) = s.parse::<Self>() {
            *self = val;
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for i8 {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let mut s = self.to_string();
        s.build_gui(ui, name)?;
        if let Ok(val) = s.parse::<Self>() {
            *self = val;
        }
        Ok(())
    }
}

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
    /// Construct a new password.
    pub fn new(s: String) -> Self {
        Password(s)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// This is a type that allows a user to create a password, prompting them once for the password, and a second time to verify the password.
pub struct Password2(String, String);

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
        Ok(Password2(buffer.clone(), buffer))
    }
}

impl Password2 {
    /// Construct a new password
    pub fn new(s: String) -> Self {
        Password2(s.clone(), s)
    }

    /// Get a mutable reference to the second copy of the password
    pub fn second(&mut self) -> &mut String {
        &mut self.1
    }

    /// Return true if the two passwords match
    pub fn matches(&self) -> bool {
        self.0 == self.1
    }
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
            let v: String = <String as Prompting>::prompt(name)?;
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

impl<T> Prompting for Box<T>
where
    T: Prompting,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        let a: T = T::prompt(name)?;
        Ok(Box::new(a))
    }
}
