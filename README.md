# Silence test

## Prologue
Current repo is `ZK knowledge proof` silence test task.

## Task Specifications
```
1: ZK knowledge proof

Please find the attached Python source file. Please implement the same primitive in Rust. 

Please use any suitable from RustCrypto. 
``` 
Full task specification you can find [here](https://www.python.org/downloads/).


**Read-Only Files**
- .gitignore
- Cargo.toml
- Cargo.lock
- README.md

**Environment**  
First of all you have to make some required installations. To make it relevant to your OS please visit official websites: <br>

1) `Rust` programming language - [official download](https://www.rust-lang.org/tools/install). Make sure `cargo` also has been installed. 

2) Also you may need to install `Python` programming language to make sure you both of your's `Rust` and `Python` work - [official download](https://www.python.org/downloads/). Actually 3.10+ version


**Usage Example**

To execute `main.rs` script first of all you need to make build. Go to `silence_test` dir:

```bash
$ cd <full_path_to_repo>/silence_test
```
then:
```bash
$ cargo build
```
After your build has been done, just execute build file with next command:
```bash
$ .target/debug/silence_test
```

Also you can execute python `dlog_proof.py` script to make sure both of scripts work. To do that just just ececute next command:
then:
```bash
$ python3 ./src/dlog_proof.py
```
