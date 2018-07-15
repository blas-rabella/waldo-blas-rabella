# Python solution

This is a very simple solution written in python

## Requirements

To run this you will need to have `OpenCV`.

On a current Ubuntu this can be installed via `apt-get install libopencv-dev`. It has been tested with version `3.1`.

## How to set up the environment

The recommendation would be to use [virtualenv](https://virtualenv.pypa.io/en/stable/) or similar solution to maintain separated packages in your environments.

Once you have a virtual environment (or not, depending on your choice), install the dependencies in `requirements.txt` using `pip install -r requirements.txt`.

## How to run it

With your virtual environment activated run `python main.py img1 img2` and you should be able to see the output.

The program will tell you if there was any error, if the cropped image couldn't be found or where it was found. The cropped image does accept some differences with the original region as it is a metric, not exact matching.

## Why use OpenCV

Well, the description doesn't say one can not use external libraries. And as I read the text I thought it would be a good fit.

Usually, I prefer to use robust libraries that have a big developer base because bugs and performance are "usually" better than what a single developer can roll out by himself.

## Complexity

Let's say that `N` is the number of pixels of the bigger image and `n` of the little one (if they are the same then `N == n`)
As the naive implementation is to compare every pixel of `N` with `n` that yields a `O(n*N)` complexity. Which according to the [documentation](https://docs.opencv.org/3.1.0/df/dfb/group__imgproc__object.html#ga586ebfb0a7fb604b35a23d85391329be) seems to be close to what the implementation does.

Then there are things like the conversion which are `O(N)` and the `minMaxLoc` function that as it is a search on `N` should not be worst than `O(N)`

## Parallelism and Concurrency

There is no parallelism or concurrency by my side.

Usually, for this to be used it would be exposed through a REST API which means that also the images (at least the big one) would have to be read either on the request or from some remote storage place. Is in that reading of data where I would apply concurrency (`asyncio` or `gevent`) and probably parallel processing of independent requests this one using `multiprocessing`.
