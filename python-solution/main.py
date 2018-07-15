# Author: Blas Rabella Mariscal
import sys
from solver import find_template
from cv2 import error


def print_help():
    print(
        "Usage: cmd img1 img2.\nWill print the position of the pixel where the template is found."
    )


if __name__ == "__main__":

    if len(sys.argv) != 3:
        print_help()
        sys.exit(0)

    img1_path = sys.argv[1]
    img2_path = sys.argv[2]
    try:
        res = find_template(img1_path, img2_path)
    except error as e:
        print("There was some kind of problem reading the images")
        ## The real error should be checked and logged here.
        exit(1)
    if res is None:
        print("The template couldn't be found in the image")
    else:
        print("The top left pixel of the template can be found at: {}".format(
            res))
