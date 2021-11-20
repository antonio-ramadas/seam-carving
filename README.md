# Seam Carving

This is a command-line program that prints to the terminal the Seam Carving algorithm (not just the final result, but its progress).

## Algorithm

Seam Carving is a content-aware image resizer.

Let's say we want to shrink an image. The most common technique is to simply shrink the image, but that will affect object's proportion. If we are not careful with the aspect ratio, the images may be severely distorted. Well, that's not ideal. What if we can remove the spacing between objects? This is the reasoning Seam Carving is built upon.

Let's say we want to resize from 1500x768 to 510x510 (34% width; 72% height):

| Shrunken | Original | Seam Carving |
|:-------:|:--------:|:----------:|
![](img/balloons-shrunken.jpg) | ![](img/balloons.jpg) | ![](img/balloons-final.png) |

As you can see, the balloons kept it proportion. We just brought them closer.

To know more about this algorithm, I highly recommend checking the blog post [_Content-Aware Image Resizing in JavaScript_](https://trekhleb.dev/blog/2021/content-aware-image-resizing-in-javascript/) by Oleksii Trekhleb. This implementation is largely based on it. Go check it out and then continue here. It even has an interactive version!

## Implementation

I'm using this project to learn and try out Rust. Be warned that if you try to run the project, it'll be slow. Not because Rust is slow, but I did not optimise the algorithm. Still, it works and acts as a good baseline for future improvements.

Here is a demo.

What you see:



What the algorithm sees:



If you want to run the project, here are the parameters:
```shell
$ cargo run -- --help
seam-carving 0.1.0
Seam Carving algorithm

USAGE:
    seam-carving [OPTIONS] --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --file <file>        Image location (no tilde expansion supported)
        --height <height>    Set height percentage (between 1 and 100) [default: 100]
        --width <width>      Set width percentage (between 1 and 100) [default: 100]
```

An example:
```shell
$ cargo run -- --file=img/balloons.jpg --width=34 --height=72
```

### Demo

The videos recorded above are from a slightly tweaked version to better convey what is going on. I'm going to describe how we can reenact them.

This project relies on [viuer](https://github.com/atanunq/viuer) to show images on the terminal. It is very good, but I wanted to show the images shrinking and for that I prefer having pixel precision control. It was necessary just one small tweak: [6270192bdce8e02f08a3d56f666e3da06b3435c9](https://github.com/antonio-ramadas/viuer/commit/6270192bdce8e02f08a3d56f666e3da06b3435c9). I considered opening an issue or even opening a Pull Request to upstream, but my use case is so specific that I dismissed. viuer exposes two configuration options for it, but they are based on character cells on iTerm and I spent more time tweaking those parameters – without much success – than to change the library for pixel precision. Another reason was that I learnt even more about Cargo from it! Anyway, this change is hardwired as a git submodule to avoid having to publish to remote (or required the user for few extra hops).

On `demo` branch I also made some small tweaks which you can see by [comparing branches](https://github.com/antonio-ramadas/seam-carving/compare/demo).

To be able to run the demo, checkout the `demo` branch and:
```shell
# Package our custom viuer version
$ cd viuer
$ cargo package
$ cd ..

# Build with optimisations (multiple times faster execution)
$ cargo build --release

# Run the optimised build
$ target/release/seam-carving --file=img/balloons.jpg --width=34 --height=72
```
