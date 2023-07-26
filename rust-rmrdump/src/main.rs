use std::net::Ipv4Addr;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Default, Clone)]
struct RouteEntry {
    final_: bool,
    prefix_length: u8,
    output_index: u32,
    children: Option<RouteEntryTable>,
}

#[derive(Debug, Default, Clone)]
struct RouteEntryTable(Vec<RouteEntry>);

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
        let mut level0_table = RouteEntryTable(entries);

        Self {
            level0_table,
            route_entries_allocated: 0,
        }
    }

    pub fn add(&mut self, prefix: &str, length: u8, destination_idx: u32) {
        let prefix_octets = prefix.parse::<Ipv4Addr>().unwrap();
        let prefix_octets = prefix_octets.octets();

        let mut table = &mut self.level0_table;
        let added = Self::add_in_table(&mut table, prefix_octets, length, destination_idx, 0);
    }

    fn add_in_table(
        table: &mut RouteEntryTable,
        prefix_octets: [u8; 4],
        length: u8,
        destination_idx: u32,
        level: usize,
    ) {
        let table_size_prefix = &Self::TABLE_SIZES[level];
        let table_size = table_size_prefix.0;
        let prefix_length = table_size_prefix.1;

        let (index, span) = Self::get_index_span_from_prefix_length(
            prefix_octets,
            length,
            level.try_into().unwrap(),
        );

        eprintln!("index: {index}, span: {span}");
        let next_size = if level < 3 {
            Self::TABLE_SIZES[level + 1].0
        } else {
            0
        };

        let mut i = 0;
        loop {
            let mut entry = &mut table.0[index as usize + i as usize];
            if length <= prefix_length {
                eprintln!("length: {length}, prefix_length: {prefix_length}, index: {index}, i: {i}, level: {level}");
                entry.final_ = true;
                entry.prefix_length = length;
                entry.output_index = destination_idx;
            } else {
                if entry.children.is_some() {
                    let mut inner_table = entry.children.as_mut().unwrap();
                    Self::add_in_table(
                        &mut inner_table,
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
                    entry.children = Some(inner_table);
                }
            }
            i += 1;
            if i >= span {
                break;
            }
        }
    }

    fn get_index_span_from_prefix_length(
        prefix_octets: [u8; 4],
        prefix_length: u8,
        level: usize,
    ) -> (usize, u16) {
        let levels: [u8; 4] = [16, 24, 28, 32];
        let level_edges: [(usize, usize); 4] = [(0, 2), (2, 3), (3, 4), (3, 4)];

        let (begin, end) = level_edges[level as usize];
        let level_offset = levels[level as usize];

        let span = if prefix_length > level_offset {
            1
        } else {
            1 << (level_offset - prefix_length)
        };

        let mut index = 0;
        for (i, value) in prefix_octets[begin..end].iter().rev().enumerate() {
            index = index + (1 << (8 * i)) * *value as usize;
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

    route_table.add("12.0.2.0", 23, 2002);

    eprintln!("{:#?}", route_table);
}
