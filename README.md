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
  - [ ] access_flags;
  - [ ] this_class;
  - [ ] super_class;
  - [ ] interfaces_count;
  - [ ] interfaces[interfaces_count];
  - [ ] fields_count;
  - [ ] fields[fields_count];
  - [ ] methods_count;
  - [ ] methods[methods_count];
  - [ ] attributes_count;
  - [ ] attributes[attributes_count];
- [ ] Command line args instead of hardcoded file
- [ ] Create basic JVM?
