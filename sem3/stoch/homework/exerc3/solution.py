import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def main():
    df = pd.read_csv("./computer_prices_all.csv")
        
    #dev = df["device_type"].to_numpy()
    os = df["os"].to_numpy()
    #brands = df["brand"].to_numpy()
    #form_factor = df["form_factor"].to_numpy()
    
    y = "Number of Devices"
    
    #bins = ["Windows", "macOS", "Linux", "ChromeOS"]
    # Operating Systems - Popularity
    plt.hist(os)
    plt.title("Popularity of Operating Systems")
    plt.xlabel("Operating System")
    plt.ylabel(y)
    #plt.show()
    plt.savefig("os_pop.jpg")

    # Brands on Desktop / Laptop
    #df.groupby(["device_type", "brand"]).size().unstack().plot(kind="bar", stacked=True, figsize=(10,6))
    df.groupby(["device_type", "brand"]).size().unstack().plot(kind="bar", stacked=False, figsize=(10,6))    
    plt.title("Brand Distribution Across Device Types")
    plt.xlabel("Device Type")
    plt.ylabel(y)
    #plt.show()
    #plt.savefig("brands_dev_stacked.jpg")
    plt.savefig("brands_dev_non_stacked.jpg")

    
    df.groupby(["form_factor", "brand"]).size().unstack().plot(kind="bar", stacked=True, figsize=(10,6))
    plt.title("Form Factor Compilation of Brands")
    plt.xlabel("Brands")
    plt.ylabel(y)
    #plt.show()
    plt.savefig("form_factor_cum_brands.jpg")
    

if __name__ == "__main__":
    main()
