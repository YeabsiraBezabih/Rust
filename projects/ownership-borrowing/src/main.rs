/*
 * // The Three Inviolable Rules
 * 1. Every value in Rust has an owner (a variable).
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value is automatically dropped (freed from memory).
 *
 */

// move semantics (heap types)

// fn main(){
//     let host:String = "192.168.1.1";
//     let ip = host; // host's data ownership is moved from host to ip, ip no longer avalible .

// }

// copy semantics
//
// on scalar and fixed arrays and tuple data types there is no ownership transfer
// the data is copied in new stack

// fn main() {
//     let array: [u8; 5] = [0; 5];
//     let new = array;

//     println!("the first{:?}, the copied {:?}", array, new);
// }
//

// deep duplication
// here if we want to duplicate heap allocated data with out transfering ownership
// we can do that by .clone()
//

// fn main() {
//     let text: String = String::from("random text");
//     let copied_text = text.clone();

//     println!("the first {}, the copied {}", text, copied_text);
// }

// fn ping_server(url: String) {
//     println!("Pinging: {}", url);
// }

// fn main() {
//     let server_url = String::from("https://api.internal.net");
//     let copy_server_url = server_url.clone();
//     ping_server(copy_server_url);
//     println!("Ping completed for: {}", server_url);
// }
// fn process_transaction(mut tx: String) -> (String, usize) {
//     let original_length = tx.len();
//     tx.push_str(" [PROCESSED]");
//     (tx, original_length)
// }

// fn main() {
//     let transaction_record = String::from("TX_9812_AUTH_CAPTURE");

//     let (updated_record, byte_count) = process_transaction(transaction_record);

//     println!("Updated Record: {}", updated_record);
//     println!("Original Length: {} bytes", byte_count);
// }

// fn verify_token(token: &String) -> bool {
//     token.starts_with("Bearer_")
// }

// fn main() {
//     let auth_token = String::from("Bearer_secret99");
//     let is_valid = verify_token(&auth_token);

//     println!("Auth Token: {}", auth_token);
//     println!("Token Valid: {}", is_valid);
// }




fn inject_content_length(headers: &mut String, length: usize) {
    headers.push_str("\nContent-Length: ");
    headers.push_str(&length.to_string());
}

fn main() {
    let mut request_headers = String::from("POST /submit HTTP/1.1\nHost: api.local");

    inject_content_length(&mut request_headers, 2048);

    println!("Final Request Headers:\n{}", request_headers);
}