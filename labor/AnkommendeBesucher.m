function b = AnkommendeBesucher(a,b2,c,t)
    b = 0;
    b = b + a*exp(-b2*(t-c).^2);
end