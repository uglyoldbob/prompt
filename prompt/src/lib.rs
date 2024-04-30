//! A module for prompting a user to fill out a struct

pub use prompt_derive::Prompting;

/// This is used to open existing files on the filesystem
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileOpen {
    /// The pathbuf
    pb: std::path::PathBuf,
    /// The optional filter for the open dialog
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub filter: Option<(String, Vec<String>)>,
    /// The initial directory
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub initial_dir: Option<std::path::PathBuf>,
    /// The initial filename
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub initial_file: Option<String>,
    /// The file dialog title
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub title: Option<String>,
}

impl std::ops::Deref for FileOpen {
    type Target = std::path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

/// This is used to create new files on the filesystem
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileCreate {
    /// The pathbuf
    pb: std::path::PathBuf,
    /// The optional filter for the open dialog
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub filter: Option<(String, Vec<String>)>,
    /// The initial directory
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub initial_dir: Option<std::path::PathBuf>,
    /// The file dialog title
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub title: Option<String>,
}

impl std::ops::Deref for FileCreate {
    type Target = std::path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for FileOpen {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        ui.label(self.pb.display().to_string());
        let n = if let Some(n) = name {
            format!(" {}", n)
        }
        else {
            format!("")
        };
        if ui.button(format!("Update{}", n)).clicked() {
            let mut dialog = rfd::FileDialog::new();
            if let Some((a, b)) = &self.filter {
                dialog = dialog.add_filter(a, b);
            }
            if let Some(p) = &self.initial_dir {
                dialog = dialog.set_directory(p);
            }
            if let Some(f) = &self.initial_file {
                dialog = dialog.set_file_name(f);
            }
            if let Some(t) = &self.title {
                dialog = dialog.set_title(t);
            }
            if let Some(path) = dialog.pick_file() {
                self.pb = path;
            }
        }
        if self.pb.exists() {
            Ok(())
        }
        else {
            Err("Selected file does not exist".to_string())
        }
    }
}

impl Prompting for FileOpen {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        let mut pb;
        loop {
            pb = <std::path::PathBuf as Prompting>::prompt(name)?;
            if !pb.exists() {
                println!("That does not exist, please try again");
            }
            else {
                break;
            }
        }
        Ok(Self {
            pb,
            filter: None,
            initial_dir: None,
            initial_file: None,
            title: None,
        })
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for FileCreate {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        ui.label(self.pb.display().to_string());
        let n = if let Some(n) = name {
            format!(" {}", n)
        }
        else {
            format!("")
        };
        if ui.button(format!("Update{}", n)).clicked() {
            let mut dialog = rfd::FileDialog::new();
            if let Some((a, b)) = &self.filter {
                dialog = dialog.add_filter(a, b);
            }
            if let Some(p) = &self.initial_dir {
                dialog = dialog.set_directory(p);
            }
            if let Some(t) = &self.title {
                dialog = dialog.set_title(t);
            }
            if let Some(path) = dialog.pick_file() {
                self.pb = path;
            }
        }
        if !self.pb.exists() {
            Ok(())
        }
        else {
            Err("Selected file does not exist".to_string())
        }
    }
}

impl Prompting for FileCreate {
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        let mut pb;
        loop {
            pb = <std::path::PathBuf as Prompting>::prompt(name)?;
            if pb.exists() {
                println!("That already exists, please try again");
            }
            else {
                break;
            }
        }
        Ok(Self {
            pb,
            filter: None,
            initial_dir: None,
            title: None,
        })
    }
}

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

#[cfg(feature = "egui")]
impl EguiPrompting for u16 {
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
impl EguiPrompting for i16 {
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
impl EguiPrompting for u32 {
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
impl EguiPrompting for i32 {
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
impl EguiPrompting for u64 {
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
impl EguiPrompting for i64 {
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
impl EguiPrompting for usize {
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
impl EguiPrompting for isize {
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
impl EguiPrompting for f32 {
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
impl EguiPrompting for f64 {
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
impl EguiPrompting for bool {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let cname = if let Some(n) = name {
            n.to_string()
        }
        else {
            "Item".to_string()
        };
        ui.checkbox(self, cname);
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl EguiPrompting for std::path::PathBuf {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let mut s = self.display().to_string();
        s.build_gui(ui, name)?;
        if let Ok(val) = s.parse::<Self>() {
            *self = val;
        }
        Ok(())
    }
}

/// A hashmap with a selection added
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectedHashMap<T> {
    map: std::collections::HashMap<String, T>,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    selection: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    new_selection: String,
}

impl<T> SelectedHashMap<T> {
    /// Construct a new Self
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
            selection: None,
            new_selection: String::new(),
        }
    }

