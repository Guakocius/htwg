// Wuerfel.java
package aufgabe2;
import java.util.Random;

/**
 * Wuerfel gibt Zufallszahlen zwischen 1 und 6 aus.
 * @author H.Drachenfels
 * @version 11.4.2023
 */
public final class Wuerfel {
    private Wuerfel() { }

    private static final int ANZAHL_AUGEN = 6;
    private static final Random WUERFEL = new Random();

    /**
     * main ist der Startpunkt des Programms.
     * @param args Anzahl der Zufallszahlen (Default 1)
     */
    public static void main(String[] args) {
        int versuche = 1;

        if (args.length > 0) {
            try {
                versuche = Integer.parseInt(args[0]);
            } catch (NumberFormatException x) {
                versuche = 0;
            }

            if (versuche <= 0 || args.length > 1) {
                System.err.printf("Zu viele oder falsche Parameter!%n"
                                  + "Aufruf: java Wuerfel [Anzahl]%n");
                return;
            }
        }

        for (int i = 0; i < versuche; ++i) {
            System.out.println(WUERFEL.nextInt(ANZAHL_AUGEN) + 1);
        }
    }
}

