# foobar_lings

## Project Description

Welcome to `foobar_lings`!

To use `foobar_lings`, you need to have Rust installed. You can get Rust by visiting https://rustup.rs. This will also install Cargo, Rust's package and project manager.

This repository is a customizable boilerplate for [`rustlings`](https://github.com/rust-lang/rustlings), a Rust project containing small exercises designed to help you learn Rust by reading and writing code. With `foobar_lings`, you can easily create your own Rust exercises by adding them to the exercise folder, and others can use your exercises to practice their Rust skills.

Here are some examples of what you can do with `foobar_lings`:

- Create exercises that focus on specific Rust concepts, such as ownership, lifetimes, or traits.

- Define challenges that require the use of Rust libraries or external APIs.

- Design exercises that simulate real-world scenarios, such as building a command-line tool or a web service.

- Collaborate with others to create and solve Rust exercises together.

By using `foobar_lings`, you have the flexibility to create exercises that cater to your specific needs and interests. Happy learning!

## Usage

### Show the Executable Version

To show the version of the executable, run the following command:

```bash
cargo run -- -v
```

### Show a Welcome Message

To show a welcome message, you can generate your own welcome message by using [Text to ASCII gengerator](http://patorjk.com/software/taag/#p=display&f=Slant&t=Composer)

<p>run the following command:</p>

```bash
cargo run
```

### Process Subcommands

`foobar_lings` includes several subcommands to help you manage your exercises. Here's how to use them:

- Verify

Use the `verify` subcommand to check that the provided container of exercise objects can be compiled and run without any failures. If there are any failures, they will be reported to the user. If the exercise being verified is a test, the nocapture boolean command argument determines whether the test harness outputs are displayed.

To run the verify subcommand, run the following command:

```bash
cargo run -- verify
```

- Watch

The `watch` command in foobar_lings is similar to the `verify` command, but it runs the exercises in the recommended order and doesn't exit until all exercises have been completed. This makes it a great tool for practicing Rust syntax and concepts in a structured way.

With the watch command, you can:

- Complete the Rust exercises in a recommended order to build your understanding of Rust syntax and concepts gradually.

- Receive immediate feedback on your code as you work through each exercise.

- Monitor your progress and see which exercises you have completed and which ones you need to work on.

To move on to the next exercise, you simply need to remove the "I AM NOT DONE" comment and make sure that your code compiles and runs as expected. Once you have completed an exercise, you can move on to the next one and continue.

Overall, the watch command provides a structured and efficient way to practice Rust and improve your skills.

To use the watch subcommand, run the following command:

```bash
cargo run -- watch
```

- Run

Use the run subcommand to invoke the Rust compiler on the path of the given exercise.

To use the run subcommand, run the following command:

```bash
cargo run -- run hello-world
```

- Reset
  The reset subcommand resets the exercise by stashing the changes.

To use the reset subcommand, run the following command:

```bash
cargo run -- reset hello-world
```

- Hint
  The hint subcommand provides hints for the given exercise.

To use the hint subcommand, run the following command:

```bash
cargo run -- hint hello-world
```

- LSP

The lsp subcommand generates a rust-project.json file at the root of the project, which allows Rust Analyzer to parse each exercise.

To use the lsp subcommand, run the following command:

```bash
cargo run -- lsp
```

- List

The list subcommand lists all the exercises, including their name, path, and status, in a table. You can use subarguments to filter by exercise name and status.

To use the list subcommand, run the following command:

```bash
cargo run -- list
```

You can customize your own subcommands or change the actions of the above subcommands.

### Install the Binaries for the Package

To install the binary executable for the `foobar_lings` package, you need to add it to Cargo.toml:

```toml
[[bin]]
name = "foobarlings"
path = "src/main.rs"
```

Then, you can install it by running the following command:

```bash
cargo install --force --path .
```

Now, you can use foobarlings instead of cargo run -- in the above commands. Remember to reinstall the binaries every time you make changes to the src folder.
