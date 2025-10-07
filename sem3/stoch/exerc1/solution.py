import statistics as stat


def main():

    # 1.3
    # a)
    cash = {
        5, 10, 20,
        50, 100, 200, 500
    }
    print("Median: ", stat.median(cash))  # 50

    # b)
    cash.add(1000)
    print("Median mit hinzugefügtem 1000€-Schein: ", stat.median(cash))  # 75

if __name__ == "__main__":
    main()
