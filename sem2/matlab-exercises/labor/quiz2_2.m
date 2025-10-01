clear all
close all

I = [0,3300];

x = linspace(I(1),I(2));

alpha = 3*10.^(-2);
beta = 8.9*(10.^(-7));
gamma = 1.5*10.^3;

bb = AnkommendeBesucher(alpha,beta,gamma,x);
A = floor(cumsum(bb));
aa = round(sum(A));
k = floor(log(A + 1) + 0.5);

hold on
%plot(x,bb,"r-")
%plot(x,A,"b-")
%plot(x,A)
%plot(x,k,"g-")
grid on
set(gca, "XAxisLocation", "origin", "YAxisLocation", "origin")

