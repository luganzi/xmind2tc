use xmind2tc::xmind;

fn main() {
    let mut args = std::env::args();
    let script_name = args.next().unwrap();

    match args.next() {
        Some(ref filename) => {
            if let Err(e) = xmind::parse_xmind_to_xls(filename) {
                println!("{}", e);
            }
        }
        None => println!("Usage: {} <filename>", script_name),
    }
}
