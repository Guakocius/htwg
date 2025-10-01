clear all
close all

I = [-0.5,0.9];
DIM = 100;
xx = linspace(I(1),I(2),DIM);
ww = linspace(0,10,DIM);

f = @(x) 1./(1-x)
xi = linspace(f(I(1)),f(I(2)),DIM);
ff = @(x) (x-1)./x;

yy = f(xx);

hold on
plot(xx,yy,"b-") % Funktion
plot(yy,xx,"g-","LineWidth",2) % Umkehrung
plot(xi,ff(xi),"m-")
plot(ww,ww,"k-") % Winkelhalbierende; Identität
daspect([1 1 1])
grid on
set(gca, "XAxisLocation", "origin", "YAxisLocation", "origin")
ylim([-1,10])

waitfor(gcf);
