# Frameport

Frameport is a local-first media import and management app for camera footage.

It is designed to automatically import videos and photos from connected cameras, organise them on disk, and provide a clean interface for reviewing media locally. The long-term goal is to support a wide range of devices including GoPro, DJI, Insta360, drones, phones, and normal cameras.

Frameport is **source-available software**. The source code is public for transparency, learning, personal use, and non-commercial builds, but commercial use requires permission.

## Features

Current/planned features:

* Detect connected cameras and removable storage devices
* Import photos and videos to a chosen local folder
* Organise media by date, device, and file type
* Review imported media in a desktop library UI
* Avoid vendor lock-in to a single camera ecosystem
* Work across Windows, Linux, and macOS

## Goals

Frameport aims to be:

* **Local-first** — media stays on your computer
* **Cross-platform** — Windows, Linux, and macOS support
* **Device-agnostic** — not limited to a specific camera brand or ecosystem
* **Simple** — import, organise, review
* **Transparent** — source code is available to inspect

## Non-goals

Frameport is not intended to be:

* A cloud photo backup service
* A full video editor
* A replacement for professional asset management tools
* A server-first alternative to Immich

## Tech stack

Planned stack:

* **Tauri** — desktop application shell
* **Rust** — backend, filesystem access, media import logic
* **Svelte** — frontend UI
* **SQLite** — local media database

## Development status

Frameport is currently in early development.

Expect breaking changes while the project structure, import system, and media database are being designed.

## Licence

Frameport is licensed under the **PolyForm Noncommercial License 1.0.0**.

You may use, study, modify, and build the software for personal and other non-commercial purposes.

Commercial use is not permitted without a separate commercial licence.

For commercial licensing, contact the project maintainer.

See [`LICENSE`](./LICENSE.md) for the full licence text.

## Contributing

Contributions are welcome, but this project is not open-source software.

By contributing, you agree that your contribution may be included in Frameport under the project licence.

Before large changes, open an issue to discuss the idea first.

## Disclaimer

Frameport is provided as-is, without warranty. Always keep backups of important media files.
