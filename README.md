<div align="center">

<img src="./banner.svg">

</div>

# About
Tux-Icons is a library for getting icons from an icon pack in linux. You can search using the icon pack and the icon name or just simply getting it from the desktop file.  

# Library

## Install
To install do this in your project:
```sh
cargo add tux-icons
```
Or add it manually in cargo.toml:
```toml
[dependencies]
tux-icons = "0.4.4"
```

## Usage
Start by creating a IconFetcher instance. This will load with the default system icon pack:
```rust
let fetcher = IconFetcher::new();
```

### Get the icon path given it's name
You can get the icon path just by searching from it's name. For example firefox:
```rust
let path = fetcher.get_icon_pack_path("firefox");
```

### Get icon path from desktop file
You can also get the icon by giving the desktop file path
```rust
let path = fetcher.get_icon_path_from_desktop(&desktop_path);
```

### Use other icon pack
You can also change the icon pack to get an icon from that specific one in case you don't want the default.
```rust
fetcher.set_icon_pack("Papirus-Dark");
```

### Get target path
In case you want to get the target path from the icon file symlink instead of the symlink file
```rust
fetcher.set_return_target_path(true);
```

# Cli
## Install
To install it you can run 
```bash
cargo install 
```

Or you can install by download the binary in the releases

<a href="https://github.com/Whiskers-Apps/tux-icons/releases">
<img src="./download-button.svg" width="200">
</a>

## Usage
To use this it's as simple as running `search-icon` and icon name and `from-desktop` with .desktop file.

You can also run `tux-icons --help` for more configurations like icon pack and return target path in case it's a symlink (it's not on by default due to performance reasons).

# Contributors
<a href="https://github.com/whiskers-apps/tux-icons/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=whiskers-apps/tux-icons" />
</a>
