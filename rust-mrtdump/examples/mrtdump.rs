use bgpkit_parser::{BgpElem, BgpkitParser};

fn main() {
    let parser = BgpkitParser::new("rib.20230626.0400.bz2").unwrap();

    for element in parser.into_iter().filter(|e| e.origin.is_some()) {
        eprintln!("prefix: {:#?}", element.prefix.prefix);
    }
}
