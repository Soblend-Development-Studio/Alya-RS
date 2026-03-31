use regex::Regex;
use once_cell::sync::Lazy;

static MENTION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(\d+)").unwrap());
static LINK_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(chat\.whatsapp\.com/[a-zA-Z0-9]+)|(whatsapp\.com/channel/[a-zA-Z0-9]+)").unwrap()
});

/// Extract @mentioned JIDs from a message body
pub fn extract_mentions(body: &str) -> Vec<String> {
    MENTION_RE
        .captures_iter(body)
        .map(|c| format!("{}@s.whatsapp.net", &c[1]))
        .collect()
}

/// Check if message contains a WhatsApp link
pub fn contains_whatsapp_link(body: &str) -> bool {
    LINK_RE.is_match(body)
}

/// Extract the phone number from a JID
pub fn extract_phone(jid: &str) -> String {
    let base = jid.split('@').next().unwrap_or("");
    let num = if base.contains(':') {
        base.split(':').next().unwrap_or(base)
    } else {
        base
    };
    num.to_string()
}

/// Check if a JID is a group JID
pub fn is_group_jid(jid: &str) -> bool {
    jid.ends_with("@g.us")
}

/// Build a stars string for gacha rarity
pub fn get_stars(value: i64) -> String {
    let rarity = (value / 400).min(5).max(0) as usize;
    "⭐".repeat(rarity.max(1))
}

pub fn get_rarity_text(value: i64) -> &'static str {
    let rarity = value / 400;
    if rarity >= 5 { "Legendario" }
    else if rarity >= 4 { "Mítico" }
    else if rarity >= 3 { "Raro" }
    else if rarity >= 2 { "Poco Común" }
    else { "Común" }
}

/// Fetch bytes from a URL
pub async fn get_buffer(url: &str) -> anyhow::Result<bytes::Bytes> {
    let resp = reqwest::get(url).await?;
    Ok(resp.bytes().await?)
}
