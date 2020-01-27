extern crate serde;
extern crate serde_xml_rs;
extern crate yaml_rust;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
  metadata: ProjectMetadata,
  java: Java,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ProjectMetadata {
  name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Java {
  source: String,
  output: String,
  deps: Vec<String>, // This should be a vec of Dep or somesuch
}

pub struct ProjectLoader {
  path_to_project: HashMap<PathBuf, Project>,
}

impl ProjectLoader {
  pub fn load_project(&mut self, file_path: &PathBuf) -> Result<&Project, ProjectError> {
    let yaml_text = fs::read_to_string(file_path)?;
    let project = serde_yaml::from_str(&yaml_text)?;

    self.path_to_project.insert(file_path.to_owned(), project);

    Ok(self.get(file_path).unwrap())
  }

  pub fn get(&mut self, file_path: &PathBuf) -> Option<&Project> {
    self.path_to_project.get(file_path)
  }

  pub fn new() -> ProjectLoader {
    ProjectLoader {
      path_to_project: HashMap::new(),
    }
  }
}

#[derive(Debug)]
pub enum ProjectError {
  ParsingFailed(serde_yaml::Error),
  ReadFailed(io::Error),
}

impl fmt::Display for ProjectError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      ProjectError::ParsingFailed(e) => e.fmt(f),
      ProjectError::ReadFailed(e) => e.fmt(f),
    }
  }
}

impl StdError for ProjectError {
  fn description(&self) -> &str {
    match &*self {
      ProjectError::ParsingFailed(e) => e.description(),
      ProjectError::ReadFailed(e) => e.description(),
    }
  }
}

impl From<io::Error> for ProjectError {
  fn from(e: io::Error) -> Self {
    ProjectError::ReadFailed(e)
  }
}

impl From<serde_yaml::Error> for ProjectError {
  fn from(e: serde_yaml::Error) -> Self {
    ProjectError::ParsingFailed(e)
  }
}
