use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn read_url_file(path: &Path) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(path)?;
    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(map)
}

pub fn create_url_file(
    path: &Path,
    url: &str,
    id: Option<&str>,
    icon_path: Option<&Path>,
) -> Result<()> {
    let mut content = String::from("[InternetShortcut]\n");
    content.push_str(&format!("URL={}\n", url));
    if let Some(id_val) = id {
        content.push_str(&format!("ID={}\n", id_val));
    }
    if let Some(icon_path_val) = icon_path {
        content.push_str(&format!("IconFile={}\n", icon_path_val.display()));
        content.push_str("IconIndex=0\n");
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn get_steam_id_from_url(url: &str) -> Option<String> {
    url.strip_prefix("steam://rungameid/")
       .map(|s| s.to_string())
}
