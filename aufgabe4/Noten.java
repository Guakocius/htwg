package aufgabe4;

/**
 * Klasse Noten dient als Uitility-Klasse für Klausurergebnis.
 */

public final class Noten {
    private Noten() { }

    /**
     * Best grade.
     */
    public static final double BESTE = 1.0;

    /**
     * Worst grade.
     */
    public static final double SCHLECHTESTE = 5.0;

    /**
     *
     * @param note String
     * @return true
     */
    public static boolean istZulaessig(String note) {


        if (note.length() != 3) {
            return false;
        }

        if (!Character.isDigit(note.charAt(0))
                || !Character.isDigit(note.charAt(2))) {
            return false;
        }

        if (note.charAt(1) != ',' && note.charAt(1) != '.') {
            return false;
        }

        switch (note.charAt(0)) {
        case '1', '2', '3', '4', '5':
            break;
        default:
            return false;
        }

        switch (note.charAt(2)) {
        case '0':
            break;
        case '3', '7':
            if (note.charAt(0) == '4' || note.charAt(0) == '5') {
                return false;
            }
            break;
        default:
            return false;
        }
        return true;
    }

    /**
     *
     * @param note String
     * @return Double.parseDouble(note)
     * @throws IllegalArgumentException invalid grades
     */
    public static double toDouble(String note) {
        if (istZulaessig(note)) {
            note = note.replace(',', '.');
            return Double.parseDouble(note);
        } else {
            throw new IllegalArgumentException(note);
        }
    }

    /**
     *
     * @param note double
     * @return note in format
     * @throws IllegalArgumentException when wrong number
     */
    public static String toString(double note)
            throws IllegalArgumentException {

        if (note < BESTE || note > SCHLECHTESTE) {
            throw new IllegalArgumentException();
        }

        return String.format("%.1f", note).replace('.', ',');
    }

    /**
     *
     * @param note double
     * @return note <= 4.0
     */
    public static boolean istBestanden(double note) {

        return note <= 4.0;
    }

    /**
     *
     * @param note double
     * @param bestGrade double
     * @return the better of the two grades
     */
    public static double bessere(double note, double bestGrade) {

        if (note <= bestGrade) {
            bestGrade = note;
        }

        return bestGrade;
    }

    /**
     *
     * @param note double
     * @param worstGrade double
     * @return the worse of the two grades
     */
    public static double schlechtere(double note, double worstGrade) {

        if (note >= worstGrade) {
            worstGrade = note;
        }

        return worstGrade;
    }
}