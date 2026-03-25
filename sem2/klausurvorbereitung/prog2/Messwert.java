import java.util.List;
import java.util.LinkedList;
import java.util.function.*;
import java.util.stream.Collectors;
import java.util.Comparator;
import java.util.Map;
import java.util.TreeMap;

public record Messwert(
        String typ,
        String plz,
        double wert) {
    public static void main(String[] args) {
        List<Messwert> mLst = new LinkedList<>();
        mLst.add(new Messwert("CO2", "78364", 418.1));
        mLst.add(new Messwert("Temp", "78464", 21.5));
        mLst.add(new Messwert("CO2", "78362", 412.2));
        mLst.add(new Messwert("CO2", "78462", 410.8));
        mLst.add(new Messwert("CO2", "78464", 419.4));
        mLst.add(new Messwert("Temp", "78567", 22.3));

        BiFunction<Messwert, Double, Messwert> incr = (Messwert m, Double p) -> new Messwert(m.typ(), m.plz(),
                m.wert() * p);

        Messwert m = new Messwert("Temp", "78464", 20.0);
        System.out.println(incr.apply(m, 1.5));

        mLst.replaceAll(n -> incr.apply(n, 1.1));
        mLst.forEach(n -> System.out.println(n));

        BiFunction<Integer, Integer, Boolean> allTemp = (a, b) -> mLst.stream()
                .filter(w -> w.typ().equals("Temp")).allMatch(n -> n.wert() >= a && n.wert() <= b);
        System.out.println(allTemp.apply(20, 30));

        int a = 20, b = 30;

        System.out
                .println(mLst.stream().filter(n -> n.typ().equals("Temp")).peek(n -> System.out.println("Prüfe: " + n))
                        .allMatch(n -> n.wert() >= a && n.wert() <= b));

        mLst.stream().filter(n -> n.typ().equals("CO2")).max(Comparator.comparingDouble(Messwert::wert))
                .map(Messwert::plz).ifPresent(System.out::println);

        System.out.println();
        System.out.println(mLst.stream().map(n -> n.plz).reduce("plz", (s1, s2) -> s1 + ":" + s2));

        Map<String, List<String>> map = mLst.stream().collect(Collectors.groupingBy(Messwert::typ, TreeMap::new,
                Collectors.mapping(Messwert::plz, Collectors.toList())));

        System.out.println(map);

    }
}
