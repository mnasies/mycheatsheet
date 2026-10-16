//! Default web browser launcher.

/// Open a URL in the system's default web browser.
///
/// If browser launching fails (e.g. in headless environments), emits a user-friendly
/// message and returns an error without crashing.
pub fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match open::that(url) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!(
                "Note: Could not open default web browser automatically ({}).\nPlease open {} manually in your browser.",
                err, url
            );
            Err(Box::new(err))
        }
    }
}
