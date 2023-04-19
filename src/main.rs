use std::{error::Error, io::stdin, env::var};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = read_line("What is your username?: ");
    let ign = read_line("what is your ign?: ");

    let res = reqwest::Client::new()
        .post("http://127.0.0.1:7878") // Loopback address until I set up an actual web-server
<<<<<<< Updated upstream
        .body(format!("User {}\nHWID {}\nIGN {}", &username, get_hwid(), &ign))
=======
        .body(format!("User {}\nHWID {}\nIGN {}", &username, &get_hwid(), &ign));
        
    println!("Request: {:?}", &request);
    
    let res = request
>>>>>>> Stashed changes
        .send()
        .await?;
        
    println!("Response: {:?}", res);

    Ok(())
}

fn read_line(prompt: &str) -> String {
    let std = stdin();
    let mut buffer = String::new();
    
    loop {
        println!("{prompt}");
        match std.read_line(&mut buffer) {
            Ok(_) => return buffer,
            Err(_) => println!("I didn't get that sorry")
        }
    }
}

<<<<<<< Updated upstream
fn get_var(name: &str) -> String {
    match var(name) {
        Ok(str) => str,
        Err(_) => "".to_string()
    }
}

fn get_hwid() -> String {
    get_var("os.name")
=======
fn get_hwid() -> String {
    "".to_string()
>>>>>>> Stashed changes
}