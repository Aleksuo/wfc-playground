{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/tasks/
  tasks."dev:fmt:check" = {
    exec = "cargo fmt --check";
    description = "Check Rust formatting";
  };

  tasks."dev:fmt:write" = {
    exec = "cargo fmt";
    description = "Format all Rust files";
  };

  tasks."dev:test" = {
    exec = "cargo test";
    description = "Run all rust tests";
    showOutput = true;
  };
}
