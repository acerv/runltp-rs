mod ltp;

use clap::{load_yaml, App};
use ltp::session::Session;

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from(yaml).get_matches();

    // process run subcommand
    if let Some(ref m) = matches.subcommand_matches("run") {
        let session;

        if m.is_present("cmdfiles") {
            let cmdfiles = m.value_of("cmdfiles").unwrap();
            let suites: Vec<&str> = cmdfiles.split(",").collect();
            session = Session::from(suites);
        } else if m.is_present("network") {
            session = Session::from_scenario("network");
        } else {
            session = Session::from_scenario("default");
        }

        session.run();
    }

    // process list subcommand
    if let Some(ref m) = matches.subcommand_matches("list") {
        if m.is_present("scenario") {
            let net_session;
            let def_session;

            net_session = Session::from_scenario("network");
            def_session = Session::from_scenario("default");

            println!("Default suites:");
            for item in def_session.suites {
                println!("\t{}", item.name);
            }
            println!();

            println!("Network suites:");
            for item in net_session.suites {
                println!("\t{}", item.name);
            }
            println!();
        } else {
            let session = Session::all();

            println!("Available suites:");
            for item in session.suites {
                println!("\t{}", item.name);
            }
            println!();
        }
    }

    // process install subcommand
    if let Some(ref m) = matches.subcommand_matches("install") {
        if let Some(ref repo) = m.value_of("gitrepo") {
            let repo_dir: &str;
            let install_dir: &str;

            if let Some(ref dir) = m.value_of("repodir") {
                repo_dir = dir;
            } else {
                repo_dir = "ltp";
            }

            if let Some(ref dir) = m.value_of("installdir") {
                install_dir = dir;
            } else {
                install_dir = "/opt/ltp";
            }

            ltp::install::install_ltp(repo, repo_dir, install_dir);
        }
    }
}
