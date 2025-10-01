// Notenspiegel.java
package aufgabe5;
import java.util.NoSuchElementException;
import java.util.Scanner;

/**
 * Notenspiegel liest die Namen von F&auml;chern mit den zugeh&ouml;rigen Noten
 * in eine verkettete Liste ein und gibt dann einen Notenspiegel aus.
 * @version 08.1.2025
 * @author Alexander Engelhardt
 */
public final class Notenspiegel {
    private Notenspiegel() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    @SuppressWarnings("UnnecessaryContinue")
    public static void main(String[] args) {
        FachnotenListe liste = new FachnotenListe(); // leere Liste

        //--------------------------------------------- Notenspiegel einlesen
        System.err.printf(
            "Faecher mit Noten zwischen %d und %d eingeben "
            + "(Ende mit Strg-D/Strg-Z):%n",
            Note.BESTE.intValue(), Note.SCHLECHTESTE.intValue());

        while (EINGABE.hasNext()) {
            try {
                // Fach und Note einlesen

                Note note;
                String fach = EINGABE.next();


                if (EINGABE.hasNextInt()) {
                    note = Note.valueOf(EINGABE.nextInt());
                    Fachnote fachnote = new Fachnote(fach, note);
                    liste.insert(fachnote);
                } else {
                    note = Note.valueOf(EINGABE.next());
                    Fachnote fachnote = new Fachnote(fach, note);
                    liste.insert(fachnote);
                }

                // neue Fachnote in Notenliste eintragen

            } catch (IllegalArgumentException x) {
                System.err.printf("Eingabefehler: %s%n", x.getMessage());
                continue;
            } catch (NoSuchElementException x) {
                System.err.println("Fach ohne Note ignoriert!");
                break;
            }
        }

        // Notenspiegel ausgeben

        System.out.println("NOTENSPIEGEL");

        FachnotenListe.Iterator i = liste.new Iterator();
        int maxLength = 0;
        while (i.hasNext()) {
            Fachnote fachnote = i.next();
            String fachname = fachnote.fach();

            if (fachname.length() > maxLength) {
                maxLength = fachname.length();
            }
        }
        i = liste.new Iterator();
        while (i.hasNext()) {
            Fachnote fachnote = i.next();
            String fachname = fachnote.fach();

            if (fachname.length() < maxLength) {
                fachname = String.format("%-" + maxLength + "s", fachname);
            }
            System.out.println(fachname + "  "
                + fachnote.note().toString() + "  "
                + passState(fachnote.note()));
        }


    } // main

    /**
     *
     * @param note
     * @return "mit Bestnote bestanden" (if note is 1.0)
     * or "nicht bestanden" (if note > 4)
     * or "bestanden" (if note <= 4 && note != 1)
     */
    private static String passState(Note note) {

        if (Note.BESTE.equals(note)) {
            return "mit Bestnote bestanden";
        } else if (!note.istBestanden()) {
            return "nicht bestanden";
        } else if (note.istBestanden()) {
            return "bestanden";
        }
        return "";
    }
}

