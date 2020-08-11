use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::net::UdpSocket;
use std::process::Command;


fn main() {
    let current_ip: String = get_current_ip_address();
    send_email(current_ip);
}

fn send_email(current_ip: String) {
    let smtp_user_name = env!("SMTP_USER_NAME");
    let smtp_password = env!("SMTP_PASSWORD");
    let smtp_from_mail = env!("SMTP_FROM_MAIL");
    let smtp_reply_mail = env!("SMTP_REPLY_MAIL");
    let smtp_to_mail = env!("SMTP_TO_MAIL");

    let email_client = Message::builder()
        .from(smtp_from_mail.parse().unwrap())
        .reply_to(smtp_reply_mail.parse().unwrap())
        .to(smtp_to_mail.parse().unwrap())
        .subject("Latest ip address")
        .body(current_ip)
        .unwrap();
    let cred_id = Credentials::new(
        smtp_user_name.to_string(),
        smtp_password.to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(cred_id)
        .build();

    match mailer.send(&email_client) {
        Ok(_) => println!("Success!"),
        Err(e) => panic!("Error! {}", e)
    }
}

fn get_current_ip_address() -> String {
    let ip_addr = get_current_ip().unwrap();
    let shell_ip = "curl ip.sb";
    let public_ip = Command::new("bash")
        .arg("-c")
        .arg(shell_ip)
        .output()
        .expect("127.0.0.1");

    let output_str = String::from_utf8_lossy(&public_ip.stdout);
    let public_ip_addr = output_str.to_string();
    let message = ip_addr + " \n" + &*public_ip_addr;
    return message;
}

fn get_current_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    return match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None,
    };
}
