<div align="center">

# spotify-artist-title
**because why not**\
*for I am too lazy to switch windows*

[Why](#why) •
[Example](#example) •
[Note](#Note) •
[Polybar Integration](#polybar-integration)

</div>


## Why
Because I wanted to see what was playing at any given moment without having to switch to Spotify.

Great with [Polybar](https://github.com/polybar/polybar).

## Example

### Compile
```terminal
$ cd <path/to/spotify-artist-title/>
$ cargo build --release
```

### Run
If Spotify **is** open:
```terminal
$ <path/to/spotify-artist-title/>release/spotify-artist-title
<TITLE> - by <ARTIST>
```

If spotify **is not** open, nothing happens:
```terminal
$ <path/to/spotify-artist-title/>release/spotify-artist-title

```

This is **by design** to allow better integration with Polybar.


## Note
Compile with `--release` to avoid debug logs. This is especially important when integrating with Polybar as it **will** display the error in the bar.


## Polybar Integration
Example: 
```
modules-right =  ... music ... 
. . . 
[module/music]
type = custom/script
exec = /path/to/spotify-artist-title/binary
interval = 2
```


