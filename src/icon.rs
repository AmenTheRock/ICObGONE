//Main icon conversion logic
use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;
use image::imageops::FilterType;

pub fn convert_to_ico(input: &Path, output: &Path) -> Result<()> {
    let img = image::open(input).context("Failed to open input image. Please ensure it's a valid image format.")?;
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    let sizes = [128];

    for &size in sizes.iter() {
        let resized_img = image::imageops::resize(&img, size, size, FilterType::Lanczos3);
        let image = resized_img;
        let icon_image = ico::IconImage::from_rgba_data(size, size, image.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }

    let file = File::create(output).context("Failed to create output .ico file")?;
    icon_dir.write(file).context("Failed to write .ico file")?;
    Ok(())
}
