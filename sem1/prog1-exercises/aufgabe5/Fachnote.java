package aufgabe5;

/**
 * Diese Klasse determiniert die Fachnote und deren Eigenschaften.
 */
public final class Fachnote {

    private final String fach;
    private final Note note;

    /**
     *
     * @param fach
     * @param note
     */
    public Fachnote(String fach, Note note) {
        this.fach = fach;
        this.note = note;
    }

    /**
     *
     * @return fach
     */
    public String fach() {

        if (fach == null || fach.isEmpty()) {
            throw new IllegalArgumentException("Fach darf nicht leer sein");
        }

        return fach;
    }

    /**
     *
     * @return note
     */
    public Note note() {

        if (note == null) {
            throw new IllegalArgumentException("Note darf nicht leer sein");
        }

        return note;
    }

}
