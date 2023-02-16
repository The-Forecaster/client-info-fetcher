use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut username = String::new();
    let mut hwid = String::new();
    let mut ign = String::new();

    println!("What is your slug username?: ");
    stdin()
        .read_line(&mut username)
        .expect("I didn't get that sorry");

    println!("What is your hwid?: ");
    stdin()
        .read_line(&mut hwid)
        .expect("I didn't get that sorry");

    println!("what is your ign?: ");
    stdin()
        .read_line(&mut ign)
        .expect("I didn't get that sorry");

    let res = client
        .post("http://127.0.0.1:7878")
        .body(format!("Username : {username}, hwid : {hwid}, ign : {ign}"))
        .send();

    // let request = format!("POST /post HTTP/1.1\r\nContent-Length: {length}\r\n\r\n{content}");

    Ok(())
}
