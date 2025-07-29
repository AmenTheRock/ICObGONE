//The Select Dialog
use rfd::FileDialog;
use std::path::PathBuf;

pub fn select_target_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Shortcut (.lnk, .url)", &["lnk", "url"])
/*   .add_filter("Executable (.exe)", &["exe"])
     To be re-added when EXE Functionality is FIXED.
*/        
        .set_title("Select a shortcut or executable to set the icon for...")
        .pick_file()
}