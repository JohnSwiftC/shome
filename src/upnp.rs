use lazyhttp;
use std::io::Write;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

fn fake_broadcast(service_desc: &str) {
    let udpsocket = UdpSocket::bind("0.0.0.0:1900").unwrap();
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    let response_headers = "HTTP/1.1 200 OK\r\n\
        Content-Type: text/xml\r\n\
        Transfer-Encoding: chunked\r\n\
        Server: Linux UPnP/1.0 Sonos/85.0-65270 (ZPS16)\r\n\
        Connection: close\r\n\r\n";

    let ssdp_service_desc: Vec<u8> =
        std::fs::read(service_desc).expect("Failed to open SSDP Service Desc");

    // Spawn http "server" thread
    let t = thread::spawn(move || {
        for stream in listener.incoming() {
            println!("Connection Incoming...");
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Error when getting incoming TCP stream");
                    continue;
                }
            };

            if let Ok(req) = lazyhttp::handle_stream(&stream) {
                println!("{}", req.uri().path());

                // Send headers
                match stream.write_all(response_headers.as_bytes()) {
                    Ok(_) => println!("Headers Sent..."),
                    Err(e) => eprintln!("Headers failed to send... {}", e),
                }

                // Send service desc
                match send_chunked_response(&mut stream, &ssdp_service_desc) {
                    Ok(_) => println!("Service desc sent..."),
                    Err(e) => eprintln!("Service desc failed to send... {}", e),
                }
            }
        }
    });

    // Just mimicking sonos speaker for now
    let ssdp_ad = "NOTIFY * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nCACHE-CONTROL: max-age = 1800\r\nLOCATION: http://192.168.1.120:80/xml/device_description.xml\r\nNT: upnp:rootdevice\r\nNTS: ssdp:alive\r\nSERVER: Linux UPnP/1.0 Sonos/85.0-65270 (ZPS16)\r\nUSN: uuid:RINCON_347E5C08876401400::upnp:rootdevice\r\nX-RINCON-HOUSEHOLD: Sonos_BcSIIoVehzL5k64XqHiPV2k0zT\r\nX-RINCON-BOOTSEQ: 138\r\nBOOTID.UPNP.ORG: 138\r\nX-RINCON-WIFIMODE: 0\r\nX-RINCON-VARIANT: 2\r\nHOUSEHOLD.SMARTSPEAKER.AUDIO: Sonos_BcSIIoVehzL5k64XqHiPV2k0zT.uTXkmeZFPArTKU20mbQE\r\nLOCATION.SMARTSPEAKER.AUDIO: lc_edbbd23a2134468a8236d24514748b54\r\nSECURELOCATION.UPNP.ORG: https://192.168.1.8:1443/xml/device_description.xml\r\nX-SONOS-HHSECURELOCATION: https://192.168.1.8:1843/xml/device_description.xml\r\n\r\n";

    // ssdp usually hits three packets to ensure a transmission
    for _ in 0..3 {
        udpsocket
            .send_to(ssdp_ad.as_bytes(), "239.255.255.250:1900")
            .expect("Broadcast packet failed!");
    }

    t.join().unwrap();
}

fn send_chunked_response(stream: &mut TcpStream, data: &[u8]) -> std::io::Result<()> {
    // Send the chunk size in hexadecimal followed by CRLF
    let chunk_size = format!("{:X}\r\n", data.len());
    stream.write_all(chunk_size.as_bytes())?;

    // Send the actual data followed by CRLF
    stream.write_all(data)?;
    stream.write_all(b"\r\n")?;

    // Send the final chunk (size 0) to indicate end of response
    stream.write_all(b"0\r\n\r\n")?;

    stream.flush()?;
    println!("Chunked response sent successfully");

    Ok(())
}

fn read_ssdp() {
    let socket = UdpSocket::bind("0.0.0.0:1900").expect("Failed to open UDP socket");
    loop {
        let mut buf = vec![0; 4096];
        let (bytes, sender) = match socket.recv_from(&mut buf) {
            Ok((bytes, sender)) => (bytes, sender),
            Err(e) => {
                eprintln!("{}", e.to_string());
                continue;
            }
        };
        let bufstr = String::from_utf8_lossy(&buf[..bytes]);
        println!("Packet from {}\n{}", sender, bufstr);
    }
}
