extern crate serde;
extern crate serde_xml_rs;
extern crate yaml_rust;
use serde_xml_rs::{from_str, to_string};
use std::path::PathBuf;
mod eclipse;
mod rogers;

fn main() {
    println!("Hello, world!");
    let mut project_loader = rogers::ProjectLoader::new();
    let path = PathBuf::from("/Users/brianduff/Documents/Development/graphs/src/test/rogers.yaml");

    let project = project_loader.load_project(&path);
    println!("{:?}", project);

    let another_project = project_loader.get(&path).unwrap();
    println!("{:?}", another_project);

    // Test out deserializing xml
    let xml = r##"
    <projectDescription>
	<name>graphs_test</name>
	<comment></comment>
	<projects>
	</projects>
	<buildSpec>
		<buildCommand>
			<name>org.eclipse.jdt.core.javabuilder</name>
			<arguments>
			</arguments>
		</buildCommand>
	</buildSpec>
	<natures>
		<nature>org.eclipse.jdt.core.javanature</nature>
	</natures>
    </projectDescription>
    "##;

    let project_desc: eclipse::ProjectDescription = from_str(&xml).unwrap();

    println!("{:?}", project_desc);

    let cpxml = r##"
    <classpath>
        <classpathentry kind="src" path="java"/>
        <classpathentry kind="con" path="org.eclipse.jdt.launching.JRE_CONTAINER">
            <attributes>
                <attribute name="module" value="true"/>
            </attributes>
        </classpathentry>
        <classpathentry kind="lib" path="../../third-party/junit/junit-4.13.jar" sourcepath="../../third-party/junit/junit-4.13-sources.jar"/>
        <classpathentry combineaccessrules="false" kind="src" path="/graphs_main"/>
        <classpathentry kind="output" path="bin"/>
    </classpath>"##;
    let cp: eclipse::Classpath = from_str(&cpxml).unwrap();

    println!("{:?}", cp);

    let ep = eclipse::create_eclipse_project(&project_loader);
    println!("{:?}", ep);
}
