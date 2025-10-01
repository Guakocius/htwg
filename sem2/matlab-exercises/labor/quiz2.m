clear all
close all

I = [0,4200];

x = linspace(I(1),I(2));

alpha = 3*10.^(-2);
beta = 8.9*(10.^(-7));
gamma = 1.5*10.^3;

bb = AnkommendeBesucher(alpha,beta,gamma,x);
A = floor(cumsum(bb));
aa = (sum(A))

hold on
%plot(x,bb,"r-")
%plot(x,A,"g-")
%title("Ankommende Personen je Zeit")
%xlabel("t")
%ylabel("b(t)")
grid on
set(gca, "XAxisLocation", "origin", "YAxisLocation", "origin")

