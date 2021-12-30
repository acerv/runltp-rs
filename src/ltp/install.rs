use git2::Repository;
use os_info;
use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::result;

/// Check the current Linux distro and return its name.
fn os_type() -> os_info::Type {
    os_info::get().os_type()
}

/// Run a generic command inside the OS.
fn run_command(cmd: &str, args: Vec<&str>, path: &str) -> result::Result<(), Error> {
    let curr_dir: String;

    if path.is_empty() {
        curr_dir = String::from(env::current_dir().unwrap().to_str().unwrap());
    } else {
        curr_dir = String::from(path);
    }

    let stdout = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .current_dir(curr_dir)
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}

/// Update the packages database according with Linux distro.
fn update_pkg_db() {
    let command: &str;
    let args: Vec<&str>;

    match os_type() {
        os_info::Type::Debian | os_info::Type::Ubuntu => {
            command = "apt-get";
            args = Vec::from(["update"]);
        }
        os_info::Type::Fedora => {
            command = "yum";
            args = Vec::from(["update", "-y"]);
        }
        os_info::Type::Alpine => {
            command = "apk";
            args = Vec::from(["update"]);
        }
        _ => {
            command = "zypper";
            args = Vec::from(["--non-interactive", "refresh"]);
        }
    }

    match run_command(command, args, "") {
        Ok(_) => (),
        Err(e) => panic!("error executing {}: {}", command, e),
    }
}

/// Install dependences according with the Linux distro.
fn install_pkg() {
    let command: &str;
    let mut args: Vec<&str>;

    let mut default = HashMap::from([
        ("git", "git"),
        ("unzip", "unzip"),
        ("autoconf", "autoconf"),
        ("automake", "automake"),
        ("make", "make"),
        ("gcc", "gcc"),
        ("bc", "bc"),
        ("dosfstools", "dosfstools"),
        ("xfstools", "xfstools"),
        ("e2fsprogs", "e2fsprogs"),
        ("btrfsprogs", "btrfsprogs"),
        ("quota", "quota"),
        ("nfs-utils", "nfs-utils"),
        ("kernel-devel", "kernel-devel"),
        ("libaio-devel", "libaio-devel"),
        ("libacl-devel", "libacl-devel"),
        ("libattr-devel", "libattr-devel"),
        ("libcap-devel", "libcap-devel"),
        ("libnuma-devel", "libnuma-devel"),
    ]);

    let alpine = HashMap::from([
        ("btrfsprogs", "btrfs-progs"),
        ("quota", "quota-tools"),
        ("kernel-devel", "linux-headers"),
        ("libaio-devel", "libaio-dev"),
        ("libacl-devel", "acl-dev"),
        ("libattr-devel", "attr-dev"),
        ("libcap-devel", "libcap-dev"),
        ("libnuma-devel", "numactl-dev"),
    ]);

    let debian = HashMap::from([
        ("btrfsprogs", "btrfs-progs"),
        ("nfs-utils", "nfs-kernel-server"),
        ("kernel-devel", "linux-headers-$ARCH$"),
        ("libaio-devel", "libaio-dev"),
        ("libacl-devel", "libacl1-dev"),
        ("libattr-devel", "libattr1-dev"),
        ("libcap-devel", "libcap-dev"),
        ("libnuma-devel", "libnuma-dev"),
    ]);

    let fedora = HashMap::from([
        ("btrfsprogs", "btrfs-progs"),
        ("libaio-devel", "libaio-devel"),
        ("libacl-devel", "libacl-devel"),
        ("libattr-devel", "libattr-devel"),
        ("libcap-devel", "libcap-devel"),
        ("libnuma-devel", "libnuma-devel"),
    ]);

    match os_type() {
        os_info::Type::Debian | os_info::Type::Ubuntu => {
            command = "apt-get";
            args = Vec::from(["-y", "install"]);
            default.extend(debian);
        }
        os_info::Type::Fedora => {
            command = "yum";
            args = Vec::from(["install", "-y"]);
            default.extend(fedora);
        }
        os_info::Type::Alpine => {
            command = "apk";
            args = Vec::from(["add"]);
            default.extend(alpine);
        }
        _ => {
            command = "zypper";
            args = Vec::from(["--non-interactive", "--ignore-unknown", "in"]);
        }
    }

    args.extend(default.into_values());
    match run_command(command, args, "") {
        Ok(_) => (),
        Err(e) => panic!("error executing {}: {}", command, e),
    }
}

/// Clone LTP repo into a path from GIT url.
fn clone_from_url(url: &str, path: &str) -> String {
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let path = repo.workdir().unwrap();
    path.to_str().unwrap().to_string()
}

/// Compile the LTP suite.
fn compile_ltp(repo_dir: &str, install_dir: &str) {
    let prefix: String;

    if install_dir.is_empty() {
        prefix = String::new();
    } else {
        prefix = format!("--prefix={}", install_dir);
    }

    match run_command("make", Vec::from(["autotools"]), repo_dir) {
        Ok(_) => (),
        Err(e) => panic!("error executing make: {}", e),
    }

    match run_command("./configure", Vec::from([prefix.as_str()]), repo_dir) {
        Ok(_) => (),
        Err(e) => panic!("error executing ./configure: {}", e),
    }

    match run_command("make", Vec::new(), repo_dir) {
        Ok(_) => (),
        Err(e) => panic!("error executing make: {}", e),
    }

    match run_command("make", Vec::from(["install"]), repo_dir) {
        Ok(_) => (),
        Err(e) => panic!("error executing make: {}", e),
    }
}

/// Install LTP from a GIT repo.
pub fn install_ltp(url: &str, repo_dir: &str, install_dir: &str) {
    update_pkg_db();
    install_pkg();

    let workdir = clone_from_url(url, repo_dir);
    compile_ltp(&workdir, install_dir);
}
