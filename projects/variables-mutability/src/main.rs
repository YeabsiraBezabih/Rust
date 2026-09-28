// const MAX_ALLOWED_CPU_PERCENT: f64 = 100.0;

// fn main() {
//     let cpu_raw = " 88.5% ";
//     let cpu_raw = cpu_raw.trim();
//     let cpu_raw: f64 = cpu_raw.trim_matches('%').parse().unwrap();

//     if cpu_raw >= MAX_ALLOWED_CPU_PERCENT {
//         println!("CPU usage is within allowed range");
//     } else {
//         println!("CPU usage exceeds maximum allowed percent");
//     }
// }

// fn main() {
//     let db_config: (&str, u16, &str, u32) = ("db.internal", 5432, "postgres", 100);
//     let (host, port, username, password) = db_config;

//     println!("Host: {} | Port: {} | Username: {} | Password: {}", host, port, username, password);

// }
//
// copy semnatics only work on stack primitics or scalar datatypes and fixed arrays/tuples

fn main() {
    let packet_header: [u8; 6] = [0, 1, 2, 3, 4, 5];

    for element in packet_header {
        
    println!("{}", element);

    }
}
