/// Format a number with thousand separators: 1234567 -> "1,234,567"
pub fn format_number(n: i64) -> String {
    let s = n.abs().to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    if n < 0 {
        result.push('-');
    }
    result.chars().rev().collect()
}

/// Format a large number with suffixes: 1200000 -> "1.2M"
pub fn format_number_large(n: i64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        format_number(n)
    }
}

/// Format a milliseconds duration into human-readable string
pub fn format_time(ms: i64) -> String {
    if ms <= 0 {
        return "0s".to_string();
    }
    let secs = ms / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h {}m", days, hours % 24, mins % 60)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins % 60, secs % 60)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs % 60)
    } else {
        format!("{}s", secs)
    }
}

/// Get remaining cooldown in milliseconds (returns 0 if cooldown has passed)
pub fn get_cooldown(last: i64, cooldown_ms: i64) -> i64 {
    let now = chrono::Utc::now().timestamp_millis();
    let remaining = cooldown_ms - (now - last);
    if remaining > 0 { remaining } else { 0 }
}

/// Apply the "Alya style" Unicode text transformation
pub fn style_text(text: &str) -> String {
    text.replace('a', "ᥲ")
        .replace('e', "ꫀ")
        .replace('u', "ᥙ")
        .replace('x', "ꪎ")
        .replace('y', "ᥡ")
}

/// Get random element from a slice
pub fn get_random<T: Clone>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    use rand::Rng;
    let idx = rand::thread_rng().gen_range(0..list.len());
    Some(list[idx].clone())
}

pub fn get_random_int(min: i64, max: i64) -> i64 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..=max)
}
