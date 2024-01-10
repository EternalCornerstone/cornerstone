use std::net::TcpListener;
use std::io::{Read, Write};

// Assuming `Api` is an enum or struct that represents your API endpoints
use crate::api::APIRoute;

pub struct CornerstoneServer {
    listener: TcpListener,
    routes: Vec<APIRoute>,
}

impl CornerstoneServer {
    // Create a new instance of the server
    pub fn new(address: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(Self {
            listener,
            routes: Vec::new(),
        })
    }
    // Add a route to the server
    pub fn add_route(&mut self, api: APIRoute) {
        self.routes.push(api);
    }
    // Function to start the server and listen for connections
    pub async fn run(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;

            let request = String::from_utf8_lossy(&buffer);

            // Determine the response based on the request and configured routes
            let response = self.handle_request(&request);

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Ok(())
    }
    // Handle incoming requests
    fn handle_request(&self, request: &str) -> String {
        // Split the request into lines and extract the first line (request line)
        let request_line = request.lines().next().unwrap_or("");
        
        // Split the request line into components
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");

        // Match the request against the configured routes
        for route in &self.routes {
            if path == route.url && method == "GET" {  // Assuming GET requests for simplicity
                return format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
                            route.handler.len(), route.handler);
            }
        }

        // If no routes match, return a 404 Not Found response
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 9\r\n\r\nNot Found".to_string()    
    }
}