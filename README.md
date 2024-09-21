<div align="center">

<img src="./banner.webp">

</div>

# About
Tux-Icons is a library for getting icons from an icon pack in linux. You can search using the icon pack and the icon name or just simply getting it from the desktop file.  

# Install
To install do this in your project:
```sh
cargo add tux-icons
```
Or add it manually in cargo.toml:
```toml
[dependencies]
tux-icons = "0.2.1"
```

# Usage
Start by creating a IconFetcher instance. This will load with the default system icon pack:
```rust
let fetcher = IconFetcher::new();
```

## Get the icon path given it's name
You can get the icon path just by searching from it's name. For example firefox:
```rust
let path = fetcher.get_icon_pack_path("firefox");
```

## Get icon path from desktop file
You can also get the icon by giving the desktop file path
```rust
let path = fetcher.get_icon_path_from_desktop(&desktop_path);
```

## Use other icon pack
You can also change the icon pack to get an icon from that specific one in case you don't want the default.
```rust
fetcher.set_icon_pack("Papirus-Dark");
```

## Get target path
In case you want to get the target path from the icon file symlink instead of the symlink file
```rust
fetcher.set_return_target_path(true);
```

<a href="https://github.com/whiskers-apps/tux-icons/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=whiskers-apps/tux-icons" />
</a>