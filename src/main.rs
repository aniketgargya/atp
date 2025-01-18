use std::process::{ Command, Stdio };
use std::path::Path;
use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(author = None, version = None, about = None, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    PullFiles {
        source_path: String,
        destination_path: String,
        device_name: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
    PullFilesAfterModDate {
        source_path: String,
        destination_path: String,
        mod_date: String,
        device_name: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
    PushFiles {
        source_path: String,
        destination_path: String,
        device_name: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
}

enum AdbError {
    SpawnError(std::io::Error),
    WaitError(std::io::Error),
    NonZeroExitCode(i32),
    NoExitCode,
}

impl std::fmt::Display for AdbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SpawnError(err) => write!(f, "Error spawning adb: {err}"),
            Self::WaitError(err) => write!(f, "Error waiting on adb process: {err}"),
            Self::NonZeroExitCode(code) => write!(f, "Error executing adb: Non zero exit code {}", code),
            Self::NoExitCode => write!(f, "Error executing adb: No exit code (because adb was likely terminated by a signal)"),
        }
    }
}

struct AdbSettings {
    verbose: bool,
    device_name: String,
}

fn pull_files(source_path: &String, destination_path: &String, settings: &AdbSettings) -> Result<(), AdbError> {
    execute_adb(&["pull", source_path, destination_path], settings)
}

fn push_files(source_path: &String, destination_path: &String, settings: &AdbSettings) -> Result<(), AdbError> {
    execute_adb(&["push", source_path, destination_path], settings)
}

fn pull_files_after_mod_date(source_path: &String, destination_path: &String, mod_date: &String, settings: &AdbSettings) -> Result<(), AdbError> {
    let files_after_mod_date: Vec<String> = query_files_after_mod_date(source_path, mod_date, settings)?;

    files_after_mod_date.into_iter().try_for_each(|file| -> Result<(), AdbError> {
        pull_files(
            &Path::new(&source_path).join(&file).to_string_lossy().to_string(),
            destination_path,
            &settings
        )
    })
}

fn query_files_after_mod_date(source_path: &String, mod_date: &String, settings: &AdbSettings) -> Result<Vec<String>, AdbError> {
    query_adb(&["shell", "find", source_path, "-type", "f", "-newermt", mod_date], settings)
        .map(|output|
            output
                .split_whitespace()
                .map(|file| file.to_string())
                .collect::<Vec<_>>()
        )
}

fn query_adb(args: &[&str], settings: &AdbSettings) -> Result<String, AdbError> {
    let stderr = if settings.verbose {
        Stdio::inherit()
    } else {
        Stdio::null()
    };

    let child = Command::new("adb")
        .arg("-s").arg(&settings.device_name)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(stderr)
        .spawn()
        .map_err(AdbError::SpawnError)?;

    let output = child.wait_with_output().map_err(AdbError::WaitError)?;

    match output.status.code() { 
        Some(0) => Ok(String::from_utf8(output.stdout).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).to_string())),
        Some(code) => Err(AdbError::NonZeroExitCode(code)),
        None => Err(AdbError::NoExitCode),
    }
}

fn execute_adb(args: &[&str], settings: &AdbSettings) -> Result<(), AdbError> {
    let (stdout, stderr) = if settings.verbose {
        (Stdio::inherit(), Stdio::inherit())
    } else {
        (Stdio::null(), Stdio::null())
    };

    let mut child = Command::new("adb")
        .arg("-s").arg(&settings.device_name)
        .args(args)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .map_err(AdbError::SpawnError)?;

    let exit_status = child.wait().map_err(AdbError::WaitError)?;

    match exit_status.code() {
        Some(0) => Ok(()),
        Some(code) => Err(AdbError::NonZeroExitCode(code)),
        None => Err(AdbError::NoExitCode),
    }
}

fn main() {
    let args = Args::parse();

    let result = match args.cmd {
        Commands::PullFiles { source_path, destination_path, device_name, verbose } =>
            pull_files(
                &source_path,
                &destination_path,
                &AdbSettings { device_name, verbose }
            ),
        Commands::PullFilesAfterModDate { source_path, destination_path, mod_date, device_name, verbose } =>
            pull_files_after_mod_date(
                &source_path,
                &destination_path,
                &mod_date,
                &AdbSettings { device_name, verbose }
            ),
        Commands::PushFiles { source_path, destination_path, device_name, verbose } =>
            push_files(
                &source_path,
                &destination_path,
                &AdbSettings { device_name, verbose }
            ),
    };

    match result {
        Ok(()) => println!("Done!"),
        Err(err) => println!("{}", err),
    };
}
