use std::path::{Path, PathBuf};
use std::{fs, error, cmp};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ffi::OsStr;
use chrono;
use chrono::{DateTime, Utc, TimeZone, Duration};
use crate::human_size::*;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Backup {
    pub date : DateTime<Utc>,
    pub path : PathBuf,
    pub size : u64,

    /// The minimum time span between this backup and the previous or next one.
    pub isolation : Duration
}

#[derive(Clone)]
pub struct BackupsFolder {
    pub path : PathBuf,
    pub backups : Vec<Backup>,
    pub total_files_size : u64
}

impl Backup {

    pub fn new(creation_date : DateTime<Utc>, path : PathBuf, size : u64) -> Backup {
        Backup {
            date: creation_date,
            path,
            size,
            isolation: Duration::zero()
        }
    }

}

impl BackupsFolder {

    pub fn read(folder: &Path) -> Result<BackupsFolder> {
        let mut backups : Vec<Backup> = Vec::new();
        let mut total_files_size = 0;
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let path = entry.path();
                let creation_date = creation_date_from_filename(&path)?;
                let backup_total_size = directory_total_size(path.as_os_str())?;
                total_files_size += backup_total_size;
                let backup = Backup::new(creation_date, path, backup_total_size);
                backups.push(backup);
            }
        }
        update_isolations(&mut backups);
        Ok(BackupsFolder {
            path : PathBuf::from(folder),
            backups,
            total_files_size
        })
    }

    fn pop_best_candidate_for_deletion(&mut self) -> Option<Backup> {
        self.backups.sort_by(|x, y| x.isolation.cmp(&y.isolation).reverse().then(x.date.cmp(&y.date).reverse()));
        match self.backups.pop() {
            Some(x) => {
                update_isolations(&mut self.backups);
                Some(x)
            },
            None => None
        }
    }

    pub fn iter_backups_in_deletion_order(&self) -> DeletionOrderBackupIterator {
        DeletionOrderBackupIterator {
            remaining_backups : self.clone()
        }
    }

}

pub struct DeletionOrderBackupIterator {
    remaining_backups : BackupsFolder
}

impl Iterator for DeletionOrderBackupIterator {
    type Item = (Backup, BackupsFolder);

    fn next(&mut self) -> Option<(Backup, BackupsFolder)> {
        match self.remaining_backups.pop_best_candidate_for_deletion() {
            Some(best_candidate) => Some((best_candidate, self.remaining_backups.clone())),
            None => None
        }
    }
}

fn compute_isolations(backups : &Vec<Backup>) -> Vec<Duration> {
    let len = backups.len();
    let mut isolations : Vec<Duration> = Vec::new();
    for index in 0 .. len {
        if (index == 0) || (index == len-1) {
            isolations.push(Duration::max_value());
        }
        else {
            let prev = backups[index - 1].date;
            let next = backups[index + 1].date;
            let curr = backups[index].date;
            isolations.push(cmp::min(curr - prev, next - curr));
        }
    }
    isolations
}

fn update_isolations(backups : &mut Vec<Backup>) {
    backups.sort();
    let isolations = compute_isolations(backups);
    for pair in backups.iter_mut().zip(isolations.iter()) {
        pair.0.isolation = *pair.1;
    }
}

fn creation_date_from_filename(path : &PathBuf) -> Result<DateTime<Utc>> {
    let filename = path.file_name().unwrap();
    match Utc.datetime_from_str(&filename.to_string_lossy(), "%F_%H%M_%S") {
        Ok(date) => return Ok(date),
        Err(_) => simple_error::bail!("cannot parse date from this file name: {}", path.to_string_lossy())
    }
}

fn directory_total_size(path : &OsStr) -> Result<u64> {
    let mut size : u64= 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            size += entry.metadata().unwrap().len();
        }
        else if entry.file_type()?.is_dir() {
            size += directory_total_size(entry.path().as_os_str())?;
        }
        else if entry.file_type()?.is_symlink() {
            eprintln!("WARNING: ignoring symlink: {}", entry.path().to_string_lossy());
        }
    }
    Ok(size)
}

impl Display for Backup {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        Display::fmt(&self.date, f)?;
        write!(f, " : ")?;
        Debug::fmt(&self.path.file_name().unwrap(), f)?;
        write!(f, " isolation=")?;
        if self.isolation == Duration::max_value() {
            write!(f, "max")?;
        }
        else {
            write!(f, "({:>05}.{:02}:{:02}:{:02})",
                   self.isolation.num_days(),
                   self.isolation.num_hours() % 24,
                   self.isolation.num_minutes() % 60,
                   self.isolation.num_seconds() % 60)?;
        }
        write!(f, " size={}", self.size.to_human_size())?;
        Ok(())
    }
}

impl Display for BackupsFolder {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(f, "\"{}\" [", self.path.display())?;
        for backup in self.backups.iter() {
            writeln!(f, "    {}", backup)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}
