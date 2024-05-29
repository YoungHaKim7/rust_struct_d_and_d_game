# rust_struct_d_and_d_game

# 유료 강의 소개240529
- 240529러스트1기_day6_struct_impl_Ubuntu24.04
  - https://youtube.com/live/pU7V6r26SBg?feature=share

  - Rust디스코드 강의 책 - https://economiceco.tistory.com/m/19359


<hr> 

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

<hr>

# Rust기초가 약해서 무너진Case ㅠㅠ 마지막 고비 동시성에서 무너지네요ㅠㅠ 동시성 마지막만 넘으면 러스타시안이 되는건데 아쉽 아쉽..ㅠㅠ

# **[GN⁺: Rust로 게임 개발을 한 3년 후에 떠나며](<https://news.hada.io/topic?id=14521&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- Rust에 익숙해지면 모든 문제가 사라질 것이라는 주장에 대해  
  - Rust에 익숙해져도 근본적인 문제는 사라지지 않음  
  - 게임은 복잡한 상태 머신이고 요구사항이 계속 바뀌기 때문에 Rust의 정적이고 과도하게 검사하는 특성과 맞지 않음  
  - 코드를 계속 리팩토링해야 하는 문제는 self-inflicted임  
- ...
