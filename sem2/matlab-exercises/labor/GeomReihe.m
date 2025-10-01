function s = GeomReihe(x,N)

    s = 0;
    for k = 0 : N
        s += x.^k;
    end
end

