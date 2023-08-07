use std::env;

use xshell::cmd;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {}", err);
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("test") => test()?,
        Some("generate") => generate()?,
        _ => {
            print_usage();
            Err("invalid arguments")?
        }
    }

    Ok(())
}

fn test() -> Result<()> {
    let what = match env::args().nth(2) {
        Some(it) => it,
        None => "all".into(),
    };

    let available = ["all", "swift", "kotlin", "python"];
    if !available.contains(&&*what) {
        print_usage();
        Err("invalid test target")?;
    }

    // Required for Kotlin to load jna.jar
    env::set_var("CLASSPATH", "jna.jar");

    match &*what {
        "all" => cmd!("cargo test -p rocketscience --tests").run()?,
        _ => {
            let test = format!("generated_{}_bindings", what);
            cmd!("cargo test -p rocketscience --test {test}").run()?
        }
    }

    Ok(())
}

fn generate() -> Result<()> {
    cmd!("cargo build").run()?;
    #[cfg(target_os = "macos")]
    let ext = "dylib";
    #[cfg(windows)]
    let ext = "dll";
    #[cfg(not(any(windows, target_os = "macos")))]
    let ext = "so";

    let library = format!("target/debug/librocketscience.{ext}");
    for lang in ["kotlin", "swift", "python"] {
        cmd!("cargo run -p uniffi-bindgen generate --library {library} --language {lang} --out-dir bindings").run()?;
    }

    Ok(())
}

fn print_usage() {
    eprintln!(
        "\
Usage: cargo run -p xtask <SUBCOMMAND>
SUBCOMMANDS:
    test [all|python|kotlin|swift]
    generate
"
    )
}
