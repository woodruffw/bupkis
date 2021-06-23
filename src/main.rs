use std::convert::TryFrom;

use anyhow::Result;
use clap::{App, Arg};
use handlebars::{handlebars_helper, Handlebars};

mod config;
mod util;
mod view;

fn app<'a>() -> App<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("skeleton")
                .about("generate an skeleton album.yml")
                .arg(
                    Arg::new("album-dir")
                        .about("the album directory")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("generate")
                .about("generate HTML for the given album")
                .arg(
                    Arg::new("album-dir")
                        .about("the album directory")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("output-dir")
                        .about("the output directory")
                        .index(2)
                        .required(true),
                ),
        )
}

fn main() -> Result<()> {
    env_logger::init();

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    // Unnecessarily annoying.
    handlebars_helper!(even: |x: u64| x % 2 == 0);
    handlebars_helper!(odd: |x: u64| x % 2 != 0);

    handlebars.register_helper("even", Box::new(even));
    handlebars.register_helper("odd", Box::new(odd));

    handlebars.register_template_string(
        "album",
        String::from_utf8(view::Templates::get("album.hbs").unwrap().to_vec())?,
    )?;
    handlebars.register_template_string(
        "photo",
        String::from_utf8(view::Templates::get("photo.hbs").unwrap().to_vec())?,
    )?;

    let app = app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("generate", matches)) => {
            let album = {
                let album_dir = matches.value_of_os("album-dir").unwrap();
                let album = config::Album::load(&album_dir)?;
                view::Album::try_from(album)?
            };

            let output_dir = matches.value_of_os("output-dir").unwrap();
            album.render(&handlebars, &output_dir)?;
        }
        _ => unimplemented!(),
    };

    Ok(())
}
