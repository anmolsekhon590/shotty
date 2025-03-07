use criterion::{Criterion, black_box, criterion_group, criterion_main};
use shotty::load_config;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn benchmark_screenshot(c: &mut Criterion) {
    let config = load_config();

    let home_dir = env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);

    // Ensure the screenshot directory exists
    if !Path::new(&screenshot_dir).exists() {
        fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");
    }

    let filename = format!("{}/test_screenshot.png", screenshot_dir);
    let selection = "0,0 1920x1080"; // Mocked selection to avoid user input

    let mut group = c.benchmark_group("Shotty Performance");
    group.sample_size(10); // Reduce sample size to avoid unnecessary repetitions

    group.bench_function("create_dir_all", |b| {
        b.iter(|| fs::create_dir_all(black_box(&screenshot_dir)).unwrap())
    });

    group.bench_function("mock_slurp_selection", |b| {
        b.iter(|| {
            // Mocking `slurp` by directly passing a known region
            let _ = Command::new("echo")
                .arg(selection)
                .output()
                .expect("Failed to mock slurp");
        })
    });

    group.bench_function("grim_screenshot", |b| {
        b.iter(|| {
            let _ = Command::new("grim")
                .args(["-g", selection, "/dev/null"]) // Fake output to avoid disk IO
                .status()
                .expect("Failed to execute grim");
        })
    });

    group.bench_function("wl-copy", |b| {
        b.iter(|| {
            let _ = Command::new("sh")
                .arg("-c")
                .arg("echo 'test image data' | wl-copy --type=image/png") // Use dummy data
                .status()
                .expect("Failed to execute wl-copy");
        })
    });

    group.finish();

    // Remove the test screenshot file to keep the directory clean
    if Path::new(&filename).exists() {
        fs::remove_file(&filename).expect("Failed to delete test screenshot");
    }
}

criterion_group!(benches, benchmark_screenshot);
criterion_main!(benches);
