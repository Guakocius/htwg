package aufgabe1.bonus;

import java.util.Scanner;

public class ByteVar {
    private ByteVar() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(String[] args) {
        /* Die Konstanten max und min in hexadezimal initialisiert */

        final byte max = 0x7F;
        final byte min = -0x80;



        // Eingabeaufforderung, dass der User
        // zwei Zahlen zwischen der kleinst- und höchstmöglichen
        // Zahl eingeben soll

        System.out.println("Zwei ganze Zahlen zwischen "
                + min + " und " + max + " eingeben:");

        /* Zwei Zahlen, die der User eingibt */
        byte ersteZahl = EINGABE.nextByte();
        byte zweiteZahl = EINGABE.nextByte();

        String ersteZahlOct = Integer.toOctalString(ersteZahl);
        String ersteZahlHex = Integer.toHexString(ersteZahl);
        String zweiteZahlOct = Integer.toOctalString(zweiteZahl);
        String zweiteZahlHex = Integer.toHexString(zweiteZahl);

        System.out.printf("%d ist oktal %s und hexadezimal %s%n",
                ersteZahl, ersteZahlOct, ersteZahlHex);
        System.out.printf("%d ist oktal %s und hexadezimal %s%n",
                zweiteZahl, zweiteZahlOct, zweiteZahlHex);
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
