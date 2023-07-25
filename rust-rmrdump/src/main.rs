use pyo3::prelude::*;
use std::net::Ipv4Addr;

#[pyclass]
#[derive(Debug, Default)]
struct RouteEntry {
    final_: bool,
    prefix_length: u8,
    output_index: u32,
    children: Option<RouteTable>,
}

#[pyclass]
#[derive(Debug, Default)]
pub struct RouteTable {
    level0_table: Vec<RouteEntry>,
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
        let level0_table = Vec::with_capacity(65536);

        Self {
            level0_table,
            route_entries_allocated: 0,
        }
    }

    pub fn add(&mut self, prefix: &str, length: u8, destination_idx: u32) {
        let prefix_octets = prefix.parse::<Ipv4Addr>().unwrap();

        let mut table = &self.level0_table;

        let mut level = 0;
        while level < Self::TABLE_SIZES.len() {
            let table_size_prefix = &Self::TABLE_SIZES[level];
            let table_size = table_size_prefix.0;
            let prefix_length = table_size_prefix.1;
        }
    }

    fn get_index_span_from_prefix_length(
        prefix_octets: [u8; 4],
        length: u8,
        level: u8,
    ) -> (u16, u16) {
        todo!();
    }
}

fn main() {
    println!("Hello, world!");
}
