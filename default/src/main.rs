use tokio::time::Instant;

#[derive(Debug, Clone)]
struct Network {
    sequence: u32,
    timestamp: u64,
    //name: String,
    incoming_data_rate: f64,
    outgoing_data_rate: f64,
    //#[serde(skip_serializing)]
    timer: Instant,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            timer: Instant::now(),
            //sequence: 42 as u32,
            ..Default::default()
        }
    }
}

fn main() {
    let _n = Network::default();
    print!("{}", std::mem::size_of::<Network>())
}
