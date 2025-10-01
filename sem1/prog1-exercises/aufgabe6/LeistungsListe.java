package aufgabe6;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * LeistungsListe kapselt eine Liste von Leistungen.
 * @author Alexander Engelhardt
 * @version 21.01.2025
 */
public final class LeistungsListe implements Iterable<Leistung> {
    private Element head;

    /**
     * Legt eine leere Liste an.
     */
    public LeistungsListe() {
        this.head = null;
    }

    /**
     * F&uuml;gt eine Zahl am Listenanfang ein.
     * @param l die einzuf&uuml;gende Zahl
     * @return die Liste
     */
    public LeistungsListe insert(Leistung l) {
        this.head = new Element(this.head, l);
        return this;
    }

    /**
     * Element speichert eine einzelne Zahl als Teil einer Liste.
     * Beispiel f&uuml;r eine statisch eingebettete Klasse.
     */
    private static final class Element {
        private final Element next;
        private final Leistung l;

        private Element(Element e, Leistung l) {
            this.next = e;
            this.l = l;
        }
    }

    @Override
    public Iterator<Leistung> iterator() {
        // Beispiel f&uuml;r eine anonyme lokale Klasse
        return new Iterator<Leistung>() {
            private Element current = LeistungsListe.this.head;

            @Override
            public boolean hasNext() {
                return this.current != null;
            }

            @Override
            public Leistung next() {
                if (this.current == null) {
                    throw new NoSuchElementException();
                }

                Element e = this.current;
                this.current = this.current.next;
                return e.l;
            }
        };
    }
}