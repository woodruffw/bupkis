bupkis
======

![license](https://raster.shields.io/badge/license-MIT%20with%20restrictions-green.png)
[![Build Status](https://img.shields.io/github/workflow/status/woodruffw/bupkis/CI/main)](https://github.com/woodruffw/bupkis/actions?query=workflow%3ACI)

A small static album generator.

[Demo album](https://yossarian.net/media/trips/phoebe-s-graduation-2021/).

## Features

* Trivial installation (pure Rust, no system dependencies)
* 100% static (no server-side rendering)
* Optional progressive enhancement with JavaScript (arrow keys, swipes for navigation)
* Optional per-album and per-image descriptions (rendered Markdown)

## {Anti,mis}-features

* A central index (each album is its own set of generated pages)
* Intelligent thumbnailing (no content-aware cropping)
* Configurable style/appearance (sorry)
* Secret albums (use basic auth) or EXIF stripping (do it beforehand)

## Installation and use

Get it with `cargo`:

```bash
$ cargo install bupkis
```

Point it at an album directory:

```bash
$ bupkis generate my-album/ /var/www/my-site
$ firefox /var/www/my-site/my-album/index.html
```

## Album layout and configuration

`bupkis` loads album directories that are structured like this:

```
my-album/
    img001.jpg
    img002.png
    img003.jpg
    album.yml
```

JPEG and PNG are the only officially supported image formats.

`album.yml` is a YAML-formatted configuration file that configures the album's
order and description/individual photo descriptions:

```yaml
# slugified as my-album
title: my album

# album description, rendered in the album's index
desc: |
  this is the album's description. it's *rendered* as **markdown**.

# whether or not to generate JavaScript for this album
javascript: true

# each photo of the album, in presentation order
photos:
  - name: img001.jpg
    desc: |
      individual images have descriptions too, which are also **markdown**.
  - name: img002.png
    desc: |
      image descriptions are optional, so this next one doesn't have one.
  - name: img003.jpg
```
