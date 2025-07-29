mod dialog;
//pub mod exe; //TODO: Exe functionality is Heavily Expermiental,and is Broken at the moment.
mod icon;
mod shortcut;
mod web_shortcut;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the icon file (e.g., .ico, .png, .webp)
    icon: Option<String>,

    /// Register the application in the "Send To" menu
    #[arg(long)]
    register: bool,

    /// Unregister the application from the "Send To" menu
    #[arg(long)]
    unregister: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.register {
        utils::register_send_to_menu().context("Failed to register in Send To menu")?;
        utils::log_info("🎴 Application registered in 'Send To' menu. 🎴");
        return Ok(());
    }

    if cli.unregister {
        utils::unregister_send_to_menu().context("Failed to unregister from Send To menu")?;
        utils::log_info("Application unregistered from 'Send To' menu.");
        return Ok(());
    }

    let icon_path_str = cli.icon.context("Icon path not provided. Use --icon <FILE> or drag and drop an image on the app,alternatively,use the Send To menu.")?;
    let path = Path::new(&icon_path_str);
    println!("Processing image: {}", path.display());

    if let Some(original_selected_path) = dialog::select_target_file_dialog() {
        if original_selected_path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("lnk")) {
            let target_path = shortcut::resolve_shortcut_target(&original_selected_path).context("Failed to resolve shortcut target")?;
            let ico_output_dir = target_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();

            let target_name = target_path.file_stem().unwrap().to_str().unwrap();
            let ico_path = ico_output_dir.join(format!("{}_custom.ico", target_name));

            icon::convert_to_ico(path, &ico_path).context("Icon conversion failed")?;

            if original_selected_path.is_dir() {
                shortcut::set_folder_icon(&original_selected_path, &ico_path)
                    .context("Failed to set folder icon")?;
            } else {
                shortcut::set_shortcut_icon(&original_selected_path, &ico_path)
                    .context("Failed to set shortcut icon")?;
            }

            println!("🎉 Icon set successfully!");
            utils::log_info("🎴 Image was set as icon! If it doesn't appear, restart your explorer.exe! 🎴");
        } else if original_selected_path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("url")) {
            let url_content = web_shortcut::read_url_file(&original_selected_path)
                .context("Failed to read .url file")?;
            let url = url_content.get("URL").context("URL not found in .url file")?;
            
            let ico_output_dir = original_selected_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            let target_name = original_selected_path.file_stem().unwrap().to_str().unwrap();
            let ico_path = ico_output_dir.join(format!("{}_custom.ico", target_name));

            icon::convert_to_ico(path, &ico_path).context("Icon conversion failed")?;

            let temp_url_path = original_selected_path.with_extension("tmp.url");

            // Check if it's a Steam shortcut
            if url.starts_with("steam://rungameid/") {
                let id = web_shortcut::get_steam_id_from_url(url);
                web_shortcut::create_url_file(&temp_url_path, url, id.as_deref(), Some(&ico_path))
                    .context("Failed to create new Steam .url file")?;
                println!("🎉 Steam shortcut icon set successfully!");
                utils::log_info("🎴 Image was set as icon for Steam shortcut! If it doesn't appear, restart your explorer.exe! 🎴");
            } else {
                web_shortcut::create_url_file(&temp_url_path, url, None, Some(&ico_path))
                    .context("Failed to create new web .url file")?;
                println!("🎉 Web shortcut icon set successfully!");
                utils::log_info("🎴 Image was set as icon for web shortcut! If it doesn't appear, restart your explorer.exe! 🎴");
            }

            fs::remove_file(&original_selected_path)
                .context("Failed to delete original .url file")?;

            fs::rename(&temp_url_path, &original_selected_path)
                .context("Failed to rename temporary .url file to original name")?;
        } else if original_selected_path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("exe")) {
            utils::log_info("Sorry, EXE Functionality is not implemented as of now, Create a shortcut instead.");
        } else {
            utils::log_error("Unsupported file type selected.");
        }
    } else {
        utils::log_error("No file selected.");
    }

    Ok(())
}
