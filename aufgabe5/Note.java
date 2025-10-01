package aufgabe5;

/**
 * Klasse Note dient als Utility-Klasse für Notenspiegel.
 */
public final class Note {

    private final int note;

    private Note(int note) {
        this.note = note;

    }

    /**
     * Bestmögliche Note.
     */
    public static final Note BESTE = new Note(10);
    /**
     * Schlechstmögliche Note.
     */
    public static final Note SCHLECHTESTE = new Note(50);

    /**
     *
     * @param note
     * @return note : valueOf Object Note
     */
    public static Note valueOf(int note) {
        if (isValidGrade(note)) {
            return new Note(note);
        } else {
            throw new IllegalArgumentException("unzulaessige Note " + note);
        }
    }

    /**
     *
     * @param note : String
     * @return note as a String
     */
    public static Note valueOf(String note) {

        int noteInt = (Character.getNumericValue(note.charAt(0)) * 10
            + Character.getNumericValue(note.charAt(2)));

        if (isValidGrade(noteInt)) {
            return new Note(noteInt);
        } else {
            throw new IllegalArgumentException("unzulaessige Note " + note);
        }
    }

    /**
     *
     * @param note
     * @return true if valid grade, false otherwise
     */
    private static boolean isValidGrade(int note) {
        final int[] validGrades = {10, 13, 17,
            20, 23, 27, 30, 33, 37, 40, 50};

        for (int n : validGrades) {
            if (n == note) {
                return true;
            }
        }
        return false;
    }

    /**
     *
     *
     * @return note.note
     */
    public int intValue() {
        return this.note;
    }
    /**
     *
     *
     * @return note.note <= 40
     */
    public boolean istBestanden() {
        return this.note <= 40;
    }

    /**
     *
     *
     * @return note as a String
     */
    @Override
    public String toString() {
        return String.format("%.1f", (double) this.note / 10);
    }

    /**
     *
     * @param o
     * @return true if equal, false otherwise
     */
    @Override
    public boolean equals(Object o) {

        if (o instanceof Note) {
            Note that = (Note) o;
            return this.note == that.note;
        }
        return false;
    }

    /**
     *
     * @return hashcode
     */
    @Override
    public int hashCode() {
        return this.note;
    }
}
