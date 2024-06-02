use std::net::{TcpListener, TcpStream};
use std::io::{self,Read, Write};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct OTAData {
    pub operation: String,
    pub value : String,
}
// const SUPPORTED_OPERATIONS: [&str; 1] = ["Hello"];

pub fn receive_ota_message(mut stream: TcpStream) -> OTAData {
    let mut buffer = [0; 512];
    let _ = stream.read(&mut buffer);
    
    let received_data = String::from_utf8_lossy(&buffer);
    let message:Result<OTAData, serde_json::Error> = serde_json::from_str(&received_data.trim_end_matches(char::from(0)));
    
    match message {
        Ok(message)=>{
            println!("Received Key: {}", message.value);
            return message;
        },
        Err(_)=> todo!()
    }
    
    
    // Ok(message)
}

pub fn send_ota_message(mut stream: TcpStream, data:OTAData){

    let serialized_message = serde_json::to_string(&data);
    match serialized_message {
        Ok(serialized_message) => {
            let _ = stream.write_all(serialized_message.as_bytes());
            println!("OTA Key ({}) Sent Successfully!", data.value);
        },
        Err(_) => todo!(),
    }
}

pub fn server(port : String ) -> std::io::Result<TcpStream> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port))?;
    println!(">>> Server listening on {}",port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // thread::spawn(|| receive_ota_message(stream));
                return Ok(stream);
            } 
            Err(e) =>{ 
                println!("Connection failed: {}", e);
                return Err(e);
            }
        }
    }
    // If the loop exits without returning, return an error
    Err(io::Error::new(io::ErrorKind::Other, "Server stopped unexpectedly"))
}

pub fn client(host_ip:String, port : String ) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(format!("{}:{}",host_ip,port))?;
    println!(">>> Connected to server");
    return Ok(stream);
    // let message = OTAData{operation:"Hello".to_string(),value:"0".to_string()};

    // let _ = send_ota_message(stream, message);
    
}