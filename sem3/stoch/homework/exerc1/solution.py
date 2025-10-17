import numpy as np

def main():

    # 1.3
    # a)
    cash = [
        5, 10, 20,
        50, 100, 200, 500
    ]
    print("Median: ", np.median(cash))  # 50
    print("Mean: ", np.mean(cash))

    # b)
    cash.append(1000)
    print("Median mit hinzugefügtem 1000€-Schein: ", np.median(cash))  # 75

if __name__ == "__main__":
    main()
