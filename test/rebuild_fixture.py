#!/usr/bin/python3
import datetime
import random
import os
import shutil

def rand_file_name(n):
    return random.choice(["foo", "bar", "baz"]) + str(n)

def rand_dir_name(n):
    return "d" + random.choice(["foo", "bar", "baz"]) + str(n)

def fill_file(path):
    f = open(path, "w+")
    sz = random.randrange(1, 64)
    for i in range(sz):
       f.write("*" * 1024)
    f.close()

def fill_dir(path1, level, maxlevel):
    num_files = random.randrange(1, 10)
    for n in range(num_files):
        fill_file(os.path.join(path1, rand_file_name(n)))
    if level < maxlevel:
        num_dirs = random.randrange(1, 3)
        for n2 in range(num_dirs):
            subdir = os.path.join(path1, rand_dir_name(n2))
            os.mkdir(subdir)
            fill_dir(subdir, level + 1, maxlevel)

target="fixture"
date_format="%F_%H%M_%S"

if os.path.exists(target):
    shutil.rmtree(target)
os.mkdir(target)

num_dirs=30
for n in range(num_dirs):
    year = random.randrange(2012, 2022)
    month = random.randrange(1, 12)
    day = random.randrange(1, 28)
    hour = random.randrange(0, 23)
    minute = random.randrange(0, 59)
    second = random.randrange(0, 59)
    date = datetime.datetime(year, month, day, hour, minute, second)
    dname = os.path.join(target, date.strftime(date_format))
    os.mkdir(dname)
    fill_dir(dname, 1, 3)


