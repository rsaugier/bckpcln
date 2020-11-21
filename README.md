# bckpcln

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
