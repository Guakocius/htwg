// Klausurergebnis.java
package aufgabe4;
//import aufgabe4.schweiz.Noten;
import java.util.Locale;
import java.util.Scanner;

/**
 * Klausurergebnis erstellt eine Notenstatistik.
 * <p>
 * Das Programm liest Noten als Strings ein und bestimmt die beste und
 * die schlechteste Note, den Durchschnitt der Bestandenen sowie
 * die Durchfallquote in Prozent.
 * Das Programm ber&uuml;cksichtigt dabei nur die laut Noten.istZulaessig
 * erlaubten Noten. Andere Noten werden unter Ausgabe einer Warnung ignoriert.
 * Welche Noten besser und schlechter sind, welche als bestanden oder
 * durchgefallen gelten und wie die String-Darstellung der Noten aussieht,
 * wird mit Methoden der Klasse Noten bestimmt.
 * </p>
 * Das Programm gibt als Klausurergebnis folgende Werte aus:
 * <ul>
 * <li>die Anzahl der ber&uuml;cksichtigten Noten</li>
 * <li>die Anzahl der Bestandenen</li>
 * <li>die beste Note</li>
 * <li>die schlechteste Note</li>
 * <li>den Durchschnitt der Bestandenen</li>
 * <li>die Durchfallquote</li>
 * </ul>
 *
 * @author Alexander Engelhardt
 * @version 11.12.2024
 */
public final class Klausurergebnis {
    private Klausurergebnis() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     *
     * @param args wird nicht verwendet.
     */
    public static void main(String[] args) {
        Locale.setDefault(Locale.GERMAN);

        int calcGrades = 0;
        int passedGrades = 0;
        double bestGrade = Noten.SCHLECHTESTE;
        double worstGrade = Noten.BESTE;
        double avgPass;
        double failureRate;
        int failureGrades = 0;
        double sumPassed = 0;
        final int percentCalc = 100;

        // Noten einlesen
        System.out.println("Noten Ganze,Zehntel oder Ganze.Zehntel eingeben "
                + "(Ende mit Strg-D/Strg-Z):");

        while (EINGABE.hasNext()) {
            String note = EINGABE.next();

            // Eingabe pruefen

            if (Noten.istZulaessig(note)
                    && Noten.istBestanden(Noten.toDouble(note))) {

                calcGrades++;
                passedGrades++;
                sumPassed += Noten.toDouble(note);

            } else if (Noten.istZulaessig(note)
                    && !Noten.istBestanden(Noten.toDouble(note))) {

                calcGrades++;
                failureGrades++;
            } else if (!Noten.istZulaessig(note)) {

                System.out.printf("Unzulaessige Note %s "
                        + "wird ignoriert!%n", note);
                continue;

            } else {
                continue;
            }

            // Note erfassen

            if (Noten.istZulaessig(note)
                    && Noten.istBestanden(Noten.toDouble(note))) {
                bestGrade = Noten.bessere(Noten.toDouble(note), bestGrade);
            } else if (Noten.istZulaessig(note)
                    && !Noten.istBestanden(Noten.toDouble(note))) {
                worstGrade = Noten.schlechtere(Noten.toDouble(note),
                        worstGrade);
            }

        }

        // Notenstatistik ausgeben

        avgPass = sumPassed / passedGrades;
        failureRate = ((double) failureGrades / calcGrades) * percentCalc;

        System.out.printf("%nAnzahl beruecksichtigter Noten: "
                + "%s%n", calcGrades);
        System.out.printf("Anzahl Bestandene: %s%n", passedGrades);

        if (calcGrades >= 1) {
            System.out.printf("Beste Note: %s%n", Noten.toString(bestGrade));
            System.out.printf("Schlechteste Note: %s%n",
                    Noten.toString(worstGrade));
            System.out.printf("Durchschnitt Bestandene: %s %n",
                    Noten.toString(avgPass));
            System.out.printf("Durchfallquote: %.1f%% %n",
                    failureRate);
        }
    }
}