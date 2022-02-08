use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::fs::{self, File};
use std::io::Read;
use std::env;
use std::path::PathBuf;

fn get_file(dir : PathBuf) -> String {
    let mut file = File::open(dir).unwrap();
    let mut result = String::new();
    file.read_to_string(&mut result).unwrap();
    result
}
#[tokio::main]
async fn main() {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); dir.pop(); dir.pop();
    let mut dir_cloned = dir.clone();
    dir_cloned.push("config");
    let mut dir_cloned_ip = dir.clone();
    dir_cloned_ip.push("last_ip");
    
    let result_config = get_file(dir_cloned);
    let result_ip = get_file(dir_cloned_ip);
    let mut config = result_config.split_ascii_whitespace();
    let email_dest = config.next().unwrap().to_string();
    let password = config.next().unwrap().to_string();
    let mut resultat = String::from("Le server à redémarer, voici la response du server : <br>");
    
    let ip_reponse = {
        if let Some(ip) = public_ip::addr_v4().await {
            ip.to_string()
        } else {
            "couldn't get an IP address".to_string()
        }
    };
    if result_ip.eq(&*ip_reponse) {
        ()
    }
    else {
        resultat.push_str("<b>");
        resultat.push_str(&*ip_reponse);
        resultat.push_str("<b>");
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
        dir.push("last_ip");
        fs::write(dir, ip_reponse).unwrap();
    }
    
}