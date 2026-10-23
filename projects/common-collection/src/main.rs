// use std::collections::HashMap;

// fn main() {
//     println!("--- 1. Vectors (Vec<T>) with Capacity Pre-allocation ---");

//     // Pre-allocating space for 8 packet IDs on the heap to prevent reallocations
//     let mut packet_queue: Vec<u32> = Vec::with_capacity(8);
//     println!("Initial -> Len: {}, Cap: {}", packet_queue.len(), packet_queue.capacity());

//     packet_queue.push(1001);
//     packet_queue.push(1002);
//     packet_queue.push(1003);

//     println!("After pushes -> Len: {}, Cap: {}", packet_queue.len(), packet_queue.capacity());

//     // Safe lookup using .get() returning Option<&T>
//     match packet_queue.get(2) {
//         Some(&id) => println!("Packet at index 2: {}", id),
//         None => println!("No packet at index 2"),
//     }

//     // Iterating and mutating in place
//     for packet_id in &mut packet_queue {
//         *packet_id += 10; // Dereference to mutate the value
//     }
//     println!("Updated queue: {:?}", packet_queue);

//     println!("\n--- 2. String Manipulation and Slicing ---");

//     let mut http_status = String::from("HTTP/1.1");
//     http_status.push(' ');          // Push a single char
//     http_status.push_str("200 OK"); // Push a string slice

//     println!("Constructed status line: {}", http_status);

//     // Iterating over Unicode characters safely
//     let emoji_tag = "Status: 🚀 Online";
//     print!("Characters: ");
//     for c in emoji_tag.chars() {
//         print!("'{}' ", c);
//     }
//     println!();

//     println!("\n--- 3. HashMaps with Entry API ---");

//     // Creating an HTTP header lookup table
//     let mut headers: HashMap<String, String> = HashMap::new();
//     headers.insert(String::from("Host"), String::from("api.domain.internal"));
//     headers.insert(String::from("Content-Type"), String::from("application/json"));

//     // Safe retrieval with .get()
//     if let Some(content_type) = headers.get("Content-Type") {
//         println!("Header Content-Type: {}", content_type);
//     }

//     // Frequency counting using the .entry() API
//     let request_methods = ["GET", "POST", "GET", "GET", "DELETE", "POST"];
//     let mut method_counts: HashMap<&str, u32> = HashMap::new();

//     for &method in &request_methods {
//         // Find existing count or insert 0, then increment
//         let count_ref = method_counts.entry(method).or_insert(0);
//         *count_ref += 1;
//     }

//     println!("Request Method Frequency: {:?}", method_counts);
// } 



// use std::collections::HashMap;



fn safe_remove_first(queue: &mut Vec<String>) -> Option<String> {
    if queue.is_empty() {
        None
    } else {
        Some(queue.remove(0))
    }
}

fn main() {
    let mut vector: Vec<String> = vec![String::from("req1"), String::from("req2")];

    let result = safe_remove_first(&mut vector);

    match result {
        Some(item) => println!("Removed item: {}", item),
        None => println!("Queue is empty!"),
    }

    println!("Remaining vector: {:?}", vector);
}