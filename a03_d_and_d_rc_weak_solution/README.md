# Result

- leak은 해결했지만 게임 진입에 실패 ㅠㅠ

```bash
$ MIRIFLAGS=-Zmiri-disable-isolation cargo miri run

Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/gy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/a03_d_and_d_rc_weak_solution`

Welcome to the dungeon, Hero!
You are in a dark cave.
The monster is no longer here.
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
```

```bash

$ valgrind --leak-check=full --show-leak-kinds=all ./a03_d_and_d_rc_weak_solution

==18199== Memcheck, a memory error detector
==18199== Copyright (C) 2002-2024, and GNU GPL'd, by Julian Seward et al.
==18199== Using Valgrind-3.23.0 and LibVEX; rerun with -h for copyright info
==18199== Command: ./a03_d_and_d_rc_weak_solution
==18199==
Welcome to the dungeon, Hero!
You are in a dark cave.
The monster is no longer here.
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
==18199==
==18199== HEAP SUMMARY:
==18199==     in use at exit: 0 bytes in 0 blocks
==18199==   total heap usage: 20 allocs, 20 frees, 3,576 bytes allocated
==18199==
==18199== All heap blocks were freed -- no leaks are possible
==18199==
==18199== For lists of detected and suppressed errors, rerun with: -s
==18199== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

