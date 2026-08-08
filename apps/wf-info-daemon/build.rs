use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

const BRIDGE_TARGET: &str = "x86_64-pc-windows-gnu";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=../wf-dbwin-bridge/src");
    println!("cargo::rerun-if-changed=../wf-dbwin-bridge/Cargo.toml");
    println!("cargo::rerun-if-changed=../../crates/wf-dbwin/src");
    println!("cargo::rerun-if-changed=../../crates/wf-dbwin/Cargo.toml");
    println!("cargo::rerun-if-changed=../../Cargo.lock");

    // On windows hosts the bridge is built natively as a workspace member.
    if env::var("CARGO_CFG_UNIX").is_err() {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let bridge_target_dir = out_dir.join("wf-dbwin-bridge-target");

    let output = Command::new(cargo)
        .args([
            "build",
            "--release",
            "-p",
            "wf-dbwin-bridge",
            "--target",
            BRIDGE_TARGET,
            "--target-dir",
        ])
        .arg(&bridge_target_dir)
        // Flags meant for the host target must not leak into the cross build.
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTFLAGS")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{stderr}");
        if stderr.contains("target may not be installed")
            || stderr.contains("can't find crate for `std`")
        {
            return Err(format!(
                "The `{BRIDGE_TARGET}` rust target is missing. \
                 Install it with: rustup target add {BRIDGE_TARGET}"
            )
            .into());
        }
        if stderr.contains("linker `x86_64-w64-mingw32-gcc` not found") {
            return Err(
                "The mingw-w64 cross linker is missing. Install your distro's \
                 mingw-w64 gcc package (e.g. `mingw-w64-gcc` on Arch, \
                 `gcc-mingw-w64-x86-64` on Debian/Ubuntu)."
                    .into(),
            );
        }
        return Err(format!(
            "cross-building wf-dbwin-bridge for {BRIDGE_TARGET} failed (see above)"
        )
        .into());
    }

    let exe = bridge_target_dir
        .join(BRIDGE_TARGET)
        .join("release")
        .join("wf-dbwin-bridge.exe");
    println!("cargo::rustc-env=WF_DBWIN_BRIDGE_EXE={}", exe.display());

    Ok(())
}
