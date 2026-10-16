//! Browser opening and single-instance management.

pub mod instance;
pub mod open;

pub use instance::check_existing_instance;
pub use open::open_browser;
