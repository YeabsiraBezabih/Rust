// const COLLECTOR_VERSION: &str = "2.4.0";

// // A public sub-module named `metrics`
// pub mod metrics {
//     // A public struct with private fields (Encapsulation)
//     #[derive(Debug)]
//     pub struct SystemMetric {
//         pub name: String,      // Public field: Read/Write from outside
//         value: f64,            // Private field: Can only be modified by methods in this module!
//     }

//     impl SystemMetric {
//         // Public constructor
//         pub fn new(name: String, value: f64) -> SystemMetric {
//             SystemMetric { name, value }
//         }

//         // Public getter method
//         pub fn read_value(&self) -> f64 {
//             self.value
//         }

//         // Internal helper using `super` to access the parent module's private constant
//         pub fn format_with_version(&self) -> String {
//             format!("[Collector v{}] {}: {}", super::COLLECTOR_VERSION, self.name, self.value)
//         }
//     }
// }
