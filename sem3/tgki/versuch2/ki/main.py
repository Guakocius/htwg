# -*- coding: utf-8 -*-
import numpy as np
import cv2
import matplotlib.pyplot as plt
from skimage.color import rgb2gray

#def main():
    
cap = cv2.VideoCapture(0)
print(cap)

#while (True):
ret, frame = cap.read()
# print("red")
grayscale = rgb2gray(frame)
fig, ax = plt.subplots(figsize=(8, 4))
ax.imshow(grayscale, cmap=plt.cm.gray)
# cv2.imshow(grayscale, cmap=plt.cm.gray)
ax.set_title("Grayscale")
plt.show()
# print("after plot")
#cv2.imwrite("./versuch2-1_2.png", frame)
img = cv2.imread("./versuch2-1_2.png")
b, l = len(img), len(img[0])
px = b * l
gray_border_px = px / 5
cropped_imgs = []
for i in range(0, 5):
    cropped_imgs.append(img[0:480][i*128:640])
print("Cropped imgs:", cropped_imgs)
mean = np.mean(cropped_imgs)
std = np.std(cropped_imgs)
print(f"Mean: {mean}\nStd: {std}")
#means = np.mean()
print("Image Matrix:", img)
print("Breite img", b) # 480
print("Länge img", l) # 640

cap.release()
cv2.destroyAllWindows()


#if __name__ == "__main__":
    #main()