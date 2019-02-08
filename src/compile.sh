# This is a mess, needs proper build system.

rm main.o main.c main.prg

~/git/rust-c64/mrustc/bin/mrustc -L /home/tibor/git/rust-c64/mrustc/output/ main.rs

~/git/rust-c64/cc65/bin/cl65 main.c -o main.prg && x64 main.prg
