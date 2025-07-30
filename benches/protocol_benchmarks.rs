// Performance benchmarks for RustSat-ESA protocol stack
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};
use rustsat_esa::protocol::network::MeshNetwork;
use rustsat_esa::cubesat::CubeSatProtocol;
use rustsat_esa::security::CryptoModule;
use rustsat_esa::telemetry::TelemetryProcessor;

fn benchmark_spacecan_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpaceCAN Encoding");
    
    for size in [8, 64, 256, 1024].iter() {
        let data = vec![0u8; *size];
        let frame = SpaceCANFrame::new(0x123, data, FramePriority::High);
        
        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| black_box(frame.encode()))
        });
    }
    
    group.finish();
}

fn benchmark_spacecan_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpaceCAN Decoding");
    
    for size in [8, 64, 256, 1024].iter() {
        let data = vec![0u8; *size];
        let frame = SpaceCANFrame::new(0x123, data, FramePriority::High);
        let encoded = frame.encode();
        
        group.bench_with_input(BenchmarkId::new("decode", size), size, |b, _| {
            b.iter(|| black_box(SpaceCANFrame::decode(&encoded)))
        });
    }
    
    group.finish();
}

fn benchmark_mesh_routing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mesh Network Routing");
    
    let network = MeshNetwork::new();
    
    group.bench_function("network_initialization", |b| {
        b.iter(|| black_box(MeshNetwork::new()))
    });
    
    group.finish();
}

fn benchmark_encryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cryptographic Operations");
    let mut crypto = CryptoModule::new();
    
    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        
        group.bench_with_input(BenchmarkId::new("encrypt", size), size, |b, _| {
            b.iter(|| black_box(crypto.encrypt(&data)))
        });
        
        if let Ok(encrypted) = crypto.encrypt(&data) {
            group.bench_with_input(BenchmarkId::new("decrypt", size), size, |b, _| {
                b.iter(|| black_box(crypto.decrypt(&encrypted)))
            });
        }
    }
    
    group.finish();
}

fn benchmark_telemetry_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Telemetry Processing");
    let processor = TelemetryProcessor::new();
    
    group.bench_function("processor_initialization", |b| {
        b.iter(|| black_box(TelemetryProcessor::new()))
    });
    
    group.finish();
}

fn benchmark_cubesat_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("CubeSat Operations");
    
    group.bench_function("cubesat_initialization", |b| {
        b.iter(|| black_box(CubeSatProtocol::new(1, "Benchmark-Sat".to_string())))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_spacecan_encoding,
    benchmark_spacecan_decoding,
    benchmark_mesh_routing,
    benchmark_encryption,
    benchmark_telemetry_processing,
    benchmark_cubesat_operations
);

criterion_main!(benches);