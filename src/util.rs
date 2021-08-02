use std::path::Path;

use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{imageops, GenericImageView};
use indicatif::{ProgressBar, ProgressStyle};
use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new_ext(input, Options::ENABLE_STRIKETHROUGH);
    let mut output = String::new();

    html::push_html(&mut output, parser);

    output
}

pub fn thumbnail(src_path: impl AsRef<Path>, dst_path: impl AsRef<Path>) -> Result<()> {
    let image = ImageReader::open(&src_path.as_ref())?.decode()?;
    let thumb = imageops::resize(
        &image,
        image.width() / 2,
        image.height() / 2,
        imageops::FilterType::Triangle,
    );

    Ok(thumb.save(&dst_path.as_ref())?)
}

pub fn spinner(msg: &'static str) -> ProgressBar {
    let bar = ProgressBar::new_spinner().with_message(msg).with_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner} {msg}"),
    );
    bar.enable_steady_tick(130);

    bar
}
