use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::Duration;

fn register_airplay_device(device_name: &str, device_id: &str, ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;
    
    let hostname = format!("{}.local.", device_name.replace(" ", "-"));
    
    let airplay_service = ServiceInfo::new(
        "_airplay._tcp.local.",
        device_name,
        &hostname,
        ip,
        port,
        create_airplay_txt_records(device_name, device_id),
    )?;
    
    let raop_instance_name = format!("{}@{}", device_id, device_name);
    let raop_service = ServiceInfo::new(
        "_raop._tcp.local.",
        &raop_instance_name,
        &hostname,
        ip,
        port,
        create_raop_txt_records(device_name, device_id),
    )?;
    
    // Register both services
    mdns.register(airplay_service)?;
    mdns.register(raop_service)?;
    
    println!("AirPlay device '{}' registered at {}:{}", device_name, ip, port);
    println!("Services registered:");
    println!("  - _airplay._tcp.local");
    println!("  - _raop._tcp.local");
    
    // Keep services running
    loop {
        std::thread::sleep(Duration::from_secs(30));
    }
}

fn create_airplay_txt_records(device_name: &str, device_id: &str) -> HashMap<String, String> {
    let mut records = HashMap::new();
    
    // Essential AirPlay TXT records that iOS looks for
    records.insert("deviceid".to_string(), device_id.to_string());
    records.insert("features".to_string(), "0x4A7FDFD5,0xBC155FDE".to_string()); // Video + Audio support
    records.insert("flags".to_string(), "0x204".to_string()); // Device flags
    records.insert("model".to_string(), "AppleTV3,2".to_string()); // Device model
    records.insert("protovers".to_string(), "1.0".to_string()); // Protocol version
    records.insert("srcvers".to_string(), "220.68".to_string()); // Source version
    records.insert("vv".to_string(), "2".to_string()); // Volume control version
    records.insert("pw".to_string(), "false".to_string()); // No password required
    records.insert("pk".to_string(), "b07727d6f6cd6e08b58ede525ec3cdeaa252ae9e".to_string()); // Public key (fake)
    
    records
}

fn create_raop_txt_records(device_name: &str, device_id: &str) -> HashMap<String, String> {
    let mut records = HashMap::new();
    
    // RAOP (Remote Audio Output Protocol) TXT records
    records.insert("txtvers".to_string(), "1".to_string());
    records.insert("ch".to_string(), "2".to_string()); // Stereo channels
    records.insert("cn".to_string(), "0,1,2,3".to_string()); // Supported codecs
    records.insert("et".to_string(), "0,3,5".to_string()); // Encryption types
    records.insert("sv".to_string(), "false".to_string()); // Server version
    records.insert("da".to_string(), "true".to_string()); // Device available
    records.insert("sr".to_string(), "44100".to_string()); // Sample rate
    records.insert("ss".to_string(), "16".to_string()); // Sample size
    records.insert("pw".to_string(), "false".to_string()); // Password required
    records.insert("vn".to_string(), "65537".to_string()); // Version number
    records.insert("tp".to_string(), "UDP".to_string()); // Transport protocol
    records.insert("md".to_string(), "0,1,2".to_string()); // Metadata support
    records.insert("vs".to_string(), "220.68".to_string()); // Version string
    records.insert("am".to_string(), "AppleTV3,2".to_string()); // Audio model
    records.insert("ek".to_string(), "1".to_string()); // Encryption key
    
    records
}

// HTTP server to handle AirPlay protocol requests
// Not really sure if I could get any use from this?
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;

fn start_airplay_http_server(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("AirPlay HTTP server listening on port {}", port);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 4096];
    
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);
        println!("Received request: {}", request.lines().next().unwrap_or(""));
        
        // Route AirPlay requests
        if request.contains("GET /info") {
            send_server_info(&mut stream);
        } else if request.contains("POST /play") {
            send_play_response(&mut stream);
        } else if request.contains("POST /stop") {
            send_stop_response(&mut stream);
        } else if request.contains("GET /scrub") {
            send_scrub_response(&mut stream);
        } else if request.contains("POST /volume") {
            send_volume_response(&mut stream);
        } else {
            send_default_response(&mut stream);
        }
    }
}

fn send_server_info(stream: &mut std::net::TcpStream) {
    let server_info = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>deviceid</key>
    <string>AA:BB:CC:DD:EE:FF</string>
    <key>features</key>
    <integer>1256677589</integer>
    <key>model</key>
    <string>AppleTV3,2</string>
    <key>protovers</key>
    <string>1.0</string>
    <key>srcvers</key>
    <string>220.68</string>
    <key>vv</key>
    <integer>2</integer>
    <key>pw</key>
    <false/>
</dict>
</plist>"#;

    send_response(stream, "200 OK", "text/x-apple-plist+xml", server_info);
}

fn send_play_response(stream: &mut std::net::TcpStream) {
    println!("Play request received - would start playback here");
    send_response(stream, "200 OK", "text/plain", "");
}

fn send_stop_response(stream: &mut std::net::TcpStream) {
    println!("Stop request received - would stop playback here");
    send_response(stream, "200 OK", "text/plain", "");
}

fn send_scrub_response(stream: &mut std::net::TcpStream) {
    send_response(stream, "200 OK", "text/plain", "duration: 0.000000\nposition: 0.000000");
}

fn send_volume_response(stream: &mut std::net::TcpStream) {
    println!("Volume change request received");
    send_response(stream, "200 OK", "text/plain", "");
}

fn send_default_response(stream: &mut std::net::TcpStream) {
    let html = "<html><body><h1>AirPlay Device</h1><p>Fake AirPlay receiver running</p></body></html>";
    send_response(stream, "200 OK", "text/html", html);
}

fn send_response(stream: &mut std::net::TcpStream, status: &str, content_type: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Server: AirTunes/220.68\r\n\
         Connection: close\r\n\r\n{}",
        status, content_type, body.len(), body
    );
    
    let _ = stream.write_all(response.as_bytes());
}
