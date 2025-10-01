// IntVar.java

package aufgabe1;

import java.util.Scanner;

/**
 * IntVar zeigt den Umgang mit Variablen vom Typ int.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Alexander Engelhardt
 * @version 15.10.2024
 */
public final class IntVar {
    private IntVar() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(String[] args) {
        /* Die Konstanten max und min in hexadezimal initialisiert */

        final int max = 0x7FFFFFFF;
        final int min = -0x80000000;



        // Eingabeaufforderung, dass der User
        // zwei Zahlen zwischen der kleinst- und höchstmöglichen
        // Zahl eingeben soll

        System.out.println("Zwei ganze Zahlen zwischen "
                + min + " und " + max + " eingeben:");

        /* Zwei Zahlen, die der User eingibt */
        int ersteZahl = EINGABE.nextInt();
        int zweiteZahl = EINGABE.nextInt();

        //String ersteZahlOct = Integer.toOctalString(ersteZahl);
        //String ersteZahlHex = Integer.toHexString(ersteZahl);
        //String zweiteZahlOct = Integer.toOctalString(zweiteZahl);
        //String zweiteZahlHex = Integer.toHexString(zweiteZahl);

        System.out.printf("%d ist oktal %o und hexadezimal %x%n",
                ersteZahl, ersteZahl, ersteZahl);
        System.out.printf("%d ist oktal %o und hexadezimal %x%n",
                zweiteZahl, zweiteZahl, zweiteZahl);
        System.out.printf("%d + %d ist %d%n",
                ersteZahl, zweiteZahl, (ersteZahl + zweiteZahl));
        System.out.printf("%d - %d ist %d%n",
                ersteZahl, zweiteZahl, (ersteZahl - zweiteZahl));
        System.out.printf("%d * %d ist %d%n",
                ersteZahl, zweiteZahl, (ersteZahl * zweiteZahl));
        System.out.printf("%d / %d ist %d%n",
                ersteZahl, zweiteZahl, (ersteZahl / zweiteZahl));
        System.out.printf("%d %% %d ist %d%n",
                ersteZahl, zweiteZahl, (ersteZahl % zweiteZahl));
        System.out.printf("%d == %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl == zweiteZahl));
        System.out.printf("%d != %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl != zweiteZahl));
        System.out.printf("%d < %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl < zweiteZahl));
        System.out.printf("%d <= %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl <= zweiteZahl));
        System.out.printf("%d > %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl > zweiteZahl));
        System.out.printf("%d >= %d ist %b%n",
                ersteZahl, zweiteZahl, (ersteZahl >= zweiteZahl));
    }
}

