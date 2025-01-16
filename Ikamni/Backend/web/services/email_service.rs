use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

/// メール送信サービス
pub struct EmailService {
    smtp_client: SmtpTransport,
    sender_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplate {
    template_name: String,
    subject: String,
    body_html: String,
    body_text: String,
}

impl EmailService {
    pub async fn send_demo_confirmation(&self, email: &str, demo_session: &DemoSession) -> Result<(), EmailError> {
        let template = self.load_template("demo_confirmation")?;
        let message = Message::builder()
            .from(self.sender_address.parse()?)
            .to(email.parse()?)
            .subject(template.subject)
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(lettre::message::SinglePart::plain(template.body_text))
                    .singlepart(lettre::message::SinglePart::html(template.body_html))
            )?;

        self.smtp_client.send(&message)?;
        Ok(())
    }
} 