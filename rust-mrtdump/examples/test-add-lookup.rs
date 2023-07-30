use rust_mrtdump::RouteTable;

fn main() {
    let mut route_table = RouteTable::new();

    route_table.add("12.0.16.0", 20, 2004);
    route_table.add("12.0.2.0", 23, 2002);
    route_table.add("12.0.2.0", 24, 2003);
    route_table.add("12.0.1.0", 24, 2001);
    route_table.add("11.0.1.0", 24, 2005);
    route_table.add("11.0.1.16", 28, 2006);
    route_table.add("11.0.1.8", 29, 2007);

    eprintln!("{:#?}", route_table);

    println!("lookup('12.0.1.22'): {:?}", route_table.lookup("12.0.1.22"));
    println!("lookup('12.0.2.22'): {:?}", route_table.lookup("12.0.2.22"));
    println!("lookup('12.0.3.22'): {:?}", route_table.lookup("12.0.3.22"));
    println!("lookup('12.0.8.22'): {:?}", route_table.lookup("12.0.8.22"));
    println!(
        "lookup('12.0.17.22'): {:?}",
        route_table.lookup("12.0.17.22")
    );
    println!("lookup('11.0.1.32'): {:?}", route_table.lookup("11.0.1.32"));
    println!("lookup('11.0.1.24'): {:?}", route_table.lookup("11.0.1.24"));
    println!("lookup('11.0.1.15'): {:?}", route_table.lookup("11.0.1.15"));

    route_table.delete("11.0.1.16", 28);
    println!("After deleting 11.0.1.16/28");
    println!("lookup('11.0.1.24'): {:?}", route_table.lookup("11.0.1.24"));
    println!("lookup('11.0.1.15'): {:?}", route_table.lookup("11.0.1.15"));

    route_table.add("11.0.1.16", 28, 2006);
    println!("After Adding 11.0.1.16/28 again");
    println!("lookup('11.0.1.24'): {:?}", route_table.lookup("11.0.1.24"));
    println!("lookup('11.0.1.15'): {:?}", route_table.lookup("11.0.1.15"));

    route_table.delete("11.0.1.0", 24);
    println!("After deleting 11.0.1.0/24");
    println!("lookup('11.0.1.32'): {:?}", route_table.lookup("11.0.1.32"));
    println!("lookup('11.0.1.24'): {:?}", route_table.lookup("11.0.1.24"));
    println!("lookup('11.0.1.15'): {:?}", route_table.lookup("11.0.1.15"));

    route_table.add("11.0.1.0", 24, 2005);
    route_table.delete("11.0.1.16", 28);
    route_table.delete("11.0.1.8", 29);
    println!("After Adding 11.0.1.0/24 again and deleting 11.0.1.16/28, 11.0.1.8/29.");
    println!("lookup('11.0.1.32'): {:?}", route_table.lookup("11.0.1.32"));
    println!("lookup('11.0.1.24'): {:?}", route_table.lookup("11.0.1.24"));
    println!("lookup('11.0.1.15'): {:?}", route_table.lookup("11.0.1.15"));

    route_table.delete("11.0.1.0", 24);
    route_table.delete("12.0.16.0", 20);
    route_table.delete("12.0.2.0", 23);
    route_table.delete("12.0.2.0", 24);
    route_table.delete("12.0.1.0", 24);

    eprintln!("{:#?}", route_table);

    drop(route_table);
    let mut route_table = RouteTable::new();

    route_table.add("12.0.16.0", 20, 2004);
    eprintln!("{:#?}", route_table);

    route_table.delete("12.0.16.0", 20);
    eprintln!("{:#?}", route_table);

    route_table.delete("12.0.16.0", 20);
    eprintln!("{:#?}", route_table);
}
