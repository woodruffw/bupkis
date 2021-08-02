use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;
use threadpool::ThreadPool;

use crate::config;
use crate::util;

#[derive(RustEmbed)]
#[folder = "src/templates/"]
pub struct Templates;

#[derive(RustEmbed)]
#[folder = "src/css/"]
pub struct Style;

#[derive(RustEmbed)]
#[folder = "src/js/"]
pub struct Script;

#[derive(Debug, Serialize)]
pub struct Album {
    #[serde(skip)]
    inner: config::Album,
    javascript: bool,
    title: String,
    slug: String,
    description: String,
    photos: Vec<Photo>,
}

impl TryFrom<config::Album> for Album {
    type Error = anyhow::Error;

    fn try_from(album: config::Album) -> Result<Self> {
        // Collect the slugs for all photos, so that we can build our
        // little index for each individual photo view.
        let slugs = album
            .photos
            .iter()
            .map(|p| p.slugify())
            .collect::<Result<Vec<String>>>()?;

        let photos = album
            .photos
            .iter()
            .enumerate()
            .map(|(i, photo)| {
                // NOTE(ww): Silly way to prevent an overflow when we're on the first photo
                // in the album (i.e., index 0).
                let prev_slug = {
                    i.checked_sub(1)
                        .and_then(|i| album.photos.get(i))
                        .map(|p| p.slugify())
                        .transpose()?
                };
                let next_slug = album.photos.get(i + 1).map(|p| p.slugify()).transpose()?;

                #[allow(clippy::redundant_field_names)]
                Ok(Photo {
                    inner: photo.clone(),
                    javascript: album.javascript,
                    index: i,
                    slugs: slugs.clone(),
                    slug: photo.slugify()?,
                    prev_slug: prev_slug,
                    next_slug: next_slug,
                    title: album.title.clone(),
                    description: photo.desc.as_ref().map(|d| util::render_markdown(d)),
                    name: photo.name.clone(),
                    thumb_name: format!("thumb-{}", &photo.name),
                })
            })
            .collect::<Result<Vec<Photo>>>()?;

        #[allow(clippy::redundant_field_names)]
        Ok(Self {
            javascript: album.javascript,
            title: album.title.clone(),
            slug: slug::slugify(&album.title),
            description: util::render_markdown(&album.desc),
            photos: photos,
            inner: album,
        })
    }
}

impl Album {
    pub fn render(&self, hbs: &Handlebars, output_dir: impl AsRef<Path>) -> Result<()> {
        let album_dir = output_dir.as_ref().join(&self.slug);
        if !album_dir.is_dir() {
            fs::create_dir_all(&album_dir)?;
        }

        // Render each photo; use a thread pool to speed up the thumbnailing operations.
        let pool = ThreadPool::new(4);
        let spinner = util::spinner("copying, thumbing, and rendering...");
        for photo in &self.photos {
            let photo_dir = album_dir.join(&photo.slug);
            if !photo_dir.is_dir() {
                fs::create_dir_all(&photo_dir)?;
            }

            let src_image = self.inner.path.join(&photo.inner.name);

            let dst_image = photo_dir.join(&photo.inner.name);
            let dst_thumb = photo_dir.join(&photo.thumb_name);

            if !dst_image.exists() {
                fs::copy(&src_image, &dst_image)?;
            }

            if !dst_thumb.exists() {
                log::debug!("thumbnail: {:?} -> {:?}", src_image, dst_thumb);
                pool.execute(move || {
                    util::thumbnail(&src_image, &dst_thumb)
                        .unwrap_or_else(|_| panic!("thumbnailing of {:?} failed", src_image));
                });
            }

            let content = hbs.render("photo", &photo)?;
            fs::write(photo_dir.join("index.html"), &content)?;
        }

        pool.join();

        // If any of our thumbnailing tasks fails, die.
        if pool.panic_count() > 0 {
            return Err(anyhow!("one or more thumbnailing tasks failed"));
        }

        spinner.finish_with_message("copying, thumbing, and rendering...done");

        let content = hbs.render("album", &self)?;
        fs::write(album_dir.join("index.html"), &content)?;

        fs::write(
            album_dir.join("style.css"),
            Style::get("style.css").unwrap().data,
        )?;

        fs::write(
            album_dir.join("photo.js"),
            Script::get("photo.js").unwrap().data,
        )?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Photo {
    #[serde(skip)]
    inner: config::Photo,
    javascript: bool,
    index: usize,
    slugs: Vec<String>,
    slug: String,
    prev_slug: Option<String>,
    next_slug: Option<String>,
    title: String,
    description: Option<String>,
    name: String,
    thumb_name: String,
}
