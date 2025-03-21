use std::env;
mod firefox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut flag: u32 = 0;
    
    if args.len() < 2 {
        return;
    }

    if args[1] == "firefox" {
        match firefox::GetFirefoxPasswordDbPaths() {
            Some((logins_json, key4_db)) => {
                // get encrypted logins
                let logins = firefox::read_logins_json(logins_json.to_str().unwrap());
                if logins.is_err() {
                    return;
                }
                // get decryption key
                let decryption_key = firefox::extract_decryption_key(key4_db.to_str().unwrap());
                if decryption_key.is_err() {

                }
                println!("{:?}",decryption_key);

                println!("Firefox Password Database found:");
                println!("logins.json: {}", logins_json.display());
                println!("key4.db: {}", key4_db.display());

            },
            None => println!("Firefox pw DB not found")
        }
    }

    

    dbg!(args);
}