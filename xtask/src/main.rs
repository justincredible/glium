use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("test") => test()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
test        run glium tests
"
    )
}

fn test() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = Command::new(cargo.clone())
        .current_dir("../glium")
        .args(&["test", "--test"])
        .output()?;

    for test in String::from_utf8(output.stderr)?
        .lines()
        .skip(2)
        .filter(|&s| s != "")
        .map(|s| {
            let target = s.trim_start().to_owned();
            println!("running: {}", target);
            let cargo = cargo.clone();
            std::thread::spawn(move || test_target(cargo, target))
        })
        .collect::<Vec<_>>() {

        test.join().unwrap_or_default();
    }

    if output.status.success() {
        Err("invalid arguments succeeded")?;
    }

    Ok(())
}

fn test_target(cargo: String, target: String) {
    let file = format!("glium/tests/{}.rs", target);
    let _ = File::open(file)
        .map(|file| {
            let mut lines = BufReader::new(file)
                 .lines()
                 .flatten();

            while let Some(line) = lines.next() {
                if line.starts_with("#[test]") {
                    let mut next_line = lines.next().unwrap();
                    while !next_line.starts_with("fn ") && !next_line.contains(" fn ") {
                        next_line = lines.next().unwrap();
                    }

                    // assuming "^fn . . . $"
                    let end = next_line.find(|c| c == '(' || c == '<').unwrap();
                    let test = &next_line[3..end];

                    let _ = Command::new(&cargo)
                        .current_dir("../glium")
                        .args(&["test", "--test", &target, "--", test])
                        .output()
                        .map_or((), |output| {
                            println!("{}", String::from_utf8(output.stdout).unwrap_or_default());
                            eprintln!("{}", String::from_utf8(output.stderr).unwrap_or_default());
                        });
                }
            }
        });

    println!("completed: {}", target);
}
