use sha2::{Digest, Sha256};
use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "start" => start(),
        "status" => status(),
        "snap" => snap(&args[2..]),
        "trace" => trace(),
        _ => help(),
    }
}

fn help() {
    println!("Grip 0.1.0");
    println!();
    println!("Usage:");
    println!("  grip start");
    println!("  grip status");
    println!("  grip snap <message>");
    println!("  grip trace");
}

fn grip_dir() -> PathBuf {
    PathBuf::from(".grip")
}

fn start() {
    let dir = grip_dir();

    if dir.exists() {
        println!("A Grip repository already exists here.");
        return;
    }

    fs::create_dir_all(dir.join("snapshots")).expect("Could not create .grip");

    fs::write(dir.join("HEAD"), "").expect("Could not create HEAD");

    println!("Initialized empty Grip repository.");
}

fn status() {
    if !grip_dir().exists() {
        println!("This directory is not a Grip repository.");
        process::exit(1);
    }

    println!("Grip repository");
    println!("Workspace status scanning will be implemented here.");
}

fn snap(args: &[String]) {
    if !grip_dir().exists() {
        println!("This directory is not a Grip repository.");
        process::exit(1);
    }

    if args.is_empty() {
        println!("Usage: grip snap <message>");
        process::exit(1);
    }

    let message = args.join(" ");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid system time")
        .as_secs();

    let data = format!("{}\n{}", timestamp, message);

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    let id = format!("{:x}", hasher.finalize());
    let short_id = &id[..12];

    let snapshot = format!(
        "id={}\ntimestamp={}\nmessage={}\n",
        id, timestamp, message
    );

    fs::write(
        grip_dir()
            .join("snapshots")
            .join(format!("{}.snapshot", id)),
        snapshot,
    )
    .expect("Could not write snapshot");

    fs::write(grip_dir().join("HEAD"), &id).expect("Could not update HEAD");

    println!("Snapshot created.");
    println!();
    println!("  {}", short_id);
    println!("  {}", message);
}

fn trace() {
    if !grip_dir().exists() {
        println!("This directory is not a Grip repository.");
        process::exit(1);
    }

    let snapshots = grip_dir().join("snapshots");

    let entries = fs::read_dir(snapshots)
        .expect("Could not read snapshots");

    println!("Grip trace");
    println!();

    for entry in entries {
        let entry = entry.expect("Could not read snapshot");

        let contents =
            fs::read_to_string(entry.path()).expect("Could not read snapshot");

        let mut message = "";

        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("message=") {
                message = value;
            }
        }

        let filename = entry.file_name();
        let filename = filename.to_string_lossy();

        let id = filename
            .strip_suffix(".snapshot")
            .unwrap_or(&filename);

        println!("● {}  {}", &id[..12.min(id.len())], message);
    }
}
