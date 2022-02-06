<h3 align="center">
	<br>
	termplay
	<br>
</h3>

<p align="center">terminal video player</p>

<p align="center">
	<a href="./LICENSE"><img src="https://img.shields.io/badge/license-GPL3-blue.svg"></a>
</p>

this video player is not very fast and requires heavy preprocessing, the frames are taken out of the video with ffmpeg then stored in a cache directory in `/tmp/termplay.cache`, next they are grayscaled and resized to the size of the terminal

once all that processing is done the individial pixels are divided by a number to determine an index of possible pixel characters, they are appended to a string that is the size of the terminal and printed

bad apple took over an hour to convert at 30 fps, in the future I'll be multithreading it and potentially swapping the processing it does to be less accurate but quicker

#### installation

it is recommended you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed

```
$ git clone git@github.com:NotCreative21/termplay.git
$ cd termplay
$ ./build.sh
$ termplay video.mp4 15 # 15 signifies fps, default is 10 if no argument is provided
```
