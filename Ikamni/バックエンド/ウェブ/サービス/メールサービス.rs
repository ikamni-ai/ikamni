use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

/// メール送信サービス
pub struct MailService {
    smtp_client: SmtpTransport,
    sender_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailTemplate {
    template_mei: String,
    ken_mei: String,
    honbun_html: String,
    honbun_text: String,
}

impl MailService {
    pub async fn send_shiyou_kakunin(&self, mail: &str, session: &ShiyouSession) -> Result<(), MailError> {
        let template = self.load_template("shiyou_kakunin")?;
        let message = Message::builder()
            .from(self.sender_address.parse()?)
            .to(mail.parse()?)
            .subject(template.ken_mei)
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(lettre::message::SinglePart::plain(template.honbun_text))
                    .singlepart(lettre::message::SinglePart::html(template.honbun_html))
            )?;

        self.smtp_client.send(&message)?;
        Ok(())
    }
} 