extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use clap::{Arg, App};
use std::path::PathBuf;
use std::str::FromStr;

mod backups;
mod human_size;

use crate::backups::BackupsFolder;
use crate::human_size::*;

static PROGRAM_NAME : &str = "bckpcln";
static PROGRAM_VERSION : &str = "0.2";

enum Action {
    Explain,
    Delete,
    Move
}

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
             .value_name("MAX_SIZE")
             .required(true)
             .help("Defines the maximum accepted size of the backup folder. Supports standard units. Examples: 5k, 10M, 6G")
             .takes_value(true))
        .arg(Arg::with_name("force")
             .short("f")
             .long("force")
             .help("Forces the deletion without prompting")
             .takes_value(false))
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .help("List all the backups and their properties")
                 .takes_value(false))
        .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .help("Print more details")
                 .takes_value(false))
        .arg(Arg::with_name("delete")
             .long("delete")
             .takes_value(false)
             .help("Perform the actual deletion"))
        .arg(Arg::with_name("move")
                 .long("move")
                 .takes_value(true)
                 .value_name("TARGET_FOLDER")
                 .help("Perform a move (instead of delete) to the specified target folder"));
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
    let max_size = u64::from_human_size(max_size).expect("invalid max size format");
    let must_delete = args.is_present("delete");
    let must_move = args.is_present("move");
    let must_list = args.is_present("list");
    let verbose = args.is_present("verbose");
    let target_folder: Option<&str> = args.value_of("move");

    let action =
        match (must_delete, must_move) {
            (true, true) => {
                eprintln!("ERROR: only one argument is allowed: move or delete");
                return;
            },
            (true, false) => Action::Delete,
            (false, true) => Action::Move,
            (false, false) => Action::Explain
        };

    println!("Backup directory to clean up: {}", backup_directory_path.to_string_lossy());

    println!("Max size: {}", max_size.to_human_size());
    println!("Perform delete: {}", if must_delete { "Yes!" } else { "No, just explain" });

    match BackupsFolder::read(backup_directory_path.as_path()) {
        Ok(backups_folder) => {
            if must_list {
                println!("Backups folder: {}", backups_folder)
            }
            process(&backups_folder, max_size, action, target_folder, verbose);
        },
        Err(error) => {
            eprintln!("ERROR: {}", error);
        }
    }
}

fn process(backups_folder : &BackupsFolder, max_size : u64, action : Action, target_dir : Option<&str>, verbose : bool) {
    println!("Cumulated size of all backup files: {}", backups_folder.total_files_size.to_human_size());

    if backups_folder.total_files_size < max_size {
        println!("Cumulated backups size is lower than the max size - nothing to do");
    }
    else {
        let mut new_size = backups_folder.total_files_size;
        println!("Cumulated backups size is higher than the max size - cleanup is needed!");

        for iteration in backups_folder.iter_backups_in_deletion_order() {
            let (candidate, new_folder_state) = iteration;
            let path = String::from(candidate.path.to_string_lossy());
            let size = String::from(candidate.size.to_human_size());
             match action {
                 Action::Explain => {
                     new_size -= candidate.size;
                     println!("Deleting (or moving) \"{}\" would free {}", path, size);
                 },
                 Action::Delete => {
                     match std::fs::remove_dir_all(&path) {
                         Ok(()) => {
                             new_size -= candidate.size;
                             println!("Deleted \"{}\" to free {}", &path, size);
                         }
                         Err(e) => {
                             eprintln!("Deletion of \"{}\" failed, ERROR : {}", &path, e);

                         }
                     }
                 },
                 Action::Move => {
                     let mut dest_path = PathBuf::from_str(target_dir.expect("missing target folder?")).expect("invalid target folder");
                     dest_path.push(&path);
                     println!("Moving \"{}\" to \"{}\" to free {}", path, dest_path.to_string_lossy(), size);
                     //std::fs::rename(path, )
                 }
            };
            if verbose {
                println!("New folder state: {}", new_folder_state);
            }
            if new_size <= max_size {
                println!("New cumulated size of all backup files : {}", new_size.to_human_size());
                break;
            }
        }
    }
}
