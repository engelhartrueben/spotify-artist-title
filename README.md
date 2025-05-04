<div align="center">

# spotify-artist-title
**because why not**\
*for I am too lazy to switch windows*

[Why](#why) •
[PolyBar Integration](#polybar-integration)

</div>

## Why
Because I wanted to see what was playing at any given moment without having to switch to Spotify.

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