    /// Get a reference to the underlying map
    pub fn map(&self) -> &std::collections::HashMap<String, T> {
        &self.map
    }

    /// Get a mutable reference to the underlying map
    pub fn map_mut(&mut self) -> &mut std::collections::HashMap<String, T> {
        &mut self.map
    }
}

impl<T> Default for SelectedHashMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "egui")]
impl<T> EguiPrompting for SelectedHashMap<T>
where
    T: EguiPrompting + std::default::Default,
{
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        let n = if let Some(n) = name {
            format!("{} ", n)
        }
        else {
            " ".to_string()
        };
        egui::ComboBox::from_label(format!("Select a {}type!", n))
            .selected_text(self.selection.as_ref().unwrap_or(&format!("Selection")))
            .show_ui(ui, |ui| {
                for elem in self.map.keys() {
                    if ui.selectable_label(false, elem).clicked() {
                        self.selection = Some(elem.to_string());
                    }
                }
            });
        ui.label(format!("Name for new {}", n));
        ui.text_edit_singleline(&mut self.new_selection);
        if ui.button("Add new entry").clicked() {
            self.map.insert(
                self.new_selection.to_owned(),
                T::default(),
            );
            self.selection = Some(self.new_selection.to_owned());
        }
        if let Some(s) = &self.selection {
            if let Some(e) = self.map.get_mut(s) {
                let tname = if let Some(n) = name {
                    n.to_string()
                } else {
                    "item".to_string()
                };
                let cname = format!("Entry {} for {}", s, tname);
                e.build_gui(ui, Some(&cname))?;
            }
            if ui.button("Delete this entry").clicked() {
                self.map.remove(s);
                self.selection = None;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl<T> EguiPrompting for Vec<T>
where
    T: core::str::FromStr + core::fmt::Display,
{
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        if let Some(n) = name {
            ui.label(n);
        }
        let s: Vec<String> = self.iter().map(|a| a.to_string()).collect();
        let mut combined = s.join("\n");
        if ui.text_edit_multiline(&mut combined).changed() {
            let v: Vec<&str> = combined.split("\n").collect();
            let mut newvec: Vec<T> = Vec::new();
            for elem in v {
                if let Ok(e) = T::from_str(elem) {
                    newvec.push(e);
                }
                else {
                    return Err(format!("Invalid element {}", elem));
                }
            }
            *self = newvec;
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl<T> EguiPrompting for Option<T>
where
    T: EguiPrompting + Default,
{
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let mut checked = self.is_some();
        if ui.checkbox(&mut checked, name.unwrap_or("Item")).changed() {
            if checked {
                *self = Some(T::default());
            }
            else {
                *self = None;
            }
        }
        if let Some(thing) = self {
            thing.build_gui(ui, name)?;
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl<T> EguiPrompting for Box<T>
where
    T: EguiPrompting,
{
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        self.as_mut().build_gui(ui, name)
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

impl<T> Prompting for SelectedHashMap<T>
where
    T: Prompting,
{
    fn prompt(name: Option<&str>) -> Result<Self, Error> {
        use std::io::Write;
        if let Some(n) = name {
            print!("{}: ", n);
            std::io::stdout().flush().unwrap();
        }
        let mut hm = SelectedHashMap::new();
        loop {
            print!("Enter key name (blank to end):");
            std::io::stdout().flush().unwrap();
            let key = String::prompt(None).unwrap();
            if key.is_empty() {
                println!("Done");
                break;
            }
            let t = T::prompt(None).unwrap();
            hm.map.insert(key, t);
        }
        Ok(hm)
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
