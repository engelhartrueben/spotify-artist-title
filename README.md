<div align="center">

# spotify-artist-title
**because why not**\
*for I am too lazy to switch windows*

[Why](#why) •
[Example](#example) •
[Note](#Note)
[PolyBar Integration](#polybar-integration)

</div>


## Why
Because I wanted to see what was playing at any given moment without having to switch to Spotify.


## Example
If Spotify **is** open:
```terminal
$ <path/to/spotify-artist-title/binary>
<TITLE> - by <ARTIST>
```

If spotify **is not** open, nothing happens:
```terminal
$ <path/to/spotify-artist-title/binary>

```

This is **by design** to allow better integration with PolyBar.


## Note
Compile with `--release` to avoid debug logs. This is especially important when integrating with PolyBar as it **will** display the error in the bar.


## PolyBar Integration
Example: 
```
modules-right =  ... music ... 
. . . 
[module/music]
type = custom/script
exec = /path/to/spotify-artist-title/binary
interval = 2
```


