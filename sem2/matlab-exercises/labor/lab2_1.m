clear all
close all

I = [-1.5,0.9];
DIM = 100;
xx = linspace(I(1),I(2),DIM);

f = @(x) 1./(1-x);
g = @(x) x.^2;

%gg = GeomReihe(xx,50)

hold on
plot(xx,f(xx),"b-")
plot(xx,f(g(xx)), "r-")
%plot(xx,gg,"r-")
%ylim([0,30])
%daspect([1 1 1])
grid on
set(gca, "XAxisLocation", "origin", "YAxisLocation", "origin")

waitfor(gcf);
