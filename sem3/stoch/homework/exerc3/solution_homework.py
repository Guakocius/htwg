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


if __name__ == "__main__":
    main()
