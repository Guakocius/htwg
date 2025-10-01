package aufgabe6;

import aufgabe5.Note;

/**
 * Utility class which gets an un-graded grade as an input
 * and writes a graded grade.
 */
public final class BenoteteLeistung extends Leistung {
    private final Note note;

    /**
     * Constructor of BenoteteLeistung which gets fach and note
     * as inputs, fach fetched from super type, note from
     * aufgabe5.Note.
     * @param fach
     * @param note
     */
    public BenoteteLeistung(String fach, String note) {
        super(fach);
        this.note = Note.valueOf(note);
    }

    @Override
    public boolean istBenotet() {
        return true;
    }

    @Override
    public boolean istBestanden() {
        if ("BE".equals(note.toString())) {
            return true;
        } else if ("NB".equals(note.toString())) {
            return false;
        }

        return this.note.istBestanden();
    }

    @Override
    public String getNote() {
        return this.note.toString();
    }

    @Override
    public String getNoteInWorten() {
        int noteValue = this.note.intValue();
        if (noteValue >= 10 && noteValue <= 15) {
            return "sehr gut";
        } else if (noteValue >= 16 && noteValue <= 25) {
            return "gut";
        } else if (noteValue >= 26 && noteValue <= 35) {
            return "befriedigend";
        } else if (noteValue >= 36 && noteValue <= 40) {
            return "ausreichend";
        } else {
            return "nicht ausreichend";
        }
    }

}
