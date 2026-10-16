//! Build script for embedding Windows resources (icon and metadata).

fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/icon.ico");
        res.set("FileDescription", "mycheatsheet — Offline Syntax Reference");
        res.set("ProductName", "mycheatsheet");
        res.set("OriginalFilename", "mycheatsheet.exe");
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {}", e);
        }
    }
}
