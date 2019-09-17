use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};
use std::fmt::{Display, Formatter, Error, Debug};

pub struct Backup {
    pub date : SystemTime,
    pub path : PathBuf
}

pub struct BackupsFolder {
    pub path : PathBuf,
    pub backups : Vec<Backup>
}

pub fn readBackupsFolder(folder : &Path) -> io::Result<BackupsFolder> {
    let mut backups : Vec<Backup> = Vec::new();
    if folder.is_dir() {
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let path = entry.path();
                let creationDate = entry.metadata()?.created()?;
                let backup = Backup {
                    date: creationDate,
                    path
                };
                backups.push(backup);
            }
        }
    }
    Ok(BackupsFolder {
        path : PathBuf::from(folder),
        backups
    })
}

impl Display for BackupsFolder {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "BackupsFolder: \"{}\" {{", self.path.display());
        for backup in self.backups.iter() {
            writeln!(f, "    {}", backup);
        }
        writeln!(f, "}}");
        Ok(())
    }
}

impl Display for Backup {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.date.fmt(f)?;
        write!(f, " : ");
        self.path.fmt(f)?;
        Ok(())
    }
    
}