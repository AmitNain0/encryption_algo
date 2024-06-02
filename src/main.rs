use connection_manager::OTAData;
mod target_parameters; 
mod connection_manager;
mod diffie_hellman;
// use std::process;

//Diffie-Hellman algorithm is used to establish a shared secret over a public network

/*  
Part-1. Command line argument parser >> Done
Part-2. Socket Communication establishment >> Done
Part-3. Deffie-Hellman Algo implementation
*/

fn host_on_connect(stream: std::net::TcpStream,  host_ota_secret : u64, private_key: u64)-> std::io::Result<()>{
    // 1. Send the generated secret
    // 2. Receive the generated Secret

    // Part-1
    let cloned_stream = stream.try_clone()?;

    let data = OTAData{operation:"SECRET_SHARING".to_string(),value:host_ota_secret.to_string()};
    connection_manager::send_ota_message(stream, data);

    //Part-2
    let returned_result = connection_manager::receive_ota_message(cloned_stream);
    if returned_result.operation.eq("SECRET_SHARING"){
        let received_key = returned_result.value.parse::<u64>();
        match received_key {
            Ok(received_key) =>{
                diffie_hellman::generate_encryption_key(received_key, private_key);  
            },
            Err(_) => todo!(),
        }
    }
    Ok(())
}  

fn client_on_connect(stream: std::net::TcpStream,  client_ota_secret : u64, private_key: u64)-> std::io::Result<()>{
    // 1. Receive the secret key from Host
    // 2. Send the generated Secret

    // Part-2
    let cloned_stream = stream.try_clone()?;

    //Part-1
    let returned_result = connection_manager::receive_ota_message(cloned_stream);
    if returned_result.operation.eq("SECRET_SHARING"){
        let received_key = returned_result.value.parse::<u64>();
        match received_key {
            Ok(received_key) =>{
                let data = OTAData{operation:"SECRET_SHARING".to_string(),value:client_ota_secret.to_string()};
                connection_manager::send_ota_message(stream, data);
                
                diffie_hellman::generate_encryption_key(received_key, private_key);  
            },
            Err(_) => todo!(),
        }
    }

    Ok(())
}

fn main() {
    let args = target_parameters::Target::get_arguments();
    let device = target_parameters::Target::configure(&args);
    // println!("Target Details : {:?}", device);

    let private_key : u64 = diffie_hellman::generate_private_key();
    let ota_secret : u64 = diffie_hellman::generate_ota_secret(private_key);


    if device.role.eq("H"){
        let host = connection_manager::server(device.port);
        match host {
            Ok(stream) => {
                // On_connect
                let _ = host_on_connect(stream,ota_secret, private_key);
            }
            Err(_) => todo!(),
        }
        

    }else if device.role.eq("C"){
        let client = connection_manager::client(device.ip_addr,device.port);
        match client{
            Ok(stream) => {
                let _ = client_on_connect(stream,ota_secret, private_key);

            },
            Err(_) => todo!(),
        }
    }


}
