# fs-scan
Scan directory and sub directories to display file layout from the size standpoint.

[![asciicast](https://asciinema.org/a/6kVXn9wv2E97VLIB2g7Yt05Ii.svg)](https://asciinema.org/a/6kVXn9wv2E97VLIB2g7Yt05Ii?autoplay=1)

The package can optionally take a parameter which will be used as base directory.
Otherwise the current directory is used.

In the current path:

```sh
$ ~/bin/fs-scan 
Files -> 642K (642896)
Directories -> 62K (62645)
Less than 4K -> 377K (377699)
Between 4K and 16K -> 168K (168241)
...
```

To a specify the path:

```sh
$ ~/bin/fs-scan bench-fs/
The path is bench-fs/
Files -> 642K (642896)
Directories -> 62K (62645)
Less than 4K -> 377K (377699)
Between 4K and 16K -> 168K (168241)
...
```
