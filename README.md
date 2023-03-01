# \_foobar_lings

## New Project

```bash
cargo new _foobar_lings
cd _foobar_lings
```

Add dependencies to Cargo.toml

```toml
[dependencies]
argh = "0.1"
```

Decalre command and subcommand structs and process commands in main.rs

### show the executable version

```bash
cargo run -- -v
```

### show WELCOME to user

Use a [Text to ASCII gengerator](http://patorjk.com/software/taag/#p=display&f=Slant&t=Composer) to obtain your favorite WELCOME message.

```bash
cargo run
```

### Exit if info.toml doesn't exist

new a info.toml file including [[exercises]] info

### Exit if rustc not installed

```bash
rustc --version
```

### Deserialize info.toml

Add dependencies to Cargo.toml

```toml
[dependencies]
toml = "0.5"
serde = { version = "1.0", features = ["derive"] }
```

create a new module `'exercise'`, declare the struct `Exercise` and Deserialize to `Exercise` list.

```rust
let toml_str = &fs::read_to_string("info.toml").unwrap();

let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
```

### Install the binaries for the package

> every time you make changes to `'src'` remember to reinstall the binaries

Add binary executable to Cargo.toml

```toml
[[bin]]
name = "foobarlings"
path = "src/main.rs"
```

install

```bash
cargo install --force --path .
```

you can use `'foobarlings'` instead of `'cargo run --'` now in the above command.
