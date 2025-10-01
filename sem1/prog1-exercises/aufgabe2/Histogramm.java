// Histogramm.java
package aufgabe2;

import java.util.Scanner;

/**
 * Histogramm liest ganze Zahlen zwischen 1 und 6 ein und
 * gibt deren H&auml;ufigkeitsverteilung als Histogramm aus.
 * @author Alexander Engelhardt
 * @version 05.11.2024
 */
public final class Histogramm {
    private Histogramm() {
    }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     *
     * @param args wird nicht verwendet.
     */
    public static void main(String[] args) {

        final int six = 6;
        int[] counters = new int[six];

        //---------------------------------------------------- Zahlen einlesen
        System.out.println("Ganze Zahlen zwischen 1 und 6 eingeben "
                + "(Ende mit Strg-D/Strg-Z):");

        while (EINGABE.hasNextInt()) {
            int number = EINGABE.nextInt();

            if (number >= 1 && number <= six) {
                counters[number - 1]++;
            } else {
                System.out.println("Falsche Eingabe wird ignoriert: "
                        + number);
            }
        }
            //Histogramm ausgeben

            System.out.println("Histogramm");
            for (int i = 0; i < counters.length; i++) {
                for (int j = 0; j <= counters[i] - 1; j++) {

                    final int five = 5;
                    if ((j + 1) % five == 0) {

                        String dollar = "$";

                        System.out.printf("%s", dollar);
                    } else {
                        System.out.print(i + 1);
                    }
                }
                System.out.printf(" (%d)%n", counters[i]);
            }
        }
    }

