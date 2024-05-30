# Result

- arc로 바꿔도 나오네 ..원래 나오는건가??

```bash

$ valgrind --leak-check=full --show-leak-kinds=all ./a06_arc_d_and_d

==1166== Memcheck, a memory error detector
==1166== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==1166== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==1166== Command: ./a06_arc_d_and_d
==1166==
==1166== error calling PR_SET_PTRACER, vgdb might block
Welcome to the dungeon, Hero!
You are in a dark cave.
You encounter a Goblin!
1. Attack
2. Run
1
You attack the Goblin. It now has 20 health.
The Goblin attacks you. You now have 95 health.
You encounter a Goblin!
1. Attack
2. Run
1
You attack the Goblin. It now has 10 health.
The Goblin attacks you. You now have 90 health.
You encounter a Goblin!
1. Attack
2. Run
2
You run away!
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
Game over.
==1166==
==1166== HEAP SUMMARY:
==1166==     in use at exit: 8,192 bytes in 1 blocks
==1166==   total heap usage: 23 allocs, 22 frees, 14,845 bytes allocated
==1166==
==1166== 8,192 bytes in 1 blocks are still reachable in loss record 1 of 1
==1166==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-a
md64-linux.so)
==1166==    by 0x10F501: std::sys::sync::once::futex::Once::call (alloc.rs:100)
==1166==    by 0x10EE09: std::sync::once_lock::OnceLock<T>::initialize (once.rs
:208)
==1166==    by 0x12F198: std::io::stdio::stdin (once_lock.rs:298)
==1166==    by 0x1145E1: a06_arc_d_and_d::main (main.rs:127)
==1166==    by 0x112B8A: core::ops::function::FnOnce::call_once (function.rs:25
0)
==1166==    by 0x1159AD: std::sys_common::backtrace::__rust_begin_short_backtra
ce (backtrace.rs:155)
==1166==    by 0x115DE0: std::rt::lang_start::{{closure}} (rt.rs:166)
==1166==    by 0x12DAA2: std::rt::lang_start_internal (function.rs:284)
==1166==    by 0x115DB9: std::rt::lang_start (rt.rs:165)
==1166==    by 0x11521D: main (in /home/y/my_project/rust_lang/111111ru/rust_st
ruct_d_and_d_game/a06_arc_d_and_d/target/debug/a06_arc_d_and_d)
==1166==
==1166== LEAK SUMMARY:
==1166==    definitely lost: 0 bytes in 0 blocks
==1166==    indirectly lost: 0 bytes in 0 blocks
==1166==      possibly lost: 0 bytes in 0 blocks
==1166==    still reachable: 8,192 bytes in 1 blocks
==1166==         suppressed: 0 bytes in 0 blocks
==1166==
==1166== For lists of detected and suppressed errors, rerun with: -s
==1166== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

```


- `MIRIFLAGS=-Zmiri-disable-isolation cargo miri run`

```bash
MIRIFLAGS=-Zmiri-disable-isolation cargo miri run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/y/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/c
argo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/a06_arc_d_and_d`
Welcome to the dungeon, Hero!
You are in a dark cave.
You encounter a Goblin!
1. Attack
2. Run
1
You attack the Goblin. It now has 20 health.
The Goblin attacks you. You now have 95 health.
You encounter a Goblin!
1. Attack
2. Run
1
You attack the Goblin. It now has 10 health.
The Goblin attacks you. You now have 90 health.
You encounter a Goblin!
1. Attack
2. Run
2
You run away!
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
Game over.

```
