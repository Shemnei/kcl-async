use crate::maven::MavenPackage;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn parse_pom(path: &str) -> Result<Vec<MavenPackage>, Box<dyn std::error::Error>> {
    eprintln!("Parsing POM: {path}");
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut current_tag = String::new();

    let mut in_dependency = false;
    let mut in_properties = false;
    let mut in_exclusions = false;

    let mut group_id = String::new();
    let mut artifact_id = String::new();
    let mut version = String::new();

    let mut versions_map = HashMap::new();
    let mut packages = vec![];

    while let Ok(ev) = reader.read_event_into(&mut buf) {
        match ev {
            Event::Eof => break,

            Event::Start(ref e) => {
                let tag = strip_ns(e.name().into_inner());
                current_tag = tag.to_string();

                match current_tag.as_str() {
                    "dependency" => {
                        in_dependency = true;
                        group_id.clear();
                        artifact_id.clear();
                        version.clear();
                    }
                    "properties" => in_properties = true,
                    "exclusions" => in_exclusions = true,
                    _ => {}
                }
            }

            Event::End(ref e) => {
                let tag = strip_ns(e.name().into_inner());

                match tag {
                    "dependency" => {
                        in_dependency = false;
                        if in_exclusions {
                            // skip dependencies inside <exclusions>
                        } else if !group_id.is_empty()
                            && !artifact_id.is_empty()
                            && !version.is_empty()
                        {
                            let version_clean = versions_map
                                .get(&version)
                                .cloned()
                                .unwrap_or(version.clone());

                            packages.push(MavenPackage::new(
                                &group_id,
                                &artifact_id,
                                &version_clean,
                            ));

                            eprintln!(
                                "Parsed dependency: {group_id}:{artifact_id}:{version_clean}"
                            );
                        }
                    }
                    "properties" => in_properties = false,
                    "exclusions" => in_exclusions = false,
                    _ => {}
                }
            }

            Event::Text(e) => {
                let text = e.decode()?.into_owned();

                if in_properties {
                    versions_map.insert(format!("${{{current_tag}}}"), text);
                } else if in_dependency && !in_exclusions {
                    match current_tag.as_str() {
                        "groupId" => group_id = text,
                        "artifactId" => artifact_id = text,
                        "version" => version = text,
                        _ => {}
                    }
                }
            }

            _ => {}
        }

        buf.clear();
    }

    eprintln!("Total dependencies parsed: {}", packages.len());
    Ok(packages)
}

fn strip_ns(tag: &[u8]) -> &str {
    // Remove namespace prefix if present
    match std::str::from_utf8(tag) {
        Ok(s) => s.split(':').next_back().unwrap_or(s),
        Err(_) => "",
    }
}
