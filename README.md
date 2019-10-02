# bckpcln
A small tool to delete oldest and less isolated backups from a directory.

/!\ This is work in progress!

####Use case:

You store backups in a directory.
Your backups are *not* incremental, but full backups. 
The backup files are stored in subdirectories, that are named using a pattern that indicates the date of the backup files inside.
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
  

They accumulate with time, and you want to clean the less useful ones from time to time to save disk space.

But what does "less useful" mean ?

In the case of this program:
- You ideally want to be able to restore your data to any given point in time.
- Still, restoring it to its most recent versions is the priority.

So the "usefulness" criteria for a backup are the following:
- The backup is isolated. A backup is isolated if its date is far from any other backup's date.
- The backup is recent.

The first criterium has a higher priority than the second.


TODO: continue this...