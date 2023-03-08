use std::{error::Error, io::stdin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
        
    let args = [(String::from("User"), username), (String::from("HWID"), hwid), (String::from("IGN"), ign)];

    let request = client
        .post("http://127.0.0.1:7878")
        .form(&args);
        
    println!("Request: {:?}", request);
    
    let res = request
        .send()
        .await?;

    println!("Response: {:?}", res);

    Ok(())
}
