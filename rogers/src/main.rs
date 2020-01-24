extern crate yaml_rust;
use std::fs;
use std::io;
use std::fmt;
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Project {
    metadata: ProjectMetadata,
    java: Java
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ProjectMetadata {
    name: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Java {
    source: String,
    output: String,
    deps: Vec<String> // This should be a vec of Dep or somesuch
}

#[derive(Debug)]
pub enum ProjectError {
    ParsingFailed(serde_yaml::Error),
    ReadFailed(io::Error)
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ProjectError::ParsingFailed(e) => e.fmt(f),
            ProjectError::ReadFailed(e) => e.fmt(f)
        }
    }
}

impl StdError for ProjectError {
    fn description(&self) -> &str {
        match &*self {
            ProjectError::ParsingFailed(e) => e.description(),
            ProjectError::ReadFailed(e) => e.description()
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

fn main() {
    println!("Hello, world!");
    let project = load_project("/Users/brianduff/Documents/Development/graphs/src/test/rogers.yaml");

    println!("{:?}", project);
}

/// Loads a Rogers project from the given file.
fn load_project(filename: &str) -> Result<Project, ProjectError> {
    let yaml_text = fs::read_to_string(filename)?;
    let project = serde_yaml::from_str(&yaml_text)?;

    Ok(project)
}