// Notenstatistik.java

package aufgabe3;

import java.util.Locale;
import java.util.Scanner;

/**
 * erstellt eine Notenstatistik.
 * <p>
 * Das Programm erwartet Pr&uuml;fungsnoten im Format
 * <code>Ganze,Zehntel</code> oder <code>Ganze.Zehntel</code>,
 * wobei <code>Ganze</code> und <code>Zehntel</code> nur aus
 * je einer Dezimalziffer bestehen d&uuml;rfen.
 * Andere Eingaben werden wegen Formatfehler ignoriert.
 * </p>
 * <p>
 * Das Programm gibt die folgende Statistik aus:
 * </p>
 * <ul>
 * <li>die Anzahl der ber&uuml;cksichtigten Noten</li>
 * <li>die Anzahl der Bestandenen</li>
 * <li>die beste Note</li>
 * <li>die schlechteste Note</li>
 * <li>den Durchschnitt der Bestandenen</li>
 * <li>die Durchfallquote in Prozent</li>
 * </ul>
 * <p>
 * Es werden in der Statistik nur die nach HTWG-Pr&uuml;fungsordnung
 * zul&auml;ssigen Noten (1,0 1,3 1,7 2,0 2,3 2,7 3,0 3,3 3,7 4,0 5,0)
 * ber&uuml;cksichtigt.
 * Andere Eingaben werden wegen falscher Vorkommastelle oder
 * falscher Nachkommastelle ignoriert.
 * Alle Noten bis 4,0 gelten als bestanden, nur die 5,0 als durchgefallen.
 * </p>
 *
 * @author Alexander Engelhardt
 * @version 10.12.2024
 */
public final class Notenstatistik {
    private Notenstatistik() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(String[] args) {
        Locale.setDefault(Locale.GERMANY);

        //--------------------------------------------------- Noten einlesen
        System.out.println("Noten Ganze,Zehntel oder Ganze.Zehntel eingeben "
                           + "(Ende mit Strg-D/Strg-Z):");

        boolean isVorkommastelle = false;
        boolean isNachkommaStelle = false;
        int calcGrades = 0;
        int passedGrades = 0;
        double bestGrade = 5;
        double worstGrade = 1;
        double avgPass = 0;
        double failureRate = 0;
        int failureGrades = 0;
        double noteDouble = 0;
        double noteDoublePassed = 0;
        double sumPassed = 0;
        boolean hasComma = false;

        while (EINGABE.hasNext()) {
            String note = EINGABE.next();

            if (note.length() != 3) {
                System.out.printf("Note %s wird wegen "
                        + "Formatfehler ignoriert!%n", note);
                continue;
            }

            if (!Character.isDigit(note.charAt(0))
                    || !Character.isDigit(note.charAt(2))) {

                System.out.printf("Note %s wird wegen "
                        + "Formatfehler ignoriert!%n", note);
                continue;
            }

            if (note.charAt(1) != ',' && note.charAt(1) != '.') {

                System.out.printf("Note %s wird wegen "
                        + "Formatfehler ignoriert!%n", note);
                continue;

            }

            // Eingabe pruefen

            if (note.charAt(0) >= '0' && note.charAt(0) <= '9') {
                if (Character.isDigit(note.charAt(0))
                        && Character.isDigit(note.charAt(2))
                        && note.charAt(1) == ',' || note.charAt(1) == '.') {
                    switch (note.charAt(0)) {
                    case '1', '2', '3', '4':
                        isVorkommastelle = true;
                        break;
                    case '5':
                        isVorkommastelle = true;
                        if (isNachkommaStelle) {
                            failureGrades++;
                        }
                        break;
                    default:
                        System.out.println("Note " + note
                                + " wird wegen Vorkommastelle "
                            + note.charAt(0) + " ignoriert!");
                        isNachkommaStelle = false;
                        continue;
                    }
                    switch (note.charAt(2)) {
                    case '0':
                        if (isVorkommastelle) {
                            isNachkommaStelle = true;
                        }
                        break;
                    case '3', '7':
                        if (note.charAt(0) >= '4') {
                            System.out.printf(
                                "Note %s wird wegen Nachkommastelle "
                                + "%c ignoriert!%n", note, note.charAt(2));
                            isNachkommaStelle = false;
                            continue;
                        } else {
                            isNachkommaStelle = true;
                        }
                        break;

                    default:
                        System.out.printf(
                            "Note %s wird wegen Nachkommastelle "
                                    + "%c ignoriert!%n", note, note.charAt(2));
                        isNachkommaStelle = false;
                        continue;
                    }
                    if (note.charAt(1) == ',') {
                        note = note.replace(',', '.');
                        hasComma = true;
                    }

                    noteDouble = Double.parseDouble(note);
                    // Note erfassen
                    if (note.charAt(0) <= '4'
                            && (isVorkommastelle && isNachkommaStelle)) {

                        noteDoublePassed = Double.parseDouble(note);

                        passedGrades++;
                        calcGrades++;
                        sumPassed += noteDoublePassed;

                        if (noteDouble <= bestGrade) {
                            bestGrade = noteDouble;
                        }
                    } else if (note.charAt(0) == '5'
                            && (isVorkommastelle && isNachkommaStelle)) {
                        calcGrades++;
                        if (noteDouble >= worstGrade) {
                            worstGrade = noteDouble;
                        }

                    }
                }
                if (hasComma) {
                    note = note.replace('.', ',');
                }
            }
                // Notenstatistik ausgeben

        }
        avgPass = (double) (sumPassed / passedGrades);
        failureRate = ((double) failureGrades / calcGrades) * 100;

        if (calcGrades >= 1) {

            System.out.printf("%nAnzahl beruecksichtigter Noten: "
                    + "%d%n", calcGrades);
            System.out.printf("Anzahl Bestandene: %d%n", passedGrades);
            System.out.printf("Beste Note: %.1f%n", bestGrade);
            System.out.printf("Schlechteste Note: %.1f%n", worstGrade);
            System.out.printf("Durchschnitt Bestandene: %.1f %n", avgPass);
            System.out.printf("Durchfallquote: %.1f%% %n", failureRate);
        } else {
            System.out.printf("%nAnzahl beruecksichtigter Noten: "
                    + "%d%n", calcGrades);
            System.out.printf("Anzahl Bestandene: %d%n", passedGrades);
        }
    }
}
