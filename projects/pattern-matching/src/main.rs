// exercise one 


// fn http_category(status: u16) -> &'static str {
//     match status {
//         100..=199 =>"Informational",
//         200..=299 => "Success",
//         300..=399 => "Redirection",
//         400..=499 => "Client Error",
//         500..=599 =>"Server Error",
//         _ => "Invalid Status Code",
//     }
// }


// fn main(){
//     let status = 200;
//     println!("{}",http_category(status));
// }
// 
// 


// exercise 2 
// 



// enum TaskPriority {
//     Low,
//     Medium,
//     High { urgency_level: u8 },
//     Critical(String),
// }


// fn estimate_sla_hours(task: &TaskPriority) -> u32 {
//     match task {
//         TaskPriority::Low => 72,
//         TaskPriority::Medium => 24 ,
//         TaskPriority::High { urgency_level } if *urgency_level > 5 => 2,
//         TaskPriority::High { .. } => 6,
//         TaskPriority::Critical(_) => 1,
//     }
// }


// fn main() {
//     let task1 = TaskPriority::High { urgency_level: 8 };
//     let task2 = TaskPriority::Critical(String::from("Database unreachable"));

//     println!("Task 1 SLA: {} hours", estimate_sla_hours(&task1));
//     println!("Task 2 SLA: {} hours", estimate_sla_hours(&task2));
// }


// exercise 3 


// enum WebSocketFrame {
//     Text(String),
//     Binary(Vec<u8>),
//     Ping,
//     Close,
// }

// fn handle_frame(frame: WebSocketFrame) {
//     // match frame {
//     //     WebSocketFrame::Text(message) => {
//     //         println!("Received text message: {}", message);
//     //     }
//     //     _ => (),
//     // }
//     // 
//     if let WebSocketFrame::Text(message) = frame {
//         println!("Received text message: {}", message);
//     }
// }
