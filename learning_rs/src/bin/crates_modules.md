````sh
    This is the general structure of a rust project.
    It can have atmost one main.rs and  one lib.rs in src directory
    But can have as many as binary crates possible in the bin directory
.
├── Cargo.toml
├── Cargo.lockp
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # directory for integrated tests files
│   └── some_integration_tests.rs
├── benches # dir for benchmark files
│   └── simple_bench.rs
└── examples # dir for example files
    └── simple_example.rs```
````

for more in depth info, refer to these examples:
https://practice.course.rs/crate-module/module.html

https://github.com/sunface/rust-by-practice/blob/master/solutions/crate-module/module.md
