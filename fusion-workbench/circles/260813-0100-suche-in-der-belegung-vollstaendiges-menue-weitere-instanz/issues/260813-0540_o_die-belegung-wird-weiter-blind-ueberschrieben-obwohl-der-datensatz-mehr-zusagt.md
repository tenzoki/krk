Die Belegung wird weiter blind überschrieben, obwohl der Datensatz mehr zusagt

---

Der bindende Datensatz zur Ablage führt unter Möglichkeit 1, auf der die Runde fährt:

> `shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`
> — „Dafür: Kein Gemisch, **keine verlorene Änderung an Lesezeichen und Belegung** …"

Gebaut ist der Lesen-Ändern-Schreiben-Durchgang nur für die Lesezeichen
(`crates/krk-ui/src/appkit/anwendung.rs:1505-1530`). Die Belegung geht weiterhin blind
darüber: `belegungsansicht_verlassen` schreibt die Arbeitskopie, die beim Öffnen der Ansicht
entstand, unter der Schreibsperre in `keymap.toml`, ohne die Datei vorher frisch zu lesen
(`crates/krk-ui/src/appkit/anwendung.rs:3039-3056`, `crates/krk-core/src/tasten/belegung.rs:1198-1203`).

**Was verlorengeht.** Instanz A öffnet die Belegungsansicht. Instanz B belegt in derselben
Spanne eine Taste um und verlässt ihre Ansicht. A verlässt ihre und schreibt ihre Arbeitskopie
darüber: die Umbelegung von B ist fort, ohne Meldung. Die Tabelle im Datensatz selbst nennt
den Fall („`keymap.toml` … Dasselbe Muster, seltener").

**Was hält.** C3.7 ist erfüllt: der Schreibvorgang steht unter der Sperre, zwei Instanzen
beschreiben nie dieselbe Nachbardatei zugleich, und ein Gemisch kann nicht entstehen. Nur die
verlorene Änderung bleibt. Der Spec verlangt das frische Lesen ausdrücklich allein für die
Lesezeichen (C3.8), und der Plan folgt ihm in S13. Der Bau folgt also Spec und Plan; die
Zusage im Datensatz ist die weiter reichende.

---

**Schwere:** mittel. Nutzerarbeit, die verlorengehen kann, aber ein enges Fenster und ein
Zustand, der vor dieser Runde ebenso bestand. Der Befund ist die Lücke zwischen dem, was der
Datensatz zusagt, und dem, was gebaut ist.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:3039-3056`,
`crates/krk-core/src/tasten/belegung.rs:1198-1203`,
`shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`

**Domain:** code

## Zwei Wege, und der zweite ist der ehrlichere

1. **Die Belegung ebenso lesen wie die Lesezeichen.** Das setzt voraus, dass sich zwei
   Belegungen zusammenführen lassen, und das ist keine kleine Frage: die Arbeitskopie ist ein
   ganzer Bestand und keine benannte Änderung. Ein `Aenderung`-Wert wie bei den Lesezeichen
   müsste die Zuweisungen der Sitzung einzeln tragen.
2. **Den Datensatz nachziehen.** Die Zeile „keine verlorene Änderung an Lesezeichen und
   Belegung" auf die Lesezeichen einschränken und den Verlust an der Belegung als benannten
   Preis dazuschreiben, wie es der Datensatz für die nicht gemerkte Aufteilung der zweiten
   Instanz schon tut.

Der Nutzer entscheidet; ohne Entscheidung steht im Speicher eine Zusage, die der Baum nicht
hält.
