use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io, error};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ffi::OsStr;
use chrono;
use chrono::{DateTime, Utc, Local, TimeZone, ParseError};
use std::borrow::Borrow;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Backup {
    pub date : DateTime<Local>,
    pub path : PathBuf
}

pub struct BackupsFolder {
    pub path : PathBuf,
    pub backups : Vec<Backup>
}

impl BackupsFolder {

    pub fn read(folder: &Path) -> Result<BackupsFolder> {
        let mut backups : Vec<Backup> = Vec::new();
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let path = entry.path();
                let creation_date = creation_date_from_filename(path.file_name().unwrap())?;
                let backup = Backup {
                    date: creation_date,
                    path
                };
                backups.push(backup);
            }
        }
        Ok(BackupsFolder {
            path : PathBuf::from(folder),
            backups
        })
    }
}

fn creation_date_from_filename(filename : &OsStr) -> Result<DateTime<Local>> {
    Ok(Local.datetime_from_str(filename.to_string_lossy().borrow(), "%F_%H%M_%S")?)
}

impl Display for BackupsFolder {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(f, "BackupsFolder: \"{}\" [", self.path.display());
        for backup in self.backups.iter() {
            writeln!(f, "    {}", backup);
        }
        writeln!(f, "]");
        Ok(())
    }
}

impl Display for Backup {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        Display::fmt(&self.date, f)?;
        write!(f, " : ");
        Debug::fmt(&self.path, f)?;
        Ok(())
    }
}



