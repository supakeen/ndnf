fn make_cli() -> clap::Command<'static> {
    clap::command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(make_cli_install())
        .subcommand(make_cli_upgrade())
        .subcommand(make_cli_remove())
        .subcommand(make_cli_reinstall())
}

fn make_cli_upgrade() -> clap::Command<'static> {
    clap::command!("upgrade")
        .about("Upgrade a package or packages on your system")
        .arg(
            clap::arg!(-q --quiet "Quiet operation (less output)")
                .required(false)
                .conflicts_with("verbose"),
        )
        .arg(
            clap::arg!(-v --verbose "Verbose operation (more output)")
                .required(false)
                .conflicts_with("quiet"),
        )
        .arg(
            clap::arg!(-y --assumeyes "Automatically answer yes for all questions")
                .required(false)
                .conflicts_with("assumeno"),
        )
        .arg(
            clap::arg!(-n --assumeno "Automatically answer no for all questions")
                .required(false)
                .conflicts_with("assumeyes"),
        )
        .arg(clap::arg!(<package> "Package(s) to upgrade").multiple(true))
}

fn make_cli_install() -> clap::Command<'static> {
    clap::command!("install")
        .about("Install a package or packages on your system")
        .arg(
            clap::arg!(-q --quiet "Quiet operation (less output)")
                .required(false)
                .conflicts_with("verbose"),
        )
        .arg(
            clap::arg!(-v --verbose "Verbose operation (more output)")
                .required(false)
                .conflicts_with("quiet"),
        )
        .arg(
            clap::arg!(-y --assumeyes "Automatically answer yes for all questions")
                .required(false)
                .conflicts_with("assumeno"),
        )
        .arg(
            clap::arg!(-n --assumeno "Automatically answer no for all questions")
                .required(false)
                .conflicts_with("assumeyes"),
        )
        .arg(clap::arg!(--nodocs "Do not install package documentation").required(false))
        .arg(
            clap::arg!(-r --refresh "Set metadata as expired before running the command")
                .required(false),
        )
        .arg(clap::arg!(<package> "Package(s) to install").multiple(true))
}

fn make_cli_reinstall() -> clap::Command<'static> {
    clap::command!("reinstall")
        .about("Reinstall a package or packages on your system")
        .arg(
            clap::arg!(-q --quiet "Quiet operation (less output)")
                .required(false)
                .conflicts_with("verbose"),
        )
        .arg(
            clap::arg!(-v --verbose "Verbose operation (more output)")
                .required(false)
                .conflicts_with("quiet"),
        )
        .arg(
            clap::arg!(-y --assumeyes "Automatically answer yes for all questions")
                .required(false)
                .conflicts_with("assumeno"),
        )
        .arg(
            clap::arg!(-n --assumeno "Automatically answer no for all questions")
                .required(false)
                .conflicts_with("assumeyes"),
        )
        .arg(clap::arg!(<package> "Package(s) to reinstall").multiple(true))
}

fn make_cli_remove() -> clap::Command<'static> {
    clap::command!("remove")
        .about("Remove a package or packages from your system")
        .arg(
            clap::arg!(-q --quiet "Quiet operation (less output)")
                .required(false)
                .conflicts_with("verbose"),
        )
        .arg(
            clap::arg!(-v --verbose "Verbose operation (more output)")
                .required(false)
                .conflicts_with("quiet"),
        )
        .arg(
            clap::arg!(-y --assumeyes "Automatically answer yes for all questions")
                .required(false)
                .conflicts_with("assumeno"),
        )
        .arg(
            clap::arg!(-n --assumeno "Automatically answer no for all questions")
                .required(false)
                .conflicts_with("assumeyes"),
        )
        .arg(clap::arg!(<package> "Package(s) to install").multiple(true))
}

fn main() {
    let _matches = make_cli().get_matches();

    println!("Hello, world!");
}
