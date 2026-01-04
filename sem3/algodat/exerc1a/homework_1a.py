from sympy import lambdify, symbols, O, log, ln, sqrt, factorial
import numpy as np
import matplotlib.pyplot as plt

def main():
    n = symbols("n", positive=True)

    log_n = log(n)
    n_sqr = n ** 2
    n_exp = n ** 2.5
    n_sqr_lg = n_sqr * log_n
    n_ln_ln = ln(ln(n))
    log_n_cu = (log_n) ** 3
    SEVEN = 7
    sq = sqrt(n)
    n_sqr_log_cu = n_sqr + log_n_cu
    n_fact = factorial(n)
    n_pow_n = n ** n
    two_pow_n = 2 ** n
    n_5th_root = n ** (1/5)
    n_sqr_num = 176 * n_sqr

    big_o_1 = O(n_exp, n)
    big_o_2 = O(n_sqr_lg, n)
    big_o_3 = O(n_ln_ln, n)
    big_o_4 = O(log_n, n)
    big_o_5 = O(log_n_cu, n)
    big_o_6 = O(SEVEN)
    big_o_7 = O(sq, n)
    big_o_8 = O(n_sqr_log_cu, n)
    big_o_9 = O(n_fact, n)
    big_o_10 = O(n_pow_n, n)
    big_o_11 = O(two_pow_n, n)
    big_o_12 = O(n_5th_root, n)
    big_o_13 = O(n_sqr_num, n)
    big_os = [big_o_1, big_o_2, big_o_3, big_o_4, big_o_5, big_o_6, big_o_7, big_o_8, big_o_9, big_o_10, big_o_11, big_o_12, big_o_13]

    exprs = [o.expr for o in big_os]

    print("Big O notations: ")
    [print(i) for i in big_os]
    print("Exprs: ")
    [print(i) for i in exprs]


    funcs = [lambdify(n, f, 'numpy') for f in exprs]

    x = np.linspace(1, 20, 200)
    eval = [f(20) for f in funcs]
    sorted = np.argsort(eval)
    funcs_sorted = [funcs[i] for i in sorted]
    labels_sorted = [str(big_os[i]) for i in sorted]

    plt.figure(figsize=(10, 0))
    for f, label in zip(funcs_sorted, labels_sorted):
        plt.plot(x, f(x), label=f"O({label})")

    plt.yscale('log')
    plt.xlabel("n")
    plt.ylabel("Growth")
    plt.title("Asymptotic Growth of the different complexity classes")
    plt.legend()
    plt.grid(True)
    plt.show()


if __name__ == "__main__":
    main()
