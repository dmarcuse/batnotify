# batnotify

Notify you about low battery levels

## Installation

batnotify supports Linux, and theoretically, Mac OS. On Linux, ensure the [requirements for the dbus dependency](https://github.com/diwic/dbus-rs#requirements) are met before compiling the project.

To install the [latest version published on crates.io](https://crates.io/crates/batnotify):

`cargo install batnotify`

To install from the latest commit on the master branch:

`cargo install --git https://github.com/dmarcuse/batnotify.git`

To update batnotify after installing, re-run the same command you used to install with the `--force` flag.

## Usage

Check battery levels every 60 seconds, and notify at 25% and 10%:

`batnotify --critical 10 --low 25 --interval 60`
