use rand::Rng;

// For now, we are going to use 23 as prime and 9 as its root
// ToDo: make this whole piece Random

const PRIME: u64 = 23; // P
const ROOT : u64 = 9;  // G 

fn power(x:u64, y : u64)-> u64{
    let mut res = 1;
    let mut counter = 0;
    loop {
        res *= x as u64;
        counter +=1;
        if counter == y{
            // println!("Multiplied Numbers : {}", res);
            return res;
        }
    }
}

pub fn generate_private_key()->u64{

    println!(">>> Selected PRIME : {}",PRIME);
    println!(">>> Selected ROOT  : {}",ROOT);

    let mut rng = rand::thread_rng();

    // Generate a random integer between 1 and 100
    let random_int: u64 = rng.gen_range(1..10);
    println!(">>> Private Key: {}", random_int);
    random_int

}

pub fn generate_ota_secret(private_key : u64)->u64{
    // x = G^a mod P 
    let temp = power(ROOT, private_key) % PRIME;
    let generated_secret:u64 = temp as u64;
    println!(">>> Public Key : {}",generated_secret);
    generated_secret
}   

pub fn generate_encryption_key(received_key:u64 , private_key : u64) -> u64{
    let temp = power(received_key, private_key) % PRIME;
    let encryption_key = temp as u64; //(private_key.pow(received_key))%PRIME;
    println!("***** Generated Encryption Key : {} *****", encryption_key);
    encryption_key

}
