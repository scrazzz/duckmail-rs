pub(crate) fn format_email(email: &str) -> String {
    if email.contains("@duck.com") {
        email.to_string()
    } else {
        format!("{}@duck.com", email)
    }
}
