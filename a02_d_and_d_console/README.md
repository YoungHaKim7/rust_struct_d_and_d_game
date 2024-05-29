# Result

- miri 는 문제 없어 보이는데

```bash

$ MIRIFLAGS=-Zmiri-disable-isolation cargo miri run
Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/gy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/a02_d_and_d_console`
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
2
You run away!
You find a treasure chest.
You find a treasure worth 50 gold!
You are in an empty room.
The room is empty.
You have cleared the dungeon. Congratulations!
```

- valgrind에서는 leak발견 ㅠㅠ
  - `valgrind --leak-check=full --show-leak-kinds=all ./a02_d_and_d_console`


```bash
$ valgrind --leak-check=full --show-leak-kinds=all ./a02_d_and_d_console

==14159== Memcheck, a memory error detector
==14159== Copyright (C) 2002-2024, and GNU GPL'd, by Julian Seward et al.
==14159== Using Valgrind-3.23.0 and LibVEX; rerun with -h for copyright info
==14159== Command: ./a02_d_and_d_console
==14159==
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
==14159==
==14159== HEAP SUMMARY:
==14159==     in use at exit: 8,192 bytes in 1 blocks
==14159==   total heap usage: 24 allocs, 23 frees, 11,792 bytes allocated
==14159==
==14159== 8,192 bytes in 1 blocks are still reachable in loss record 1 of 1
==14159==    at 0x484680F: malloc (vg_replace_malloc.c:446)
==14159==    by 0x10F27F: alloc (alloc.rs:100)
==14159==    by 0x10F27F: alloc_impl (alloc.rs:183)
==14159==    by 0x10F27F: allocate (alloc.rs:243)
==14159==    by 0x10F27F: try_allocate_in<u8, alloc::alloc::Global> (raw_vec.rs:230)
==14159==    by 0x10F27F: with_capacity<u8> (raw_vec.rs:117)
==14159==    by 0x10F27F: new_uninit_slice<u8> (boxed.rs:636)
==14159==    by 0x10F27F: with_capacity (buffer.rs:34)
==14159==    by 0x10F27F: with_capacity<std::io::stdio::StdinRaw> (bufreader.rs:94)
==14159==    by 0x10F27F: {closure#0} (stdio.rs:337)
==14159==    by 0x10F27F: {closure#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}> (once_lock.rs:250)
==14159==    by 0x10F27F: {closure#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}>, !> (once_lock.rs:457)
==14159==    by 0x10F27F: {closure#0}<std::sync::once_lock::{impl#0}::initialize::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}>, !>> (once.rs:208)
==14159==    by 0x10F27F: std::sys::sync::once::futex::Once::call (futex.rs:124)
==14159==    by 0x10ED67: call_once_force<std::sync::once_lock::{impl#0}::initialize::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}>, !>> (once.rs:208)
==14159==    by 0x10ED67: std::sync::once_lock::OnceLock<T>::initialize (once_lock.rs:456)
==14159==    by 0x12D1EB: get_or_try_init<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}>, !> (once_lock.rs:338)
==14159==    by 0x12D1EB: get_or_init<std::sync::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>, std::io::stdio::stdin::{closure_env#0}> (once_lock.rs:250)
==14159==    by 0x12D1EB: std::io::stdio::stdin (stdio.rs:336)
==14159==    by 0x11252A: a02_d_and_d_console::main (main.rs:135)
==14159==    by 0x112F1A: core::ops::function::FnOnce::call_once (function.rs:250)
==14159==    by 0x112EDD: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:155)
==14159==    by 0x114970: std::rt::lang_start::{{closure}} (rt.rs:159)
==14159==    by 0x12B9EC: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:284)
==14159==    by 0x12B9EC: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:559)
==14159==    by 0x12B9EC: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:523)
==14159==    by 0x12B9EC: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:149)
==14159==    by 0x12B9EC: {closure#2} (rt.rs:141)
==14159==    by 0x12B9EC: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:559)
==14159==    by 0x12B9EC: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:523)
==14159==    by 0x12B9EC: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:149)
==14159==    by 0x12B9EC: std::rt::lang_start_internal (rt.rs:141)
==14159==    by 0x114949: std::rt::lang_start (rt.rs:158)
==14159==    by 0x112BBD: main (in /home/gy/my_project/Rust_Lang/111/rust_struct_d_and_d_game/a02_d_and_d_console/target/debug/a02_d_and_d_console)
==14159==
==14159== LEAK SUMMARY:
==14159==    definitely lost: 0 bytes in 0 blocks
==14159==    indirectly lost: 0 bytes in 0 blocks
==14159==      possibly lost: 0 bytes in 0 blocks
==14159==    still reachable: 8,192 bytes in 1 blocks
==14159==         suppressed: 0 bytes in 0 blocks
==14159==
==14159== For lists of detected and suppressed errors, rerun with: -s
==14159== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

```
