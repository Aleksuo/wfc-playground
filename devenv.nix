{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.samply
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/tasks/
  tasks."fmt:check" = {
    exec = "cargo fmt --check";
    description = "Check Rust formatting";
    showOutput = true;
  };

  tasks."fmt:write" = {
    exec = "cargo fmt";
    description = "Format all Rust files";
    showOutput = true;
  };

  tasks."test:wfc" = {
    exec = "cargo test -p wfc --color=always";
    description = "Run tests for wfc";
    showOutput = true;
  };

  tasks."test:wfc-cli" = {
    exec = "cargo test -p wfc-cli --color=always";
    description = "Run tests for wfc-cli";
    showOutput = true;
  };

  tasks."build:release" = {
    exec = "cargo build --release --color=always";
    description = "Build the project in release mode";
    showOutput = true;
  };

  tasks."lint:wfc" = {
    exec = "cargo clippy -p wfc --color=always -- -D warnings";
    description = "Run clippy lints for wfc";
    showOutput = true;
  };

  tasks."lint:wfc-cli" = {
    exec = "cargo clippy -p wfc-cli --color=always -- -D warnings";
    description = "Run clippy lints for wfc-cli";
    showOutput = true;
  };

  tasks."profile:wfc-cli" = {
    exec = "cargo build --profile profiling -p wfc-cli --color=always && samply record ./target/release/wfc-cli";
    description = "Profile wfc-cli with samply";
    showOutput = true;
  };

  tasks."bench:wfc" = {
    exec = "cargo bench -p wfc";
    description = "Run benchmarks for wfc crate";
    showOutput = true;
  };
}
