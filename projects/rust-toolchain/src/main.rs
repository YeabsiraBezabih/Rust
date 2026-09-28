fn main() {
    let service_name = "AuthWorker";
    let version = "1.0.0";

    println!("========================================");
    println!("  Starting Service: {} (v{})", service_name, version);
    println!("========================================");

    // "cfg!(debug_assertions)" is a built-in macro that evaluates to `true`
    // when compiled in debug mode, and `false` when compiled with `--release`.
    if cfg!(debug_assertions) {
        println!("Build Profile : DEBUG");
        println!("Optimizations : Disabled (Fast compilation, debug symbols present)");
        println!("Warning       : Do not benchmark performance in this mode!");
    } else {
        println!("Build Profile : RELEASE");
        println!("Optimizations : Maximum (High-performance production binary)");
    }

    println!("----------------------------------------");
    println!("System ready to accept network requests.");
}
