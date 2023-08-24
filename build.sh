#!/bin/bash

sudo systemctl start docker

cargo check | exit 2

cross build --target-dir win-student --release --target x86_64-pc-windows-gnu | exit 1
cross build --target-dir win-teacher --features teacher --release --target x86_64-pc-windows-gnu | exit 1

cargo build --target-dir lin-student/ --release | exit 1
cargo build --target-dir lin-teacher --features teacher --release | exit 1

mv lin-student/release/turing-machine ./turing-machine_student-linux
mv lin-teacher/release/turing-machine ./turing-machine_teacher-linux
mv win-student/x86_64-pc-windows-gnu/release/turing-machine.exe ./turing-machine_student-windows.exe
mv win-teacher/x86_64-pc-windows-gnu/release/turing-machine.exe ./turing-machine_teacher-windows.exe
