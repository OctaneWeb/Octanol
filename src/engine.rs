use crate::lexer::Lexer;
use std::default::Default;
use std::error;
use std::fmt::{Display, Error, Formatter};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Octanol {
    serve_folders: Vec<PathBuf>,
    default_page: Option<PathBuf>,
}
#[derive(Debug, Clone)]
struct Engine {
    octanol: Octanol,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            octanol: Octanol::new(),
        }
    }
    pub fn with_instance(octanol: Octanol) -> Self {
        Engine { octanol }
    }
    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        let mut default_page = PathBuf::new();
        if self.default_page.is_some() {
            default_page = self.default_page.as_ref().unwrap().to_path_buf();
        } else {
            default_page = PathBuf::from("index.octml");
        }
        let file = File::open(default_page)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read(&mut contents)?;
        Lexer::lex(contents);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctanolError {
    FolderNotFound,
    FileNotFound,
}

impl Octanol {
    pub fn new() -> Self {
        Octanol {
            serve_folders: Vec::new(),
            default_page: None,
        }
    }
    pub fn folder(&mut self, folder_name: &str) -> Result<&mut Self, OctanolError> {
        let exists = fs::metadata(folder_name);
        if exists.is_ok() {
            self.serve_folders.push(PathBuf::from(folder_name));
            Ok(self)
        } else {
            Err(OctanolError::FolderNotFound)
        }
    }
    pub fn set_default(&mut self, default_page: &str) -> Result<&mut Self, OctanolError> {
        let exists = fs::metadata(default_page);
        if exists.is_ok() {
            self.default_page = Some(PathBuf::from(default_page));
            Ok(self)
        } else {
            Err(OctanolError::FileNotFound)
        }
    }
    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        Engine::with_instance(self).run()?;
        Ok(())
    }
}

impl Display for Octanol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut default_page = String::with_capacity(7);
        if self.default_page.is_some() {
            default_page.push_str("default")
        } else {
            default_page.push_str(self.default_page.as_ref().unwrap().to_str().unwrap_or(""))
        }
        write!(
            f,
            "folders {:?} default_page {:?}",
            self.serve_folders, default_page
        )
    }
}
impl Display for OctanolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

impl Default for Octanol {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Engine {
    type Target = Octanol;
    fn deref(&self) -> &Self::Target {
        &self.octanol
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl error::Error for OctanolError {}
