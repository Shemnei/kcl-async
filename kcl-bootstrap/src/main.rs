pub mod cli;
pub mod maven;
pub mod xml;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser as _;

use cli::Options;
use maven::MavenPackage;

fn fetch_jars(
    jar_folder: &Path,
    packages: &[MavenPackage],
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    eprintln!("Fetching JARs into folder: {}", jar_folder.display());
    let mut paths = vec![];

    for pkg in packages {
        pkg.fetch(jar_folder)?;
    }

    for entry in std::fs::read_dir(jar_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "jar") {
            eprintln!("Found JAR: {}", path.display());
            paths.push(path);
        }
    }

    paths.push(env::current_dir()?); // Add CWD to classpath
    Ok(paths)
}

fn find_java(custom: Option<String>) -> Option<String> {
    if let Some(java) = custom {
        eprintln!("Using custom Java path: {java}");
        return Some(java);
    }

    let found = which::which("java")
        .ok()
        .map(|p| p.to_string_lossy().to_string());
    if let Some(ref path) = found {
        eprintln!("Found Java in PATH: {path}");
    } else {
        eprintln!("Java not found in PATH");
    }

    found
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Parsing CLI arguments...");
    let args = Options::parse();

    eprintln!("Parsing POM...");
    let packages = xml::parse_pom(&args.pom_file)?;

    let jar_folder = Path::new(&args.jar_folder);
    eprintln!("Fetching JAR files...");
    let classpath = fetch_jars(jar_folder, &packages)?;

    eprintln!("Looking for Java...");
    let java =
        find_java(args.java_location).ok_or("Java not found (neither custom path nor in PATH)")?;

    eprintln!("Building classpath...");
    let classpath_str = std::env::join_paths(classpath)?
        .to_str()
        .expect("Non unicode in path")
        .to_owned();

    eprintln!("Building command line...");
    let mut cmd = Command::new(&java);
    cmd.args(["-cp", &classpath_str]);
    cmd.arg("software.amazon.kinesis.multilang.MultiLangDaemon");
    cmd.args(["-p", &args.properties_file]);

    if let Some(logback) = args.logback_configuration {
        cmd.args(["-l", &logback]);
    }

    if args.should_execute {
        eprintln!("Executing Java process...");
        let mut proc = cmd.spawn().expect("Failed to execute Java");

        proc.wait()?;
    } else {
        let program = shell_escape::escape(cmd.get_program().to_string_lossy());
        let args = cmd
            .get_args()
            .map(|arg| shell_escape::escape(arg.to_string_lossy()));
        let output = std::iter::once(program)
            .chain(args)
            .collect::<Vec<_>>()
            .join(" ");

        println!("{output}");
    }

    Ok(())
}
