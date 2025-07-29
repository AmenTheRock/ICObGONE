// LISTEN CAREFULLY: The exe code in here is highly unstable, very much prone to crashing,and may not work even if it compiles.
// There is a reason why I dont allow you to use the .exe option yet,this coed just..dies.
// It is here and will be in the github commit however, for future reference.
use anyhow::{Result, Context};
use std::path::Path;
use winres_edit::{resource_type, Id, Resources};
use ico::IconDir;

pub fn set_exe_icon(exe_path: &Path, icon_path: &Path) -> Result<()> {
    let icon_file = std::fs::File::open(icon_path)
        .context("Failed to open icon file")?;
    let icon_dir = IconDir::read(icon_file)
        .context("Failed to read icon directory")?;

    let target_icon = icon_dir
        .entries()
        .iter()
        .find(|&e| e.width() == 256) // Try to find a 256x256 icon
        .or_else(|| icon_dir.entries().iter().max_by_key(|e| e.width())) // Otherwise, find the largest icon
        .context("No suitable icon entry found in .ico file")?;
    let icon_data = target_icon.data().to_vec();

    // Create a temporary copy of the executable
    let temp_exe_path = std::env::temp_dir().join(format!("{}.tmp", uuid::Uuid::new_v4()));
    std::fs::copy(exe_path, &temp_exe_path).context("Failed to create temporary copy of executable")?;

    let mut resources = Resources::new(&temp_exe_path);
    resources.load().context("Failed to load resources from temporary executable")?;
    resources.open().context("Failed to open temporary executable for resource modification")?;

    resources
        .find(resource_type::ICON, Id::Integer(1)) // Common ID for main application icon
        .ok_or_else(|| anyhow::anyhow!("Unable to find main icon with ID 1 in the executable. Consider inspecting the executable's resources."))?
        .replace(&icon_data)
        .context("Failed to replace icon data")?
        .update()
        .context("Failed to update icon resource")?;

    resources.close();

    // Replace the original executable with the modified temporary copy
    std::fs::remove_file(exe_path).context("Failed to remove original executable")?;
    std::fs::rename(&temp_exe_path, exe_path).context("Failed to rename temporary executable to original")?;

    Ok(())
}
