//execrsise 1 

// struct HttpServerConfig {
//     host: String,
//     port: u16,
//     max_connections: u32,
//     enable_tls: bool,
// }

// fn main() {
//     let config = HttpServerConfig {
//         host: String::from("127.0.0.1"),
//         port: 8443,
//         max_connections: 5000,
//         enable_tls: true,
//     };

//     println!("Host: {}, Port: {}, Max: {}, TLS: {}", config.host, config.port, config.max_connections, config.enable_tls);
// }


// exercise 2



// struct HttpServerConfig {
//     host: String,
//     port: u16,
//     max_connections: u32,
//     enable_tls: bool,
// }


// impl HttpServerConfig {

//     fn new(host: String ,port: u16) -> HttpServerConfig {
        
//         let enable_tls = port == 443 || port == 8443;
        
//         HttpServerConfig { 
//             host, 
//             port, 
//             max_connections: 1000, 
//             enable_tls,
//         }
//     }

//     fn summary(&self) -> String {
//         let protocol = if self.enable_tls {"https"} else{ "http"};
//         format!("{}://{}:{} (Max:{})", protocol, self.host, self.port ,self.max_connections)
//     }
    
// }

// fn main() {
//     let config = HttpServerConfig::new(String::from("0.0.0.0"), 8443);
//     config.summary();
// }
// 


// // exercise 3 
// #[derive(Debug)]
// struct UnverifiedToken {
//     raw_token: String,
// }

// struct VerifiedToken {
//     user_id: u64,
// }

// impl UnverifiedToken {
//     fn verify(self) -> VerifiedToken {
//         VerifiedToken {
//             user_id: 42,
//         }
//     }
    
// }


// fn main(){
//     let user_token = UnverifiedToken{
//         raw_token: String::from("xgfchv4657"),
//     };
//     let user_id = user_token.verify();

//     println!("{}", user_id.user_id);

//     // println!("{}", user_token.raw_token);  // this doesn't work 
    
// }
// 
// 
// 
// 


// #[derive(Debug)]
// enum IpAddress {
//     // Variant 1: Stores 4 bytes directly on the stack
//     V4(u8, u8, u8, u8),
//     // Variant 2: Stores a heap-allocated String
//     V6(String),
// }

// impl IpAddress {
//     // Method to format the IP address into a standard display string
//     fn to_display_string(&self) -> String {
//         match self {
//             IpAddress::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
//             IpAddress::V6(addr_str) => addr_str.clone(),
//         }
//     }

//     // Method to check if the address is a local loopback
//     fn is_loopback(&self) -> bool {
//         match self {
//             IpAddress::V4(127, 0, 0, 1) => true,
//             IpAddress::V6(addr) if addr == "::1" => true,
//             _ => false,
//         }
//     }
// }

// #[derive(Debug)]
// enum WebhookPayload {
//     Ping,
//     UserCreated { user_id: u64, email: String },
//     PaymentReceived(u64), // Amount in cents
// }

// fn main() {
//     println!("--- 1. Modeling IP Addresses with Enums ---");
//     let home_v4 = IpAddress::V4(127, 0, 0, 1);
//     let gateway_v6 = IpAddress::V6(String::from("fe80::1ff:fe23:4567:890a"));

//     println!("IPv4: {} (Is Loopback: {})", home_v4.to_display_string(), home_v4.is_loopback());
//     println!("IPv6: {} (Is Loopback: {})", gateway_v6.to_display_string(), gateway_v6.is_loopback());

//     println!("\n--- 2. Processing Multi-Payload Webhooks ---");
//     let event_a = WebhookPayload::Ping;
//     let event_b = WebhookPayload::UserCreated {
//         user_id: 10482,
//         email: String::from("dev@systems.internal"),
//     };
//     let event_c = WebhookPayload::PaymentReceived(4999);

//     println!("Event A: {:?}", event_a);
//     println!("Event B: {:?}", event_b);
//     println!("Event C: {:?}", event_c);
// }

// #[derive(Debug)]
// enum NetworkProtocol {
//     Http { port: u16 },
//     Https { port: u16, cert_name: String },
//     Tcp(u16),
//     Disabled,
// }

// impl NetworkProtocol {

//     fn get_port(&self) -> u16 {
//         match self {
//             NetworkProtocol::Http { port } => *port,
//             NetworkProtocol::Https { port, .. } => *port,
//             NetworkProtocol::Tcp(port ) => *port,
//             NetworkProtocol::Disabled => 0,
            
//         }
//     }
// }


// fn main(){
//     let protocol = NetworkProtocol::Https { 
//         port: (443), 
//         cert_name: (String::from("prod_wildcard.crt")) 
//     };

//     println!("{:?}",protocol);

//     let proto = NetworkProtocol::Https {
//            port: 8443,
//            cert_name: String::from("local.crt"),
//        };
   
//        println!("Port for protocol: {}", proto.get_port());
// }
// 
// 



enum AgentExecutionState {
    Idle,
    Prompting { model: String, temperature: f64 },
    ToolExecution { tool_name: String, step_index: u32 },
    Finished { total_tokens_used: u64 },
    Failed(String),
}


impl AgentExecutionState {
    fn is_active(&self) -> bool {
        match  self {
            AgentExecutionState::ToolExecution { ..  } => true,
            AgentExecutionState::Prompting { ..  } => true,
            _ => false,
        }    
    }  
}