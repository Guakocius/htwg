import numpy as np
import statistics
from scipy import stats


def main():
    # Aufgabe 1.11
    # Teilaufgabe 1.11.1
    java_repos = np.array([9, 11, 9, 21, 16, 13, 0])
    modal = statistics.mode(java_repos)
    mean = np.mean(java_repos)
    median = np.median(java_repos)
    ninety_quantile = np.quantile(java_repos, 0.9)
    estd = np.std(java_repos, ddof=1)
    q1 = np.quantile(java_repos, .25)
    q2 = np.quantile(java_repos, .5)
    q3 = np.quantile(java_repos, .75)
    iqr = stats.iqr(java_repos)  # Interquartilabstand -- Delta(q1,q3) = q3 - q1
    range = np.ptp(java_repos)  # max(java_repos) - min(java_repos) -- peak-to-peak

    print(
        "Modal: ", modal,
        "\nMittel: ", mean,
        "\nMedian: ", median,
        "\n90%-Quantil: ", ninety_quantile,
        "\nempirische Standardabweichung: ", estd,
        f"\n25%-Quartil: {q1}\n50%-Quartil: {q2}\n75%-Quartil: {q3}",
        "\nInterquartilabstand: ", iqr,
        "\nSpannweite: ", range
    )

    # 1.11.2
    # a)
    # I
    # Der Median und der Mittelwert sind nahezu identisch; das liegt und der Spanne
    # von 0 bis 21
    # II

    # b)
    # Java hat einen höheren Mittel und Median als Python und Ruby
    # Java IQR liegt zwischen Pythons und Rubys
    # c)
    # d)
    # rjp ~ 0.072
    # rjr ~ -0.815
    # rpr ~ 0.504


if __name__ == "__main__":
    main()
