# bckpcln reference doc

#### Version / Disclaimer

**bckpcln vesion 0.2** is still early WIP, although I use it everyday on my personal server and *it seems to work*. 

## Description

#### Summary

This is a small CLI tool to automatically cleanup your less useful local system backups.

Written in Rust, it is reliable, has very few dependencies, and can easily run in a small server.

#### Use case:

On your server, you have put in place a program or script to periodically create local system backups in a directory.
Your backups are *not* incremental, but **full backups**.

Over time, the backups quickly pile up, and you have to archive or delete them if you don't want 
to run out of disk space.
 
**bckpcln** is a small tool that allows you to delete some of these backups automatically.
It is intended to be run periodically (via **cron** for example) in the directory where your backups are stored,
to "clean" the backups and regain space.

**bckpcln** won't simply delete the oldest backups,
but instead uses an algorithm to try to delete the "less useful" ones.
It assume that not only the latest backups are important, but also older ones.
 
Indeed, ideally, you want to be able to restore your system in its latest state after a hardware failure,
but also maybe in some of its older states. 
This can be the case if you want to restore it before a system upgrade failure,
or before an intrusion (presuming the backup files were not compromised!). 
Or if you simply want to go back in time, out of curiosity...  

Note that there is no assumption on the rhythm at which the backups are created.

#### Backup directory format 
   
The backups must stored in a single directory.
In this version of **bckpcln**, the backups must be stored in subdirectories in this central directory.
These subdirs must be named using a pattern that indicates the date of the backup files inside.
The precise content of each subdirectory is not important.

As an example:
```text
backups_folder
│
├── 2012-02-24_1545_08
│   ├── foo.bak
|   └── bar
|── 2012-07-15_1854_53
│   ├── foo
│   └── baz
│       ├── backup_bar1
│       └── bar2.tgz
|── 2012-07-15_1854_53
│   ├── foo.bk
│   └── baz
│       └── bar
│           └── baf.partimage
|
.
.
.
```
 
#### How **bckpcln** chooses the backups to delete

The "usefulness" criteria for a backup are the following:
- The backup is isolated. A backup is isolated if its creation date is far from any other backup's date.
- The backup is recent.

The first criterium has a higher priority than the second.

## Running bckpcln

The best way to learn about **bckpcln** is to run it. It won't delete anything without being passed the right arguments.
You can run this command to learn about its usage:

```
> bckpcln --help

USAGE:
    bckpcln [FLAGS] [OPTIONS] --max-size <MAX_SIZE>

FLAGS:
        --delete     Perform the actual deletion
    -f, --force      Forces the deletion without prompting
    -h, --help       Prints help information
    -l, --list       List all the backups and their properties
    -V, --version    Prints version information
    -v, --verbose    Print more details

OPTIONS:
    -d, --directory <BACKUP_DIR>    The directory to process. Default is current working directory.
    -m, --max-size <MAX_SIZE>       Defines the maximum accepted size of the backup folder. Supports standard units.
                                    Examples: 5k, 10M, 6G
        --move <TARGET_FOLDER>      Perform a move (instead of delete) to the specified target folder
```

## Simple example

Assuming your backups are stored in /mnt/foo/backups, and the max total size you want to allow them is 50GB,
you can run this command: 
```
> bckpcln -d /mnt/foo/backups -m 50G

Backup directory to clean up: /mnt/foo/backups
Max size: 75 GiB
Perform delete: No, just explain
Cumulated size of all backup files: 52 GiB
Cumulated backups size is higher than the max size - cleanup is needed!
Deleting (or moving) "/mnt/foo/backups/sys425123.bak" would free 1 GiB
Deleting (or moving) "/mnt/foo/backups/sys423456.bak" would free 1 GiB
New cumulated size of all backup files : 49 GiB
```

**bckpcln** will show you the file it intends to delete, and the space you would regain.
But nothing gets deteled - by default, it's a *dry run!*

When you're ready to actually delete the files, simply add the --delete flag to **actually delete the files**:

```
> bckpcln -d /mnt/foo/backups -m 50G --delete
```

You also have the option to move the files to another directory, instead of deleting them:

```
> bckpcln -d /mnt/foo/backups -m 50G --move /mnt/bar/archive
```

