# Result(Leak 해결 했지만.... 게임이 진행이 안됨 ㅠㅠ)

```bash

$ MIRIFLAGS=-Zmiri-disable-isolation cargo miri run

Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
   Compiling a04_d_and_d_weak_error v0.1.0 (/home/gy/my_project/Rust_Lang/111/rust_struct_d_and_d_game/a04_d_and_d_weak_error)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `/home/gy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/a04_d_and_d_weak_error`

Welcome to the dungeon, Hero!
You are in a dark cave.
The monster is no longer here.
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
```

- 

```bash
valgrind --leak-check=full --show-leak-kinds=all ./a04_d_and_d_weak_error
==13369== Memcheck, a memory error detector
==13369== Copyright (C) 2002-2024, and GNU GPL'd, by Julian Seward et al.
==13369== Using Valgrind-3.23.0 and LibVEX; rerun with -h for copyright info
==13369== Command: ./a04_d_and_d_weak_error
==13369==
Welcome to the dungeon, Hero!
You are in a dark cave.
The monster is no longer here.
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
==13369==
==13369== HEAP SUMMARY:
==13369==     in use at exit: 0 bytes in 0 blocks
==13369==   total heap usage: 21 allocs, 21 frees, 3,573 bytes allocated
==13369==
==13369== All heap blocks were freed -- no leaks are possible
==13369==
==13369== For lists of detected and suppressed errors, rerun with: -s
==13369== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

