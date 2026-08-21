bundle und release schreiben an denselben Ort, und ein Entwicklungsbau zerstört das beglaubigte Bündel

---

`cargo xtask bundle` und `cargo xtask release` legen beide `target/KRK.app` an. Beide gehen
über dieselbe `bundle::Vorlage`, und das ist Absicht: ein zweiter Bündelbauer wäre die zweite
Wahrheit über die Struktur von `KRK.app`, und `release.rs` begründet die Wiederverwendung
ausdrücklich.

Die Folge ist trotzdem unangenehm. Ein beglaubigtes Bündel wird von **jedem** gewöhnlichen
Entwicklungsbau überschrieben, und über `bundle` hängen `run`, `run-terminal`, `tasten`,
`menue`, `durchstich` und `frisch`. Wer nach einem Release-Lauf `make run` tippt, hat die
Beglaubigung weg.

---

**Der Unterschied im Preis ist der Punkt.** Ein Entwicklungsbündel ist in Sekunden wieder da.
Ein beglaubigtes verlangt zwei Übersetzungsläufe im Profil `release`, `lipo`, eine Signatur mit
gehärteter Laufzeitumgebung und einen Netzlauf zu Apple, den `--wait` abwartet. Am 260813
gemessen: rund eine Minute Übersetzung, dazu die Beglaubigung, die in drei Statusabfragen
durchlief. Das ist kein Datenverlust, aber es sind Minuten für einen Tastendruck, der wie ein
gewöhnlicher aussieht.

**Am 260813 stand genau das an.** Der Nutzer hatte ein beglaubigtes Bündel für einen zweiten
Mac und musste zugleich den Abnahmelauf der Runde 6 fahren, der über `make durchstich` und
damit über `bundle` läuft. Beides an einem Ort geht nicht.

**Drei Zuschnitte, mit ihren Kosten.**

1. **Getrennte Orte.** `release` legt sein Ergebnis woanders ab, etwa `target/release-bundle/KRK.app`.
   Billig und wirksam, ändert aber einen Pfad, den `README.md`, der Makefile und die
   Messstrecke nennen. Wer das baut, zählt die Stellen nach, statt sie zu schätzen.
2. **`bundle` weigert sich**, ein beglaubigtes Bündel zu überschreiben, solange nicht ausdrücklich
   etwas anderes gesagt wird. Erkennbar ist die Lage an `xcrun stapler validate`, das genau diese
   Frage beantwortet. Kostet einen Aufruf je Bau und eine neue Ausnahme.
3. **Nur eine Warnung.** `release` sagt am Ende, dass der nächste Entwicklungsbau das Ergebnis
   nimmt. Billigste Möglichkeit, verhindert nichts, und dieses Projekt hat am 260812 gerade
   erlebt, was eine Meldung wert ist, die eine Folge nicht nennt
   (`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-…`).

**Bis das entschieden ist, hilft eine Kopie:** `ditto target/KRK.app ~/Desktop/KRK.app` vor dem
nächsten Entwicklungsbau. Die Beglaubigung hängt am Bündel und nicht an seinem Ort, das Ticket
reist also mit.

Herkunft: gemeinsamer Speicher. Betrifft `xtask` und den Bauweg des ganzen Projekts, nicht die
Directive einer Runde.

---

## Nachtrag 260821-1532 (Abgleich): zwei Abfangstellen, beide hinter dem Schaden

**Offen und unverändert.** `bundle` schreibt weiter nach `target/KRK.app`, und ein
Entwicklungsbau zerstört ein dort liegendes beglaubigtes Bündel unwiderruflich. Keiner der drei
Zuschnitte oben ist gebaut.

**Was seit dem 260821 dazugekommen ist**, und was die Durchsicht
`shared/reviews/260821-1346-coderev-artefakt-und-release.md` hier einzutragen verlangt hat: es
sind jetzt **zwei** Stellen, die den Fall abfangen, `beglaubigen` und `veroeffentlichen`. Die
zweite prüft am Bündel, ob das Beglaubigungsticket angeheftet ist
(`xtask/src/veroeffentlichung.rs`, `traegt_angeheftetes_ticket` an
`Contents/CodeResources`), und ein Entwicklungsbündel trägt keins. **Beide stehen hinter dem
Schaden und nicht davor:** sie fangen den Fall am Anfang der siebten und der achten Station,
also nachdem das beglaubigte Bündel bereits überschrieben ist. Das ist eine Milderung und kein
Abschluss; der Plan `shared/planning/260821-1221_*_plan-artefakt-und-release.md` sagt es in
seiner Risikotabelle ausdrücklich, und kein Schritt jener Runde behauptet etwas anderes.

**Nachgetragen von:** reconciler, Abgleich 260821-1532, Baumstand `4e810f9`.
