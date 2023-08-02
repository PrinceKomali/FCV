#!/usr/bin/env python3
import subprocess
from pathlib import Path
from time import sleep
from threading import Thread;
import math
import re
import os
import sys
from math import floor
procs = []
objs = []
srcs = []
color = True
tw = os.get_terminal_size().columns
w = int((tw - 2) / 4)
abort = False
if not os.path.exists("build"): os.mkdir("build")
jobs = [
    ("build/rust.a","rust/src","cargo build --manifest-path=rust/Cargo.toml --release; cp rust/target/release/lib_.a build/rust.a"),
    ("build/main.o", "src/main.d", "gdc -Isrc -c -o build/main.o src/main.d")
    

]
for job in jobs: 
    objs.append(job[0])
def mtime(p):
    if os.path.isfile(p): return os.path.getmtime(p)
    else:
        max_mt = 0
        for root, _, file in os.walk(p):
            f = os.path.join(root, file[0])
            mt = os.path.getmtime(f) 
            if mt > max_mt: max_mt = mt
        return max_mt
m = 0
for i in range(len(jobs.copy())):
    if os.path.exists(jobs[i-m][0]) and mtime(jobs[i-m][0]) > mtime(jobs[i-m][1]):
        jobs.pop(i - m)
        m += 1
for job in jobs:
    procs.append(job[2])
    srcs.append(job[1])
objs_copy = objs.copy()
processes=[]
i = 0
nproc = 16
complete = 0
l = len(procs)
print_queue = []
def compiled_message(complete):
    num_filled = int(((complete) / l) * w) if l > 0 else w
    num_blank = w - num_filled
    lb = "\n"
    msg = f"\x1b[36;1mCompiled {complete}/{l}\x1b[0m [{'='*num_filled}{'=' if l - 1 == i else '>'}{' ' * num_blank}] \r"
    print_queue.append(msg)
    return msg
def cmd_thread( cmd, obj, src):
    global abort
    global complete
    print_queue.append(f"\x1b[32;1mBuilding\x1b[0m: \x1b[33m{src}\x1b[34m -> \x1b[32m{obj}\x1b[0m\n")
    sp = subprocess.Popen(["/bin/sh", "-c", proc])
    sp.wait()
    if sp.returncode == 1: 
        abort = True
        return
    complete += 1
    compiled_message(complete)
while len(procs) > 0:
    while [ i.is_alive() for i in processes].count(None) > nproc:0
    proc = procs.pop(0)
    obj = objs.pop(0)
    src = srcs.pop(0)
    processes.append(Thread(target=cmd_thread, args=(proc, obj, src)))
    processes[-1].start()
    i += 1
while True in [ i.is_alive() for i in processes]:
    if len(print_queue) > 0: 
        print(print_queue.pop(0), end="")
    for thread in processes:
        if not thread.is_alive(): 
            thread.join()
print(compiled_message(complete), end="")
if abort: 
    print(f"Errors found, not linking{' ' * w}")
    exit(1)
elif l > 0:
    print("\nLinking final executable")
    objs_copy_copy = objs_copy.copy()
    objs_copy_copy.reverse()
    subprocess.run(("gdc " + " ".join(objs_copy_copy) + " -o main -lcurl").split(" "))
else: print("\nNo changes were made, not linking")