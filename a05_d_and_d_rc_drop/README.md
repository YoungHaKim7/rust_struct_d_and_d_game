# Result

- 그나마 오류 메세지 약간 줄임

```bash

$ valgrind --leak-check=full --show-leak-kinds=all ./a05_d_and_d_rc_drop

==2727== Memcheck, a memory error detector
==2727== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==2727== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==2727== Command: ./a05_d_and_d_rc_drop
==2727==
==2727== error calling PR_SET_PTRACER, vgdb might block
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
1
You attack the Goblin. It now has 0 health.
You have defeated the Goblin!
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
Game over.
==2727==
==2727== HEAP SUMMARY:
==2727==     in use at exit: 8,192 bytes in 1 blocks
==2727==   total heap usage: 22 allocs, 21 frees, 14,848 bytes allocated
==2727==
==2727== 8,192 bytes in 1 blocks are still reachable in loss record 1 of 1
==2727==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2727==    by 0x14349F: std::sys::sync::once::futex::Once::call (alloc.rs:100)
==2727==    by 0x13E947: std::sync::once_lock::OnceLock<T>::initialize (once.rs:208)
==2727==    by 0x13B66B: std::io::stdio::stdin (once_lock.rs:338)
==2727==    by 0x121475: a05_d_and_d_rc_drop::main (main.rs:126)
==2727==    by 0x11E39A: core::ops::function::FnOnce::call_once (function.rs:250)
==2727==    by 0x11F56D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:155)
==2727==    by 0x11F350: std::rt::lang_start::{{closure}} (rt.rs:159)
==2727==    by 0x139D7C: std::rt::lang_start_internal (function.rs:284)
==2727==    by 0x11F329: std::rt::lang_start (rt.rs:158)
==2727==    by 0x121EFD: main (in /home/y/my_project/rust_lang/rust_struct_d_and_d_game/a05_d_and_d_rc_drop/target/debug/a05_d_and_d_rc_drop)
==2727==
==2727== LEAK SUMMARY:
==2727==    definitely lost: 0 bytes in 0 blocks
==2727==    indirectly lost: 0 bytes in 0 blocks
==2727==      possibly lost: 0 bytes in 0 blocks
==2727==    still reachable: 8,192 bytes in 1 blocks
==2727==         suppressed: 0 bytes in 0 blocks
==2727==
==2727== For lists of detected and suppressed errors, rerun with: -s
==2727== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

