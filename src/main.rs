use std::{error::Error, io::stdin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = read_line("What is your username?: ");
    let hwid = read_line("What is your hwid?: ");
    let ign = read_line("what is your ign?: ");

    let request = reqwest::Client::new()
        .post("http://127.0.0.1:7878") // Loopback address until I set up an actual web-server
        .body(format!("User {}\nHWID {}\nIGN {}", &username, &hwid, &ign));
        
    println!("Request: {:?}", &request);
    
    let res = request
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