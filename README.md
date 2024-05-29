# rust_struct_d_and_d_game


# 러스트 컴파일 빠르게 세팅 & nightly 사용
- Faster compilation with the parallel front-end in nightly
  - https://blog.rust-lang.org/2023/11/09/parallel-rustc.html


# cargo miri
- https://github.com/rust-lang/miri

- https://rust-unofficial.github.io/too-many-lists/fifth-miri.html

```
cargo miri test
```


```
MIRIFLAGS=-Zmiri-backtrace=full cargo miri run
```
