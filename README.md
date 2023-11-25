# Java classfile parser (WIP)
A java classfile parser written in Rust. Project for learning both Rust and the java classfile structure.

## Instructions
- [Optional] Compile java class `javac java/Main.java`
- Build: `cargo build`
- Run: `./target/debug/java-parser` or `cargo run` 

## TODO
- Parse complete file:
  - [x] magic;
  - [x] minor_version;
  - [x] major_version;
  - [x] constant_pool_count;
  - [x] constant_pool[constant_pool_count-1];
  - [x] access_flags;
  - [x] this_class;
  - [x] super_class;
  - [x] interfaces_count;
  - [ ] interfaces[interfaces_count];
  - [x] fields_count;
  - [ ] fields[fields_count];
  - [x] methods_count;
  - [x] methods[methods_count];
  - [x] attributes_count;
  - [x] attributes[attributes_count];
- [ ] Command line args instead of hardcoded file
- [ ] Create basic JVM?
