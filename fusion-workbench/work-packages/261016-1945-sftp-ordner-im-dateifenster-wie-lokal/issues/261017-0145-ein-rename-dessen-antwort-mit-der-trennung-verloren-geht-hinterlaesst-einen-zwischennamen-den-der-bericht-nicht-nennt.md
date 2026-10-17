Ein RENAME, dessen Antwort mit der Trennung verloren geht, hinterlässt einen Zwischennamen, den der Bericht nicht nennt
---
Bricht die Verbindung, nachdem ein `RENAME` des Überschreibens an den Server ging, ist sein Ausgang unbekannt: der Server kann es ausgeführt haben. Der Code liest jedes `Fernfehler::Getrennt` als „nicht geschehen“. An zwei Stellen liegt dann ein Eintrag unter einem Zwischennamen mit Punkt am Anfang, und der Bericht nennt den Namen nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (kein Inhalt geht verloren; der Eintrag liegt auf dem Server unter einem versteckten Namen, den der Nutzer aus dem Bericht nicht erfährt)
**Domain:** code
**Cross-references:** `261017-0145-reviewer-schlussdurchsicht-tausch-a97e4fbb-bis-0284ace2.md`

**Belege** (Code gelesen, nicht ausgeführt; dass eine wartende Anfrage nach dem Ende des Servers `Getrennt` bekommt, hält `ein_beendeter_server_beantwortet_jede_wartende_anfrage_mit_getrennt` in `crates/krk-core/tests/fern.rs`)

- `crates/krk-core/src/operation/fern.rs`, `tauschen`, Schritt 1: `beiseite_legen` gibt `Getrennt` als `Err` zurück (`kann_vergeben_heissen` ist dafür falsch), also `Tausch::ZielBlieb`. Hat der Server umbenannt, liegt das Ziel unter `.krk-alt-…`.
  - Kopieren (`kopie_ersetzt_ziel`): der Bericht sagt „das Ziel ließ sich nicht ersetzen“ und nennt `.krk-ersatz-…`, nicht `.krk-alt-…`. Am Namen des Ziels steht nichts.
  - Verschieben auf demselben Server (`umbenennen_ersetzt_ziel`): ebenso; genannt wird allein der Zwischenname der Quelle.
- `fern.rs`, `umbenennen_ersetzt_ziel`, Schleife über die Zwischennamen: `Some(Err(fehler))` mit `Getrennt` bricht ab, `zwischen` bleibt `None`, und der Bericht sagt „das Ziel ließ sich nicht ersetzen“. Hat der Server umbenannt, liegt die **Quelle**, also der Eintrag, den der Nutzer verschiebt, unter `.krk-ersatz-…`, und ihr Name steht nirgends.
- Schritt 2 und das Zurückholen sind gedeckt: dort nennt `beide_beiseite_melden` beide Namen, auch wenn einer davon schon wieder frei ist.

**Behebung (Richtung)**

Der Ausgang eines `RENAME` ohne Antwort ist vom Client nicht entscheidbar. Ein `Getrennt` an diesen zwei Stellen bekommt deshalb einen eigenen Ausgang, dessen Text beide möglichen Namen nennt („liegt unter … oder unter …“), statt „nicht ersetzt“ zu behaupten.

**Abnahme**

- Für jedes `RENAME` des Überschreibens gilt: endet es mit `Getrennt`, nennt der Bericht jeden Namen, unter dem der betroffene Eintrag danach liegen kann.
- Eine Probe mit einer Attrappe, die das `RENAME` ausführt und vor der Antwort trennt, zeigt das für das Beiseitelegen und für das Umbenennen der Quelle.

---
Resolved: Ein `RENAME` des Überschreibens, das mit `Fernfehler::Getrennt` endet, gilt als vielleicht geschehen (`Fehlschlag::laesst_den_ausgang_offen`). `crates/krk-core/src/operation/fern.rs`: `beiseite_legen` liefert `Beiseite::Vielleicht { alt, fehler }`, `tauschen` daraus `Tausch::ZielVielleichtBeiseite`; `kopie_ersetzt_ziel` und `umbenennen_ersetzt_ziel` melden unter dem Namen des Ziels `Text::VorgangZielVielleichtBeiseite` mit dem Zwischennamen `.krk-alt-…`. In `umbenennen_ersetzt_ziel` nennt ein getrenntes `RENAME` der Quelle auf `.krk-ersatz-…` oder zurück `Text::VorgangQuelleVielleichtUnterZwischennamen` mit dem Zwischennamen. Texte in de/fr/en, Modulkopf (Absatz zum `RENAME` ohne Antwort) und `HowTo.md` nachgezogen. Proben mit einem Perl-Vermittler, der das `RENAME` ausführen lässt und vor der Antwort trennt: `ein_beiseitelegen_ohne_antwort_nennt_beide_namen_des_ziels` und `ein_umbenennen_der_quelle_ohne_antwort_nennt_beide_namen_der_quelle` (fern.rs, Prüfmodul).
