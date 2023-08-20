use std::fs;
const RESOLVE_CONF: &str = "/etc/resolv.conf";
pub fn print_resolve_conf() {
    //let mut file = File::open(file_name)?;
    let contents = fs::read_to_string(RESOLVE_CONF).expect("Error reading {RESOLVE_CONF} file");
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    contents
        .lines()
        .map(|line| {
            if line.starts_with("nameserver") {
                println!("Debug: {RESOLVE_CONF} nameserver {}", line);
            }
        })
        .collect()

    //println!("{:#?}", contents);
}
