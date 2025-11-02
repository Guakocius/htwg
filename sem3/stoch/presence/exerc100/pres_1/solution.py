from matplotlib.typing import ColorType
import numpy as np
import matplotlib.pyplot as plt
import statistics as stat
import pandas as pd

def main():
    # 1.1
    zimmer = [3,4,5,1,5,2,1,3,1,3,3,3,2,1,5]

    # 1.2
    rel = np.unique(zimmer)
    abs = np.bincount(zimmer)[1:]

    print("Relative Häufigkeiten:\n", rel, "\nAbsolute Häufigkeiten:\n", abs)

    # 1.3
    plt.hist(x=zimmer[1:], color="#800020", histtype="bar")
    plt.title("Absolute Häufigkeit")
    plt.ylabel("Anzahl Personen")
    plt.xlabel("Anzahl Zimmer")
    plt.savefig("absolute.png")

    # 1.4
    plt.close()
    plt.ecdf(zimmer)
    plt.title("Empirische Verteilungsfunktion")
    plt.xlabel("Anzahl Zimmer")
    plt.ylabel("Kumulierte relative Häufigkeit")
    plt.savefig("ecdf.png")
    plt.close()

    # 1.5
    # a)
    mean = np.mean(zimmer)
    # b)
    median = np.median(zimmer)
    # c)
    modal = stat.mode(zimmer)
    print(f"Mittel: {mean}\nMedian: {median}\nModalwert: {modal}")


if __name__ == "__main__":
    main()
