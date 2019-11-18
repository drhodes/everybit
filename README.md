[+Project 1 of Performance Engineering of Software Systems](https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-172-performance-engineering-of-software-systems-fall-2018/projects/)

Ported to rust.

``` 
├── benches
│   ├── my_benchmark.rs
│   └── rotate_bench.rs
├── Cargo.lock
├── Cargo.toml
├── clean.bash
├── Makefile
├── README.org
└── src
    ├── bitarray.rs
    ├── lib.rs
    ├── main.rs
    └── modulo.c
```

Using criterion for benchmarking, it's fantastic, check this out:
It generates reports and maintains a history for comparing changes.

![alt text](https://i.imgur.com/BL0ookS.jpg)
