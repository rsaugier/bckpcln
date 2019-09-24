extern crate clap;
use clap::{Arg, App};
use std::path::PathBuf;
use crate::backups::BackupsFolder;

mod backups;


static PROGRAM_NAME : &str = "bckpcln";
static PROGRAM_VERSION : &str = "1.0";

fn main() {
    let app = App::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author("rodolphe saugier <rodolphe.saugier@gmail.com>")
        .about("bckpcln is a BaCKuP CLeaNer. \n\
        Removes backup files from a directory containing many in order to keep the directory size \
        under a given threshold. \n\
        Older backups have a higher probability to be deleted, but we try \
        to keep old ones too. \n\
        NOTE: sub-folders are unsupported.")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("BACKUP_DIR")
             .help("The directory to process. Default is current working directory."))
        .arg(Arg::with_name("max-size")
             .short("m")
             .long("max-size")
             .value_name("MAX_SIZE_IN_GB")
             .help("Defines the maximum accepted size of the backup folder in gigabytes (accepts decimal value with . as a decimal separator)")
             .takes_value(true))
        .arg(Arg::with_name("force")
             .short("f")
             .long("force")
             .help("Forces the deletion without prompting")
             .takes_value(false))
        .arg(Arg::with_name("dry-run")
             .long("dry-run")
             .help("Do not delete anything, just show what would be deleted"));
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

    println!("Target backup directory: {}", backup_directory_path.to_string_lossy());

    println!("Max size: {} GiB", max_size);

    match BackupsFolder::read(backup_directory_path.as_path()) {
        Ok(backupsFolder) => {
            process(&backupsFolder);
        },
        Err(error) => {
            eprintln!("ERROR: {}", error);
        }
    }
}

fn process(backupsFolder : &BackupsFolder) {
    println!("Cumulated size of all backup files: {}", backups::human_size(backupsFolder.total_files_size));
    println!("Backup cleanup order:");
    for backup in backupsFolder.iter_backups_in_deletion_order() {
        println!("{}", backup);
    }
}
