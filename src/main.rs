use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::fs::File;
use std::io::Read;
use std::env;

#[tokio::main]
async fn main() {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("config");
    
    let mut file = File::open(dir).unwrap();
    let mut result = String::new();
    file.read_to_string(&mut result).unwrap();
    let mut config = result.split_ascii_whitespace();
    let email_dest = config.next().unwrap().to_string();
    let password = config.next().unwrap().to_string();
    let mut resultat = String::from("Le server à redémarer, voici la response du server : <br>");
    
    let ip_reponse = {
        if let Some(ip) = public_ip::addr().await {
            ip.to_string()
        } else {
            "couldn't get an IP address".to_string()
        }
    };
    resultat.push_str(&*ip_reponse);
    let email = EmailBuilder::new()
        .to(email_dest.clone())
        .from(email_dest.clone())
        .subject("The server has restarted")
        .html(resultat)
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new(email_dest, password))
        .transport();

    let result = mailer.send(email.into());

    println!("{:?}", result);
}