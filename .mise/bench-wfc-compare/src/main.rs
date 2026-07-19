use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};
use std::time::{SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let against = env::args().nth(1).ok_or("expected head or branch")?;
    let repo = PathBuf::from(git(None, &["rev-parse", "--show-toplevel"])?);

    let (label, commit) = match against.as_str() {
        "head" => (
            "HEAD".to_owned(),
            git(Some(&repo), &["rev-parse", "HEAD^{commit}"])?,
        ),
        "branch" => {
            let branch = default_branch(&repo)?;
            let commit = git(Some(&repo), &["merge-base", "HEAD", &branch])?;
            (branch, commit)
        }
        _ => return Err("expected head or branch".into()),
    };

    let short_commit = git(Some(&repo), &["rev-parse", "--short", &commit])?;
    let target = repo.join("target").join("bench-comparison");
    let marker = target.join("baselines").join(&commit);
    let baseline = format!("git-{commit}");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    if marker.exists() {
        println!("Using cached baseline {short_commit}");
    } else {
        println!("Creating baseline {short_commit} (output hidden)");
        create_baseline(&repo, &target, &cargo, &commit, &baseline)?;
        fs::create_dir_all(marker.parent().unwrap())?;
        fs::write(&marker, [])?;
    }

    println!("Comparing working tree with {label} at {short_commit}");
    benchmark(&cargo, &repo, &target, "--baseline", &baseline, false)?;

    Ok(())
}

fn create_baseline(
    repo: &Path,
    target: &Path,
    cargo: &OsStr,
    commit: &str,
    baseline: &str,
) -> Result<()> {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let worktree = repo
        .join(".output")
        .join("bench-worktrees")
        .join(format!("{}-{unique}", std::process::id()));

    fs::create_dir_all(worktree.parent().unwrap())?;

    let mut command = Command::new("git");
    command
        .current_dir(repo)
        .args(["worktree", "add", "--detach"])
        .arg(&worktree)
        .arg(commit);
    hidden(command.output()?, "creating temporary worktree")?;

    let _guard = Worktree::new(repo, &worktree);
    benchmark(cargo, &worktree, target, "--save-baseline", baseline, true)
}

fn benchmark(
    cargo: &OsStr,
    working_dir: &Path,
    target: &Path,
    flag: &str,
    baseline: &str,
    hide_output: bool,
) -> Result<()> {
    let mut command = Command::new(cargo);
    command
        .current_dir(working_dir)
        .env("CARGO_TARGET_DIR", target)
        .args([
            "bench",
            "--profile",
            "release",
            "-p",
            "wfc",
            "--bench",
            "wfc_bench",
            "--",
            flag,
            baseline,
        ]);

    if hide_output {
        hidden(command.output()?, "running baseline")
    } else {
        success(command.status()?, "running comparison")
    }
}

fn default_branch(repo: &Path) -> Result<String> {
    if let Some(branch) = try_git(
        repo,
        &["symbolic-ref", "--quiet", "refs/remotes/origin/HEAD"],
    )? {
        return Ok(branch);
    }

    for branch in ["origin/main", "main", "origin/master", "master"] {
        if try_git(repo, &["rev-parse", "--verify", "--quiet", branch])?.is_some() {
            return Ok(branch.to_owned());
        }
    }

    Err("could not determine the default branch".into())
}

fn git(repo: Option<&Path>, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(repo) = repo {
        command.current_dir(repo);
    }
    let output = command.args(args).output()?;
    success(output.status, &format!("git {}", args.join(" ")))?;
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn try_git(repo: &Path, args: &[&str]) -> Result<Option<String>> {
    let output = Command::new("git").current_dir(repo).args(args).output()?;
    Ok(output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned()))
}

fn hidden(output: Output, action: &str) -> Result<()> {
    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    }
    success(output.status, action)
}

fn success(status: ExitStatus, action: &str) -> Result<()> {
    status
        .success()
        .then_some(())
        .ok_or_else(|| format!("{action} failed with {status}").into())
}

struct Worktree<'a> {
    repo: &'a Path,
    path: &'a Path,
}

impl<'a> Worktree<'a> {
    fn new(repo: &'a Path, path: &'a Path) -> Self {
        Self { repo, path }
    }
}

impl Drop for Worktree<'_> {
    fn drop(&mut self) {
        let status = Command::new("git")
            .current_dir(self.repo)
            .args(["worktree", "remove", "--force"])
            .arg(self.path)
            .status();
        if !matches!(status, Ok(status) if status.success()) {
            eprintln!("warning: could not remove {}", self.path.display());
        }
    }
}
