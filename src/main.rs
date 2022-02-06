use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

#[tokio::main]
async fn main() {
    // Attempt to get an IP address and print it.
    let mut resultat = String::from("Le server à redémarer, voici la response du server : <br>");
    let email_dest = String::from("L'email en question");
    let password = String::from("Mes couilles sur ton front");

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