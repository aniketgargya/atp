use std::process::{ Command, Stdio };

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
    let result = pull_files(
        &String::from("/storage/emulated/0/DCIM/Camera/"),
        &String::from("/Users/aniketgargya/Documents/GitHub/android-file-fetch/test"),
        &AdbSettings {
            device_name: String::from("2B191JEG509242"),
            verbose: true,
        },
    );

    match result {
        Ok(()) => println!("Done!"),
        Err(err) => println!("{}", err),
    };
}
