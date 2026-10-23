const MAX_CONNECTIONS: u32 = 50;


pub struct Client{
    client_count: u32,
}

impl Client {
    pub fn new() -> Client {
        Client { client_count: 0,}
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if self.client_count < MAX_CONNECTIONS {
            self.client_count += 1;
            Ok(())
        } else {
            Err(String::from("Pool full"))
            
        }
        
    }
}