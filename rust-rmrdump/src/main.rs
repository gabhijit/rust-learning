use std::net::Ipv4Addr;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Default, Clone)]
struct RouteEntry {
    r#final: bool,
    index: usize,
    prefix_length: u8,
    output_index: u32,
    children: Option<RouteEntryTable>,
}

#[derive(Default, Clone)]
struct RouteEntryTable(Vec<RouteEntry>);

impl std::fmt::Debug for RouteEntryTable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "RouteEntryTable: ")?;
        fmt.debug_list()
            .entries(self.0.iter().filter(|e| e.r#final || e.children.is_some()))
            .finish()
    }
}

#[pyclass]
#[derive(Debug, Default)]
pub struct RouteTable {
    level0_table: RouteEntryTable,
    route_entries_allocated: u32,
}

struct TableSize(u32, u8);

impl RouteTable {
    const TABLE_SIZES: [TableSize; 4] = [
        TableSize(65536, 16),
        TableSize(256, 24),
        TableSize(16, 28),
        TableSize(16, 32),
    ];

    pub fn new() -> Self {
        let entries = vec![RouteEntry::default(); 65536];
        let level0_table = RouteEntryTable(entries);
        let route_entries_allocated = 65536;

        Self {
            level0_table,
            route_entries_allocated,
        }
    }

    pub fn lookup(&self, ip_address: &str) -> Option<u32> {
        let ip_address = ip_address.parse::<Ipv4Addr>().unwrap();
        let octets = ip_address.octets();

        let mut table = &self.level0_table;
        for (i, _) in Self::TABLE_SIZES.iter().enumerate() {
            let (index, _) = Self::get_index_span_from_prefix_length(octets, 32, i);
            let entry = &table.0[index];
            if entry.r#final {
                return Some(entry.output_index);
            } else if let Some(next_table) = &entry.children {
                table = next_table
            } else {
                return None;
            }
        }
        None
    }

    pub fn add(&mut self, prefix: &str, length: u8, destination_idx: u32) {
        let prefix_octets = prefix.parse::<Ipv4Addr>().unwrap();
        let prefix_octets = prefix_octets.octets();

        let table = &mut self.level0_table;
        self.route_entries_allocated +=
            Self::add_in_table(table, prefix_octets, length, destination_idx, 0);
    }

    fn add_in_table(
        table: &mut RouteEntryTable,
        prefix_octets: [u8; 4],
        length: u8,
        destination_idx: u32,
        level: usize,
    ) -> u32 {
        let table_size_prefix = &Self::TABLE_SIZES[level];
        let prefix_length = table_size_prefix.1;

        let (index, span) = Self::get_index_span_from_prefix_length(prefix_octets, length, level);

        eprintln!("index: {index}, span: {span}");
        let next_size = if level < 3 {
            Self::TABLE_SIZES[level + 1].0
        } else {
            0
        };

        let mut i = 0;
        let mut entries_allocated = 0;
        loop {
            let mut entry = &mut table.0[index + i as usize];
            if length <= prefix_length {
                eprintln!("length: {length}, prefix_length: {prefix_length}, index: {index}, i: {i}, level: {level}");
                entry.index = index + i as usize;
                if entry.r#final {
                    if length > entry.prefix_length {
                        entry.prefix_length = length;
                        entry.output_index = destination_idx;
                    }
                } else {
                    entry.r#final = true;
                    entry.prefix_length = length;
                    entry.output_index = destination_idx;
                }
            } else if entry.children.is_some() {
                let inner_table = entry.children.as_mut().unwrap();
                Self::add_in_table(
                    inner_table,
                    prefix_octets,
                    length,
                    destination_idx,
                    level + 1,
                );
            } else {
                let mut inner_table =
                    RouteEntryTable(vec![RouteEntry::default(); next_size as usize]);

                Self::add_in_table(
                    &mut inner_table,
                    prefix_octets,
                    length,
                    destination_idx,
                    level + 1,
                );
                entry.index = index + i as usize;
                entry.children = Some(inner_table);
                entries_allocated += next_size;
            }
            i += 1;
            if i >= span {
                break;
            }
        }
        entries_allocated
    }

    fn get_index_span_from_prefix_length(
        prefix_octets: [u8; 4],
        prefix_length: u8,
        level: usize,
    ) -> (usize, u16) {
        let levels: [u8; 4] = [16, 24, 28, 32];
        let level_edges: [(usize, usize); 4] = [(0, 2), (2, 3), (3, 4), (3, 4)];

        let (begin, end) = level_edges[level];
        let level_offset = levels[level];

        let span = if prefix_length > level_offset {
            1
        } else {
            1 << (level_offset - prefix_length)
        };

        let mut index = 0;
        for (i, value) in prefix_octets[begin..end].iter().rev().enumerate() {
            index += (1 << (8 * i)) * *value as usize;
        }

        index = match level {
            2 => index >> 4,
            3 => index & 0x0F,
            _ => index,
        };

        (index, span)
    }
}

fn main() {
    let mut route_table = RouteTable::new();

    route_table.add("12.0.16.0", 20, 2004);
    route_table.add("12.0.2.0", 23, 2002);
    route_table.add("12.0.2.0", 24, 2003);
    route_table.add("12.0.1.0", 24, 2001);

    eprintln!("{:#?}", route_table);

    println!("lookup('12.0.1.22'): {:?}", route_table.lookup("12.0.1.22"));
    println!("lookup('12.0.2.22'): {:?}", route_table.lookup("12.0.2.22"));
    println!("lookup('12.0.3.22'): {:?}", route_table.lookup("12.0.3.22"));
}
