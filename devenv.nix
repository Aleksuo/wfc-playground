{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/tasks/
  tasks."dev:fmt:check" = {
    exec = "cargo fmt --check --color=always";
    description = "Check Rust formatting";
    showOutput = true;
  };

  tasks."dev:fmt:write" = {
    exec = "cargo fmt";
    description = "Format all Rust files";
    showOutput = true;
  };

  tasks."dev:test" = {
    exec = "cargo test --color=always";
    description = "Run all rust tests";
    showOutput = true;
  };

  tasks."dev:build:release" = {
    exec = "cargo build --release --color=always";
    description = "Build the project in release mode";
    showOutput = true;
  };

  tasks."dev:lint" = {
    exec = "cargo clippy --color=always -- -D warnings";
    description = "Run clippy lints";
    showOutput = true;
  };
}
