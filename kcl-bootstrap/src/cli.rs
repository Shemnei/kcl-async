use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Options {
    #[arg(short = 'j', long = "java")]
    pub java_location: Option<String>,

    #[arg(short = 'p', long = "properties")]
    pub properties_file: String,

    #[arg(long = "jar-folder", default_value = "jars")]
    pub jar_folder: String,

    #[arg(long = "pom", default_value = "pom.xml")]
    pub pom_file: String,

    #[arg(short = 'e', long = "execute")]
    pub should_execute: bool,

    #[arg(short = 'l', long = "log-configuration")]
    pub logback_configuration: Option<String>,
}
