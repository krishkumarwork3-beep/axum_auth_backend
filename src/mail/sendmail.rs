use std::{env, fs};
use lettre::{
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport,
    Transport,
};

pub async fn send_email(
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)]
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_server = env::var("SMTP_SERVER")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;

}