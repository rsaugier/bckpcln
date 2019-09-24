extern crate clap;
use clap::{Arg, App};
use std::path::PathBuf;
use crate::backups::BackupsFolder;
use std::str::FromStr;
use std::cmp::max;

mod backups;


static PROGRAM_NAME : &str = "bckpcln";
static PROGRAM_VERSION : &str = "1.0";

fn main() {
    let app = App::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author("rodolphe saugier <rodolphe.saugier@gmail.com>")
        .about("bckpcln is a simple tool to periodically and automatically cleanup a folder filled with backups")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("BACKUP_DIR")
             .help("The directory to process. Default is current working directory."))
        .arg(Arg::with_name("max-size")
             .short("m")
             .long("max-size")
             .value_name("MAX_SIZE_IN_GB")
             .required(true)
             .help("Defines the maximum accepted size of the backup folder in gigabytes (accepts decimal value with . as a decimal separator)")
             .takes_value(true))
        .arg(Arg::with_name("force")
             .short("f")
             .long("force")
             .help("Forces the deletion without prompting")
             .takes_value(false))
        .arg(Arg::with_name("delete")
             .long("delete")
             .takes_value(false)
             .help("Perform the actual deletion (by default the tool only explains what would be deleted)"));
    let args = app.get_matches();

    let backup_directory_path : PathBuf;
    match args.value_of("directory") {
        Some(path) => {
            backup_directory_path = PathBuf::from(path);
        },
        None => {
            backup_directory_path = std::env::current_dir().unwrap();
        }
    }
    let max_size = args.value_of("max-size").expect("max size is required");
    let max_size = u64::from_str(max_size).expect("invalid max size format") * (1024 * 1024 * 1024);
    let must_delete = args.is_present("delete");

    println!("Target backup directory: {}", backup_directory_path.to_string_lossy());

    println!("Max size: {}", backups::human_size(max_size));
    println!("Perform delete: {}", if must_delete { "Yes!" } else { "No, just explain" });

    match BackupsFolder::read(backup_directory_path.as_path()) {
        Ok(backupsFolder) => {
            process(&backupsFolder, max_size, must_delete);
        },
        Err(error) => {
            eprintln!("ERROR: {}", error);
        }
    }
}

fn process(backupsFolder : &BackupsFolder, max_size : u64, must_delete : bool) {
    println!("Cumulated size of all backup files: {}", backups::human_size(backupsFolder.total_files_size));

    if backupsFolder.total_files_size < max_size {
        println!("Cumulated backups size is lower than the max size - nothing to do");
    }
    else {
        let mut new_size = backupsFolder.total_files_size;
        println!("Cumulated backups size is higher than the max size - cleanup is needed!");
        for backup in backupsFolder.iter_backups_in_deletion_order() {
            println!("Deleting {} would gain {}", backup.path.to_string_lossy(), backup.size);
            new_size -= backup.size;
            if new_size <= max_size {
                println!("New size would be : {}", backups::human_size(new_size));
                break;
            }
        }
    }
}
