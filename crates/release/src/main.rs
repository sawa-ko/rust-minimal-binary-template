use std::process::Command;
use std::str::from_utf8;
use std::{env, fs};

fn main() {
    println!("ℹ️ Check if the create cargo-run-bin is installed");

    let cargo_run_bin = Command::new("cargo").args(["bin"]).output().expect("✖️ cargo-run-bin crate is not installed");

    if cargo_run_bin.status.code().expect("✖️ Failed to check cargo-run-bin code") != 0 {
        eprintln!("✖️ cargo-run-bin crate is not installed");
        std::process::exit(1);
    }

    println!("ℹ️ Parse git-cliff version");

    if Command::new("git-cliff").status().is_err() {
        eprintln!(
            "✖️ git-cliff command not found. Please install git-cliff (https://git-cliff.org/docs/installation)."
        );
        std::process::exit(1);
    }

    let version = Command::new("cargo")
        .args(["bin", "git-cliff", "--bumped-version"])
        .output()
        .expect("Failed to get version with git-cliff");

    if version.status.code().expect("✖️ Failed to get version code") != 0 {
        eprintln!("✖️ Failed to get version with git-cliff");
        std::process::exit(1);
    }

    let version = from_utf8(&version.stdout).expect("✖️ Failed to convert version to string").trim();

    println!("👌 The new version is: {}", version);

    println!("ℹ️ Check Github Token environment variable");
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable is not set");

    println!("ℹ️ Update Cargo.toml version");
    update_cargo_toml_version(version);

    println!("ℹ️ Update Cargo.lock");

    let update_cargo_lock =
        Command::new("cargo").args(["generate-lockfile"]).output().expect("✖️ Failed to update Cargo.lock");

    if update_cargo_lock.status.code().expect("✖️ Failed to update Cargo.lock code") != 0 {
        eprintln!("✖️ Failed to update Cargo.lock");
        std::process::exit(1);
    }

    println!("ℹ️ Generate changelog");

    let generate_changelog = Command::new("cargo")
        .args(["bin", "git-cliff", "--bump", "--output", "CHANGELOG.md"])
        .output()
        .expect("✖️ Failed to generate changelog with git-cliff");

    if generate_changelog.status.code().expect("✖️ Failed to generate changelog code") != 0 {
        eprintln!("✖️ Failed to generate changelog with git-cliff");
        std::process::exit(1);
    }

    println!("⌛ Commiting changes");

    let commit_changes = Command::new("git")
        .args([
            "commit",
            "-am",
            format!("chore(release): bump to version {}", version).as_str(),
        ])
        .output()
        .expect("✖️ Failed to commit the new version");

    if commit_changes.status.code().expect("✖️ Failed to commit the new version code") != 0 {
        eprintln!("✖️ Failed to commit the new version");
        std::process::exit(1);
    }

    println!("⌛ Tagging the new version");

    let tagging_version =
        Command::new("git").args(["tag", version]).output().expect("✖️ Failed to tag the new version");

    if tagging_version.status.code().expect("✖️ Failed to tag the new version code") != 0 {
        eprintln!("✖️ Failed to tag the new version");
        std::process::exit(1);
    }

    println!("⌛ Pushing the new version to the repository");

    let git_push =
        Command::new("git").args(["push", "origin", "main"]).output().expect("✖️ Failed to push the new version");

    if git_push.status.code().expect("✖️ Failed to push the new version code") != 0 {
        eprintln!("✖️ Failed to push the new version");
        std::process::exit(1);
    }

    println!("⌛ Pushing the new tag to the repository");

    let git_push_tag =
        Command::new("git").args(["push", "origin", "tag", version]).output().expect("✖️ Failed to push the new tag");

    if git_push_tag.status.code().expect("✖️ Failed to push the new tag code") != 0 {
        eprintln!("✖️ Failed to push the new tag");
        std::process::exit(1);
    }

    println!("✅ Release process completed successfully");
}

fn update_cargo_toml_version(new_version: &str) {
    let cargo_toml_path = "Cargo.toml";
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).expect("✖️ Failed to read Cargo.toml");

    let mut updated_content = String::new();
    let mut in_package_section = false;

    for line in cargo_toml_content.lines() {
        if line.starts_with("[package]") {
            in_package_section = true;
            updated_content.push_str(line);
            updated_content.push('\n');
        } else if in_package_section && line.starts_with("version") {
            updated_content.push_str(&format!("version = \"{}\"\n", new_version.replace('v', "")));
        } else {
            updated_content.push_str(line);
            updated_content.push('\n');
        }
    }

    fs::write(cargo_toml_path, updated_content).expect("✖️ Failed to write Cargo.toml");
}
