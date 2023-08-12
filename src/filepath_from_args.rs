pub fn filepath_from_args() -> String {
    let mut args = std::env::args();
    let this = args.next().unwrap();

    let mut args = args.collect::<Vec<_>>();
    match (args.len(), args.pop()) {
        (1, Some(filename)) => filename,
        (_, _) => {
            eprintln!("Usage: {this} <filename>");
            std::process::exit(1);
        }
    }
}
