use crate::errors::AppError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
}

impl EmailService {
    pub fn new(
        smtp_host: &str,
        username: String,
        password: String,
        from_email: String,
    ) -> Result<Self, AppError> {
        let creds = Credentials::new(username, password);
        let mailer = SmtpTransport::relay(smtp_host)?.credentials(creds).build();

        Ok(Self { mailer, from_email })
    }

    pub fn send_meeting_invitation(
        &self,
        to_email: &str,
        user_name: &str,
        meeting_title: &str,
        meeting_date: &str,
        meeting_time: &str,
    ) -> Result<(), AppError> {
        let subject = format!("Meeting Invitation: {}", meeting_title);

        let body = format!(
            r#"<div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                <h2 style="color: #333;">Meeting Invitation</h2>
                <p>Hi, <strong>{}</strong>!</p>
                <p>You've been invited to a meeting:</p>
                <div style="background: #f5f5f5; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p><strong>Title:</strong> {}</p>
                    <p><strong>Date:</strong> {}</p>
                    <p><strong>Time:</strong> {}</p>
                </div>
                <p style="color: #666; font-size: 12px;">This email was sent automatically</p>
            </div>"#,
            user_name, meeting_title, meeting_date, meeting_time
        );

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)?;

        self.mailer.send(&email)?;
        Ok(())
    }
}
