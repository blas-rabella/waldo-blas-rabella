# Author: Blas Rabella Mariscal
import cv2

THRESHOLD = 0.8


def find_template(path1, path2):
    # Read the images, pass them to greyscale and sort them
    img1 = cv2.imread(path1)
    img1_gray = cv2.cvtColor(img1, cv2.COLOR_BGR2GRAY)
    w1, h1 = img1_gray.shape[::-1]

    img2 = cv2.imread(path2)
    img2_gray = cv2.cvtColor(img2, cv2.COLOR_BGR2GRAY)
    w2, h2 = img2_gray.shape[::-1]

    template = None
    big = None
    if w2 > w1 or h2 > h1:  # Second one is bigger in one of the axis
        template = img1_gray
        big = img2_gray
    else:
        template = img2_gray
        big = img1_gray

    res = cv2.matchTemplate(big, template, cv2.TM_CCOEFF_NORMED)
    minVal, maxVal, minLoc, maxLoc = cv2.minMaxLoc(res)

    if maxVal < THRESHOLD:  # The value is not over the threshold we "suppose" that the template is not there.
        return None

    return maxLoc
