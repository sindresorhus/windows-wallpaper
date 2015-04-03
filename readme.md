# win-wallpaper

> Get or set the desktop wallpaper on Windows


## Install

### [npm](https://github.com/sindresorhus/node-win-wallpaper#cli)

```
$ npm install --global win-wallpaper
```

### Manually

[Download the binary](https://github.com/sindresorhus/win-wallpaper/releases/latest) and put it somewhere in your `%path%`.


## Usage

```sh
# set
wallpaper unicorn.jpg

# get
wallpaper
> /Users/sindresorhus/unicorn.jpg
```


## Dev

To compile you'll need a C compiler like `gcc`, which you can get by installing [`MinGW`](http://www.mingw.org/).

```
$ gcc wallpaper.c -o wallpaper.exe
```


## Related

- [`node-win-wallpaper`](https://github.com/sindresorhus/node-win-wallpaper) - Node wrapper.
- [`wallpaper`](https://github.com/sindresorhus/wallpaper) - Get or set the desktop wallpaper.


## License

MIT © [Sindre Sorhus](http://sindresorhus.com)
