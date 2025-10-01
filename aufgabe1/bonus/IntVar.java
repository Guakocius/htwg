package aufgabe1.bonus;

import java.util.Scanner;

public class IntVar {
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
        int ersteZahl = 0;
        int zweiteZahl = 0;

        while(!EINGABE.hasNextInt()) {
            System.out.println("Bitte gebe eine valide Zahl ein!");
            EINGABE.next();
        }
        while(EINGABE.hasNextInt()) {
            ersteZahl = EINGABE.nextInt();
            zweiteZahl = EINGABE.nextInt();
        }

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
