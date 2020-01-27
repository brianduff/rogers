extern crate serde;
extern crate serde_xml_rs;
extern crate yaml_rust;
use serde::{Deserialize, Serialize};
use std::io;

use crate::rogers;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDescription {
  name: String,
  comment: String,
  #[serde(rename = "buildSpec")]
  build_spec: BuildSpec,
  natures: Vec<Nature>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BuildSpec {
  #[serde(rename = "buildCommand")]
  build_command: Vec<BuildCommand>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BuildCommand {
  name: String,
  arguments: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Nature {
  #[serde(rename = "$value")]
  value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classpath {
  #[serde(rename = "classpathentry", default)]
  entries: Vec<ClasspathEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClasspathEntry {
  kind: String,
  path: String,
  #[serde(rename = "combineaccessrules")]
  combine_access_rules: Option<bool>,
}

#[derive(Debug)]
pub struct EclipseProject {
  project: ProjectDescription,
  classpath: Classpath,
}

#[derive(Debug)]
pub enum CreateProjectError {
  ParsingFailed(serde_yaml::Error),
  ReadFailed(io::Error),
}

pub struct ProjectCollector {}

pub fn create_eclipse_project(
  project_loader: &rogers::ProjectLoader,
) -> Result<EclipseProject, CreateProjectError> {
  let build_command = BuildCommand {
    name: "org.eclipse.jdt.core.javabuilder".to_string(),
    arguments: [].to_vec(),
  };

  let build_commands = vec![build_command];

  Ok(EclipseProject {
    project: ProjectDescription {
      name: "Fake".to_string(),
      comment: "Fake description".to_string(),
      build_spec: BuildSpec {
        build_command: build_commands,
      },
      natures: vec![Nature {
        value: "java_nature".to_string(),
      }],
    },
    classpath: Classpath {
      entries: Vec::new(),
    },
  })
}
