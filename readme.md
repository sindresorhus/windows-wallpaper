# win-wallpaper

> Manage the desktop wallpaper on Windows

*Requires Windows 10 or later.*

## Install

[Download](https://github.com/sindresorhus/win-wallpaper/releases/latest) the binary and put it somewhere in your [`%path%`](http://stackoverflow.com/a/28778358/64949).

## Usage

```sh
# Set
wallpaper set unicorn.jpg

# Set with scaling options
wallpaper set unicorn.jpg --scale [center | stretch | tile |  span | fit | fill]

# Get
wallpaper get
> /Users/sindresorhus/unicorn.jpg
```

## Build

Inside the project's folder:

```
cargo build --release
```

## Related

- [wallpaper](https://github.com/sindresorhus/wallpaper) - Get or set the desktop wallpaper.
