use chrono::{DateTime, Utc};
use modification_updater::{LastModification, ProjectResource};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Must pass a path as first argument");
    let actor = args.get(2).expect("Must pass an actor as second argument");
    let timestamp_string = args.get(3).expect("Must pass a timestamp as third argument");
	let timestamp : DateTime<Utc> = DateTime::parse_from_rfc3339(timestamp_string).unwrap().with_timezone(&Utc);
    
    let resource = ProjectResource::from_path(Path::new(path))?;
    let updated = resource.update(LastModification {
        actor: actor.clone(),
        timestamp: timestamp,
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&updated.manifest).unwrap()
    );

    Ok(())
}
