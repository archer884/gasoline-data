extern crate gasoline_data;

use gasoline_data::service::*;

fn main() {
    let mut service = PgConnectionService::new();    
    match service.fillups().create((1, 1, 1122, 5.1)) {
        Err(e) => println!("Error: {}", e),
        Ok(n) => println!("Rows inserted: {}", n),
    }
}
