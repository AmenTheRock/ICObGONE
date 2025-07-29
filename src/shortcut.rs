//Handles main shortcut creation logic,Folder Functionality is...Unimplemented,ouch.
use anyhow::{anyhow, Ok, Result, Context};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use windows::{
    core::*,
    Win32::System::Com::*,
    Win32::UI::Shell::*,
    Win32::Storage::FileSystem::{SetFileAttributesW, GetFileAttributesW, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM, FILE_ATTRIBUTE_HIDDEN, FILE_FLAGS_AND_ATTRIBUTES},
};
use std::fs;
use std::io::Write;
use windows::Win32::UI::Shell::SLGP_RAWPATH;

const MAX_PATH_LEN: usize = 260;

pub fn set_shortcut_icon(shortcut_path: &Path, icon_path: &Path) -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let persist_file: IPersistFile = shell_link.cast()?;
        let mut shortcut_path_wide: Vec<u16> = shortcut_path.as_os_str().encode_wide().collect();
        shortcut_path_wide.push(0); // Null terminator
        persist_file.Load(PCWSTR(shortcut_path_wide.as_ptr()), STGM_READWRITE).ok().ok_or_else(|| anyhow!("Failed to load shortcut file"))?;
        let mut icon_path_wide: Vec<u16> = icon_path.as_os_str().encode_wide().collect();
        icon_path_wide.push(0); // Null terminator
        shell_link.SetIconLocation(PCWSTR(icon_path_wide.as_ptr()), 0).ok().ok_or_else(|| anyhow!("Failed to set icon location"))?;
        persist_file.Save(PCWSTR(shortcut_path_wide.as_ptr()), true).ok().ok_or_else(|| anyhow!("Failed to save shortcut file"))?;
    }
    Ok(())
}

pub fn set_folder_icon(folder_path: &Path, icon_path: &Path) -> Result<()> {
    let desktop_ini_path = folder_path.join("desktop.ini");
    let icon_path_relative = icon_path.strip_prefix(folder_path.parent().unwrap()).unwrap_or(icon_path);

    let content = format!(
        r"[.ShellClassInfo]\r\nIconFile={}\r\nIconIndex=0\r\n",
        icon_path_relative.display()
    );

    let mut file = fs::File::create(&desktop_ini_path)
        .context("Failed to create desktop.ini")?;
    file.write_all(content.as_bytes())
        .context("Failed to write to desktop.ini")?;

    unsafe {
        let folder_path_wide: Vec<u16> = folder_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        let desktop_ini_path_wide: Vec<u16> = desktop_ini_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

        // Set folder attributes
        let mut attributes = FILE_FLAGS_AND_ATTRIBUTES(GetFileAttributesW(PCWSTR(folder_path_wide.as_ptr())));
        if attributes.0 == 0 {
            return Err(anyhow!("Failed to get folder attributes"));
        }
        attributes |= FILE_ATTRIBUTE_READONLY | FILE_ATTRIBUTE_SYSTEM;
        SetFileAttributesW(PCWSTR(folder_path_wide.as_ptr()), attributes)
            .ok().context("Failed to set folder attributes")?;

        // Set desktop.ini attributes
        let mut ini_attributes = FILE_FLAGS_AND_ATTRIBUTES(GetFileAttributesW(PCWSTR(desktop_ini_path_wide.as_ptr())));
        if ini_attributes.0 == 0 {
            return Err(anyhow!("Failed to get desktop.ini attributes"));
        }
        ini_attributes |= FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM;
        SetFileAttributesW(PCWSTR(desktop_ini_path_wide.as_ptr()), ini_attributes)
            .ok().context("Failed to set desktop.ini attributes")?;
    }

    Ok(())
}

pub fn resolve_shortcut_target(shortcut_path: &Path) -> Result<PathBuf> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let persist_file: IPersistFile = shell_link.cast()?;
        let shortcut_path_wide: Vec<u16> = shortcut_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        persist_file.Load(PCWSTR(shortcut_path_wide.as_ptr()), STGM_READWRITE).ok().ok_or_else(|| anyhow!("Failed to load shortcut file"))?;

        let mut path_buf = [0u16; MAX_PATH_LEN];
        shell_link.GetPath(path_buf.as_mut_slice(), std::ptr::null_mut(), SLGP_RAWPATH.0 as u32).ok().ok_or_else(|| anyhow!("Failed to get shortcut target path"))?;

        let target_path = String::from_utf16_lossy(&path_buf).trim_end_matches('\0').to_string();
        Ok(PathBuf::from(target_path))
    }
}