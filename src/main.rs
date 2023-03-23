use clap::Parser;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

fn start_dialog(cwd: &str, file_count: usize) {
    println!("BEWARE! Starting from current directory ({})", cwd);
    println!(
        "{} files and directories with spaces will be renamed automatically.",
        file_count
    );
}

fn user_confirm() -> io::Result<bool> {
    println!("Press \"ENTER\" to continue or \"N\" to exit:");

    let mut ops = String::new();

    io::stdin().read_line(&mut ops)?;

    Ok(!ops.trim().eq_ignore_ascii_case("N"))
}

fn recurse_dirs(dir: &PathBuf, vec: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recurse_dirs(&path, vec)?;
        }
        if path.to_str().unwrap().contains(' ') {
            vec.push(entry.path());
        }
    }
    Ok(())
}

#[derive(Parser, Default, Debug)]
#[clap(author = "HTunne", version, about)]
/// Recursively replace spaces with underscores in file and directory names.
/// Optionally supply a different 'from' char and 'to' char.
struct Arguments {
    /// Path to begin renaming
    path: PathBuf,
    #[clap(default_value = " ", short, long)]
    /// From char
    from_char: String,
    #[clap(default_value = "_", short, long)]
    /// To char
    to_char: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    println!("{:?}", args);

    let mut vec: Vec<PathBuf> = Vec::new();
    recurse_dirs(&args.path, &mut vec)?;

    // TODO: dialog for 0 files
    start_dialog(args.path.to_str().unwrap(), vec.len());

    loop {
        match user_confirm() {
            Ok(result) if result => break,
            Ok(result) if !result => return Ok(()),
            Ok(_) => return Ok(()),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    let mut i = 1;
    println!("Beginning...");
    for from_path in vec {
        let mut to_path = from_path.clone();
        let new_name = to_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&args.from_char, &args.to_char);
        to_path.pop();
        to_path.push(new_name);
        println!(
            "{} Renaming: {} -> {}",
            i,
            from_path.to_str().unwrap(),
            to_path.to_str().unwrap()
        );
        fs::rename(from_path, to_path)?;
        i += 1;
    }

    Ok(())
}
