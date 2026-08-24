`zusammenfassen` nimmt auch eine Datei an, und C2.6 hängt allein am künftigen Rufer

---

`leseprofil::bausteine::zusammenfassen` prüft nicht, ob der übergebene Pfad ein Verzeichnis ist.
Für eine **Datei**, deren Pfad ein Pfadmuster erfüllt, liefert es heute `Some(Zusammenfassung)`
statt `None`. C2.6 verlangt das Gegenteil: „Kein Profil greift auf eine Datei, auch dann nicht,
wenn ihr Pfad ein Pfadmuster erfüllt." Der Kern trägt die Zusage nicht, weder als Prüfung noch
als Vorbedingung im Doc-Kommentar; sie steht allein in einem Nebensatz von Planschritt 9, der
noch nicht gebaut ist.

---

**Gemessen am 260824-1214 an diesem Baum**, Stand `abe1a31`, in einer Wegwerfprobe unter
`crates/krk-core/tests/`, die danach wieder entfernt wurde.

## Der Lauf

```text
Profil:   pfad = 'irgendeine',  eine Zeile  zaehlung = { }
Aufruf:   zusammenfassen(&profile, <ordner>/irgendeine.md)
Ergebnis: Some(Zusammenfassung { name: "irgendeine.md", zeilen: [("Zahl", Nicht)] })
Text:     "Name: irgendeine.md\nPfad: …/irgendeine.md\nZahl: --"
```

Der Weg dorthin steht in `crates/krk-core/src/leseprofil/bausteine.rs:134-155`:
`std::fs::canonicalize` gelingt für eine Datei, der **erste** Erkennungsdurchgang sieht allein
auf den Pfadtext (`erkennung.rs:104-112`) und braucht keine Einträge, also greift das Profil.
Erst der zweite Durchgang bräuchte einen Verzeichnisleselauf und käme leer zurück; er läuft hier
nie. Ein Profil, das seinen Ort über eine **Kennzeichendatei** erkennt, kann eine Datei deshalb
nicht treffen, ein Profil mit Pfadmuster schon.

## Warum das heute niemandem auffällt und morgen doch

Planschritt 9 setzt die Prüfung in `vorschaumodell::laden`: „ist der Eintrag kein `Typ::Datei`
und liefert `zusammenfassen` ein Ergebnis, entsteht `Inhalt::Zusammenfassung`". Das ist eine
richtige Stelle, aber die einzige. Die Aufstellung `## Was der Übersetzer einfordert, und was er
nicht einfordert` im Plan führt C2.6 nicht; kein Übersetzerfehler und keine Probe im Kern hält
die Bedingung. Ein zweiter Rufer von `zusammenfassen` — der Messmodus, eine spätere Runde, eine
Probe — bekäme für eine Datei eine Zusammenfassung geliefert und nähme sie für gültig.

Der Doc-Kommentar von `zusammenfassen` (`bausteine.rs:107-122`) nennt drei Lagen für `None` und
diese nicht; er spricht durchgehend von „einem ausgewaehlten Ordner", sagt aber nirgends, dass
der Aufrufer für einen Ordner einzustehen hat.

## Zwei Wege, und sie schließen einander nicht aus

1. **Die Vorbedingung ausschreiben.** Ein Absatz im Doc-Kommentar von `zusammenfassen`: der
   Aufrufer stellt sicher, dass `ordner` ein Verzeichnis benennt, und C2.6 hängt daran. Kostet
   nichts, hält aber nichts.
2. **Die Frage im Kern entscheiden.** `zusammenfassen` liefert `None`, wenn der aufgelöste Pfad
   kein Verzeichnis ist. Die Auskunft liegt im `canonicalize`-Umfeld ohnehin nahe; ein
   `symlink_metadata`/`metadata` am aufgelösten Pfad kostet einen Systemaufruf je
   Zusammenfassung, und der Haushalt aus C6 zählt Verzeichnisleseläufe und Dateiöffnungen und
   nicht diesen. Dann hält der Kern die Zusage, und Schritt 9 behält seinen Zweig als zweite
   Sperre.

Welcher Weg gilt, entscheidet der Nutzer oder der Plan. **Der heutige Zustand ist keiner von
beiden:** die Zusage steht in keinem Kommentar und in keiner Probe.

**Schwere:** mittel. Kein Fehlverhalten im ausgelieferten Bündel, denn der Rufer fehlt noch.
Der Befund ist eine Zusage ohne Halter, und sie fällt genau dann, wenn ein zweiter Rufer
hinzukommt.

**Gefunden:** coderev, bei der Durchsicht von Bündel B am 260824-1214.

**Betroffen:** `crates/krk-core/src/leseprofil/bausteine.rs` (`zusammenfassen`, `gezaehlt`),
`crates/krk-core/src/leseprofil/erkennung.rs` (erster Durchgang),
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Schritt 9, `## Was der Übersetzer einfordert`)

**Domain:** code
