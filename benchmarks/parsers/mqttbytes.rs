use bytes::{Buf, BytesMut};
use mqttbytes::v4::{mqtt_read, Publish};
use mqttbytes::QoS;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod common;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    pretty_env_logger::init();
    let count = 1024 * 1024;
    let payload_size = 1024;
    let data = generate_data(count, payload_size);
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    // ------------------------- write throughput -------------------------------
    let start = Instant::now();
    let mut output = BytesMut::new();
    for publish in data.into_iter() {
        publish.write(&mut output).unwrap();
    }

    let elapsed_micros = start.elapsed().as_micros();
    let total_size = output.len();
    let throughput = (total_size * 1000_000) / elapsed_micros as usize;
    let write_throughput = throughput as f32 / 1024.0 / 1024.0 / 1024.0;
    let total_size_gb = total_size as f32 / 1024.0 / 1024.0 / 1024.0;

    // --------------------------- read throughput -------------------------------

    let start = Instant::now();
    let mut packets = Vec::with_capacity(count);
    while output.has_remaining() {
        let packet = mqtt_read(&mut output, 10 * 1024).unwrap();
        packets.push(packet);
    }

    let elapsed_micros = start.elapsed().as_micros();
    let throughput = (total_size * 1000_000) / elapsed_micros as usize;
    let read_throughput = throughput as f32 / 1024.0 / 1024.0 / 1024.0;

    // --------------------------- results ---------------------------------------

    // println!(
    //     "
    //     Id = mqttbytes,
    //     Messages = {},
    //     Payload (bytes) = {},
    //     Total size (GB) = {},
    //     Write Throughput (GB/sec) = {}
    //     Read Throughput (GB/sec) = {}",
    //     count, payload_size, total_size_gb, write_throughput, read_throughput
    // );
    let print = Print {
        id: "mqttbytesparser".to_owned(),
        messages: count,
        payload_size,
        total_size_gb,
        write_throughput_gpbs: write_throughput,
        read_throughput_gpbs: read_throughput,
    };

    println!("{}", serde_json::to_string_pretty(&print).unwrap());
    common::profile("bench.pb", guard);
}

fn generate_data(count: usize, payload_size: usize) -> Vec<Publish> {
    let mut data = Vec::with_capacity(count);
    for i in 0..count {
        let mut publish = Publish::new("hello/world", QoS::AtLeastOnce, vec![1; payload_size]);
        publish.pkid = (i % 100 + 1) as u16;
        data.push(publish);
    }

    data
}

#[derive(Serialize, Deserialize)]
struct Print {
    id: String,
    messages: usize,
    payload_size: usize,
    total_size_gb: f32,
    write_throughput_gpbs: f32,
    read_throughput_gpbs: f32,
}
