use std::process::Command;
use std::str::from_utf8;
use std::{env, fs};

fn main() {
    println!("ℹ️ Parse git-cliff version");

    let version = Command::new("cargo")
        .args(["bin", "git-cliff", "--bumped-version"])
        .output()
        .expect("Failed to get version with git-cliff");
    let version = from_utf8(&version.stdout).expect("✖️ Failed to convert version to string").trim();

    println!("👌 The new version is: {}", version);

    println!("ℹ️ Check Github Token environment variable");
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable is not set");

    println!("ℹ️ Update Cargo.toml version");
    update_cargo_toml_version(version);

    println!("ℹ️ Update Cargo.lock");
    Command::new("cargo").args(["generate-lockfile"]).output().expect("✖️ Failed to update Cargo.lock");

    println!("ℹ️ Generate changelog");
    Command::new("cargo")
        .args(["bin", "git-cliff", "--bump", "--output", "CHANGELOG.md"])
        .output()
        .expect("✖️ Failed to generate changelog with git-cliff");

    println!("⌛ Commiting changes");
    Command::new("git")
        .args([
            "commit",
            "-am",
            format!("chore(release): bump to version {}", version).as_str(),
        ])
        .output()
        .expect("✖️ Failed to commit the new version");

    println!("⌛ Tagging the new version");
    Command::new("git").args(["tag", version]).output().expect("✖️ Failed to tag the new version");

    println!("⌛ Pushing the new version to the repository");
    Command::new("git").args(["push", "origin", "main"]).output().expect("✖️ Failed to push the new version");

    println!("⌛ Pushing the new tag to the repository");
    Command::new("git").args(["push", "origin", "tag", version]).output().expect("✖️ Failed to push the new tag");

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
