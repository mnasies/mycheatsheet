use std::io::{self, Write};

/// Method used to copy text to the clipboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardMethod {
    SystemClipboard,
    Osc52Terminal,
}

/// Copies text to the system clipboard using `arboard`, with fallback to ANSI OSC 52.
pub fn copy_to_clipboard(text: &str) -> Result<ClipboardMethod, String> {
    // 1. Try native system clipboard via arboard
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if clipboard.set_text(text.to_string()).is_ok() {
            return Ok(ClipboardMethod::SystemClipboard);
        }
    }

    // 2. Fallback to ANSI OSC 52 terminal copy sequence
    if copy_via_osc52(text).is_ok() {
        return Ok(ClipboardMethod::Osc52Terminal);
    }

    Err("Clipboard unavailable (both system clipboard and OSC 52 failed)".to_string())
}

/// Emits an ANSI OSC 52 copy sequence to stdout.
/// Supported in modern terminals (Alacritty, Kitty, WezTerm, iTerm2, Windows Terminal, tmux).
fn copy_via_osc52(text: &str) -> io::Result<()> {
    let encoded = base64_encode(text.as_bytes());
    let mut stdout = io::stdout();
    // \x1b]52;c;<base64>\x07
    write!(stdout, "\x1b]52;c;{}\x07", encoded)?;
    stdout.flush()
}

/// Minimal base64 encoder for OSC 52 escape sequences.
fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len().div_ceil(3) * 4);

    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        result.push(CHARSET[(b0 >> 2) as usize] as char);
        result.push(CHARSET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARSET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARSET[(b2 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"Hello, World!"), "SGVsbG8sIFdvcmxkIQ==");
    }
}
