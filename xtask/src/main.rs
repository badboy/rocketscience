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
    match env::args().nth(1) {
        Some(it) if it == "test" => (),
        _ => {
            print_usage();
            Err("invalid arguments")?
        }
    }

    let what = match env::args().nth(2) {
        Some(it) => it,
        None => "all".into(),
    };

    let available = ["all", "swift", "kotlin", "python"];
    if !available.contains(&&*what) {
        print_usage();
        Err("invalid test target")?;
    }

    match &*what {
        "all" => cmd!("cargo test -p rocketscience --tests").run()?,
        _ => {
            let test = format!("generated_{}_bindings", what);
            cmd!("cargo test -p rocketscience --test {test}").run()?
        }
    }

    Ok(())
}


fn print_usage() {
    eprintln!(
        "\
Usage: cargo run -p xtask <SUBCOMMAND>
SUBCOMMANDS:
    test [all|python|kotlin|swift]
"
    )
}
