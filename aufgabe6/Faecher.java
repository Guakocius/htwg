package aufgabe6;

/**
 * Utility-Klasse, welche alle im Semester vorkommenden
 * Fächer speichert und prüft, ob das eingegebene Fach
 * zulässig ist.
 */
public final class Faecher {

    private Faecher() { }

    private static final String[] FAECHER = {
        "Mathematik 1", "Digitaltechnik", "Programmiertechnik 1",
        "Softwaremodellierung"
    };

    /**
     * Überprüft, ob das eingelesene Fach zulässig ist.
     * @param fach
     * @return f.equals(fach) ? true : false
     */
    public static boolean istZulaessig(String fach) {
        for (String f : FAECHER) {
            if (f.equals(fach)) {
                return true;
            }
        }
        return false;
    }
}
