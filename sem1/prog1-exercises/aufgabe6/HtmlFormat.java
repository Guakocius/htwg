package aufgabe6;

import java.io.PrintWriter;

/**
 * printet eine HTML-Seite in tabellarischer Form, welche den
 * ausgewerteten Notenspiegel darstellt.
 */
public final class HtmlFormat {

    private HtmlFormat() { }

    /**
     * Gibt HTML-Code in Form eines Textblocks aus.
     * @param ausgabe
     * @param name
     * @param leistungen
     */
    public static void ausgeben(
        PrintWriter ausgabe, String[] name,
        LeistungsListe leistungen) {

        StringBuilder sb = new StringBuilder();
        for (String s : name) {
            sb.append(s).append(" ");
        }

        String nameString = sb.toString().trim();

        ausgabe.printf("""
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            table { width: 100%%; }
            th { text-align: left; }
        </style>
        <title>Notenspiegel</title>
        </head>
        <body>
        <h1>Notenspiegel für %s</h1>
        <hr>
        <table>
        <tr><th>Fach:</th><th>Art:</th><th>Note:</th></tr>
            """.replace("\n", System.lineSeparator()), nameString);

        for (Leistung l : leistungen) {
            String art = l instanceof BenoteteLeistung ? "L" : "S";
            String note = (l != null && l.istBenotet() && art.equals("L"))
                ? l.getNoteInWorten() + " ("
                + l.getNote().replace('.', ',') + ")"
                : (l != null && l.istBestanden()
                ? "bestanden" : "nicht bestanden");
            String color = (l != null && !l.istBestanden())
                ? " style=\"color:red;\"" : "";

            if (l != null) {
                ausgabe.printf("<tr><td%s>%s</td><td>%s</td><td>%s</td></tr>%n",
                    color, l.getFach(), art, note);
            }
        }

        ausgabe.println("""
            </table>
            <hr>
            L = Leistungsnachweis, S = Schein
            </body>
            </html>
            """.replace("\n", System.lineSeparator()));
    }
}
