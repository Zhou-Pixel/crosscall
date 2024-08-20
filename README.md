# Crosscall

**Interaction and communication between Rust and Flutter using grpc and memory channels**



# Requirements 
1. rust toolchain
2. flutter toolchain
3. protoc in `$PATH`
4. protoc dart plugin int `$PATH`






# Quick start
1. **Install rust toolchain**
    - follow <a href='https://www.rust-lang.org/tools/install'>link</a>
2. **Install flutter toolchain**
    - follow <a href='https://docs.flutter.dev/get-started/install'>link</a>

3. **Install protoc and dart plugin**
    - follow <a href='https://grpc.io/docs/languages/dart/quickstart/'>link</a>

4. Install `crossc`
    - `cargo install crossc`
    - `crossc check`: Check flutter and protc installation

5. Create project
    - `crossc new my_app`
    - `cd my_app && flutter run`

