# Waldo test

## Assumptions

Given two images, it does not matter if the image is in color or grayscale. That is, color is irrelevant for template matching.

The use of external libraries is allowed.

No need to use a GPU to achieve decent performance. Just the CPU should be enough.

## Usage of libraries vs naive implementation

There are two implementations. The python one is able to handle 3000*5000 pixels in arround a second (with my CPU)

With a more little image

Python using OpenCV:

`Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.32`

Rust naive:

`Elapsed (wall clock) time (h:mm:ss or m:ss): 0:35.09`

## Complexity

Both solutions (albeit optimizations) should be `O(P1*P2)` where `Pn` is the number of pixels of each image.
