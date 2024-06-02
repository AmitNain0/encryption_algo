use std::{env, process};
use std::net::IpAddr;

#[derive(Debug)]
pub struct Target {
    pub role : String, // Role of current device
    pub port: String,
    pub ip_addr: String,
}

impl Target {

    pub fn configure(args : &Vec<String>)-> Target{
        
        let role = args[1].clone();
        let port = args[2].clone();
        let ip_addr = args[3].clone();
        // let (role, ip_addr, port) = args;
        
        // if args[1] != "C" || args[1] !="H"{  >> For Comparing string Don't use '==' 
        // This compares the address and NOT the actual string 
        if !(role.eq("C") || role.eq("H")){
            eprintln!("!!! Invalid Argument. Expected 'H'(HOST) or 'C'(Client), got {}. Exiting..",role);
            process::exit(-1);
        }else if role.eq("C") {
            println!(">>> Acting as Client!");
        }else{
            println!(">>> Acting as Host!");
        }
    
        // let target_ip_addr = args[2].clone();
        // let target_port_addr = args[3].clone();
        
        // Verify that the provided address is indeed a correct IP address 
        match ip_addr.parse::<IpAddr>(){
            Ok(ip)=> {
                println!(">>> IP ({}) address is valid!",ip);
                // println!(">>> Target PORT address : {}",port);
            }
            Err(_) => {
                eprintln!("!!! IP Address is not valid !!!");
            }
        }

        Target{role,ip_addr,port}
    }

    pub fn get_arguments()-> Vec<String>{
        // Reading command line args 
        // Declare a vector to store the arguments
        let cmd_args  = env::args(); // this returns an iterator 
        let args: Vec<String> = cmd_args.collect(); // This transforms iterators into a Vector 
    
        // for arg in args{
        //     println!("ARG : {}",arg);
        // }
    
        // Expect Args to be like this :: encrypter host {client ip} {client port}
        if args.len() < 4{
            eprintln!("!!! Arguments Missing !!!");
            println!("The following Arguments are required : ");
            println!("1. Host(H)/Client(C) \n2. Target IP address\n3. Target Port address\n");
            process::exit(-1);
        }else{
            args
        }
    

    }
}