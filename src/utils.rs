//Registering,Unregistering,and Message Boxes.
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONERROR, MB_ICONINFORMATION};
use windows::core::{PCWSTR, PWSTR, Interface};
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::env;
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_SendTo, KNOWN_FOLDER_FLAG};
use windows::Win32::System::Com::{CoTaskMemFree, CoInitializeEx, COINIT_APARTMENTTHREADED, CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::Win32::System::Com::IPersistFile;
use std::os::windows::ffi::OsStrExt;
use std::ffi::c_void;

pub fn log_error(msg: &str) {
    eprintln!("❌ {}", msg);
    let msg_wide: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
    let title_wide: Vec<u16> = "Error".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(msg_wide.as_ptr()),
            PCWSTR(title_wide.as_ptr()),
            MB_OK | MB_ICONERROR,
        );
    }
}

pub fn log_info(msg: &str) {
    println!("ℹ️ {}", msg);
    let msg_wide: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
    let title_wide: Vec<u16> = "Information".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(msg_wide.as_ptr()),
            PCWSTR(title_wide.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

pub fn register_send_to_menu() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().context("Failed to initialize COM")?;

        let psz_path: PWSTR = SHGetKnownFolderPath(&FOLDERID_SendTo, KNOWN_FOLDER_FLAG(0), None)
            .ok().context("Failed to get SendTo folder path")?;
        let send_to_path = PathBuf::from(psz_path.to_string().context("Failed to convert path to string")?);
        CoTaskMemFree(Some(psz_path.0 as *const c_void));

        let shortcut_path = send_to_path.join("ICObGONE - Use Image as Icon.lnk");
        let exe_path = env::current_exe().context("Failed to get current executable path")?;

        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).context("Failed to create ShellLink instance")?;
        shell_link.SetPath(PCWSTR(exe_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr())).context("Failed to set shortcut path")?;
        shell_link.SetDescription(PCWSTR("Use an image file to change a shortcut's icon.".encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr())).context("Failed to set shortcut description")?;

        let persist_file: IPersistFile = shell_link.cast().context("Failed to cast to IPersistFile")?;
        let shortcut_path_wide: Vec<u16> = shortcut_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        persist_file.Save(PCWSTR(shortcut_path_wide.as_ptr()), true).context("Failed to save shortcut")?;
    }
    Ok(())
}

pub fn unregister_send_to_menu() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().context("Failed to initialize COM")?;

        let psz_path: PWSTR = SHGetKnownFolderPath(&FOLDERID_SendTo, KNOWN_FOLDER_FLAG(0), None)
            .ok().context("Failed to get SendTo folder path")?;
        let send_to_path = PathBuf::from(psz_path.to_string().context("Failed to convert path to string")?);
        CoTaskMemFree(Some(psz_path.0 as *const c_void));

        let shortcut_path = send_to_path.join("ICObGONE - Use Image as Icon.lnk");

        if shortcut_path.exists() {
            std::fs::remove_file(&shortcut_path).context("Failed to remove shortcut file")?;
        } else {
            log_info("Shortcut not found in 'Send To' menu. Nothing to unregister.");
        }
    }
    Ok(())
}