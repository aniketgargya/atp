# Android Transfer Program

A small CLI application written in Rust to transfer files to and from Android devices because Android File Transfer sucks now (or at least last time I checked). At this point[^1], it's probably easier to maintain this project than to try to get Android File Transfer to work. Currently, the program serves as a thin wrapper around the adb push and pull commands, and has a more sophisticated pull command that filters out files that were not modified after a supplied date[^2].

## Commands

The following commands are available, and their respective use cases should be self explanatory:
- `push_files <SOURCE_PATH> <DESTINATION_PATH> <DEVICE_NAME>`
- `pull_files <SOURCE_PATH> <DESTINATION_PATH> <DEVICE_NAME>`
- `pull_files_after_mod_date <SOURCE_PATH> <DESTINATION_PATH> <MOD_DATE> <DEVICE_NAME>`

The `MOD_DATE` argument should be formatted as `YYYY-MM-DD`. It's also possible to supply a more precise time, but doing so is left as an exercise to the reader[^3].

The `DEVICE_NAME` argument should be the id of the target android device, which can be found by running `adb devices`.

The `--verbose` flag can be added to any command to view all the adb output — try this to diagnose any program failures.

## Dependencies:
- [adb](https://developer.android.com/tools/adb): Ensure that adb can be found via the `$PATH` variable.

[^1]: haha, get it?

[^2]: This was the original motivation behind this project, as the only way to do this was a long and ugly command that involved pipes and moderate bash scripting skills.

[^3]: See `newerXY`'s `t` in the [`find` man pages](https://linux.die.net/man/1/find)
