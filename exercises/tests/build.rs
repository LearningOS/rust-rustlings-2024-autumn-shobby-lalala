use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 针对tests8.rs的条件设置
    if env::var("CARGO_FEATURE_PASS").is_ok() {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
