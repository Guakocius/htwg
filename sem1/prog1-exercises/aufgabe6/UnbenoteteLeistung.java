package aufgabe6;

/**
 * UnbenoteteLeistung, eine Unterklasse von Leistung, prüft,
 * ob Noten schon benotet und/oder bestanden wurde.
 */
public final class UnbenoteteLeistung extends Leistung {
    private final boolean istBestanden;

    /**
     * Konstruktor für UnbenoteteLeistung, fach: super type,
     * istBestanden: Lokalvariabel.
     * @param fach
     * @param istBestanden
     */
    public UnbenoteteLeistung(String fach, boolean istBestanden) {
        super(fach);
        this.istBestanden = istBestanden;
    }

    @Override
    public boolean istBenotet() {
        return false;
    }

    @Override
    public boolean istBestanden() {
        return istBestanden;
    }
}
