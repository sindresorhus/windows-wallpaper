# win-wallpaper

> Manage the desktop wallpaper on Windows

*Requires Windows 10 or later.*

## Install

[Download](https://github.com/sindresorhus/win-wallpaper/releases/latest) the binary and put it somewhere in your [`%path%`](http://stackoverflow.com/a/28778358/64949).

## Usage

```sh
# Set
wallpaper unicorn.jpg

# Set with scaling options
wallpaper unicorn.jpg --scale [center | stretch | tile |  span | max | crop-to-fit | keep-aspect-ratio]

# Get
wallpaper
> /Users/sindresorhus/unicorn.jpg
```

## Dependencies

Install [`MinGW-w64`](http://sourceforge.net/projects/mingw-w64) from their official repository.

**or**

```sh
choco install mingw
```

## Build

Inside the project's folder:

```
.\build
```

## Related

- [wallpaper](https://github.com/sindresorhus/wallpaper) - Get or set the desktop wallpaper.
