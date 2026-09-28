
// // 2. Bringing the public struct into scope with `use`
// mod telemetry;

// use telemetry::metrics::SystemMetric;

// fn main() {
//     // Constructing the metric using the public constructor
//     let cpu_metric = SystemMetric::new(String::from("cpu_load_percent"), 45.8);

//     println!("Metric Name : {}", cpu_metric.name);
//     println!("Metric Value: {}", cpu_metric.read_value());
//     println!("Full Report : {}", cpu_metric.format_with_version());

//     // UNCOMMENTING THE LINE BELOW CAUSES A COMPILE ERROR (field `value` is private):
//     // println!("Direct private access: {}", cpu_metric.value);
// }
