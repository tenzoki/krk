# Wie wird jeder Löschweg abgesichert, und fällt das endgültige Löschen ganz weg?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md` (die überholte Festlegung), `shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`, `shared/analyses/260817-0419-verlust-des-speichers-shared.md`, `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`

---

## Question

KRK führte bis zum 260817 zwei Löschwege, und sie waren ungleich gesichert: `delete` und `cmd+delete` räumten ohne jede Rückfrage in den Papierkorb, `f8` und `opt+cmd+delete` löschten endgültig nach einer Rückfrage je Vorgang. Diese Aufteilung ist die Nutzerantwort vom 260802-1105 und in den Commits `daecb45` und `343a7f3` umgesetzt.

In der Nacht zum 260817 hat der ungesicherte Weg einen Schaden angerichtet: KRK hat, von Hand bedient, den Speicher `fusion-workbench/shared` des eigenen Projektverzeichnisses mit 189 verfolgten Dateien in zehn Unterordnern in den Papierkorb geräumt, auf einen Tastendruck, ohne Rückfrage, vier Stunden unbemerkt und gefunden durch einen zufälligen `git status`. Der Nutzer berichtet einen gleichartigen Vorfall auf einem zweiten Gerät. Die Fehlbedienung ist als Ursache belegt.

Die Frage zerfällt in drei: Bekommt jeder Löschweg eine Rückfrage? Bleibt der endgültige Weg bestehen? Und was geschieht auf Zielen, die keinen Papierkorb führen, wenn der endgültige Weg nicht mehr existiert?

## Options

1. **Rückfrage für beide Wege, beide Wege bleiben** — der Papierkorbweg bekommt dieselbe Rückfrage, die der endgültige schon trägt.
   - Pro: kleinster Eingriff, keine Aufzählung wird angefasst, die Belegung bleibt wie sie ist.
   - Contra: der unwiederbringliche Weg bleibt in der Anwendung. Eine Rückfrage ist ein Tastendruck, und ein Nutzer, der sie gewohnheitsmäßig bestätigt, hat weiterhin einen Weg zum endgültigen Verlust.

2. **Rückfrage für den Papierkorbweg, der endgültige Weg fällt ganz weg** — KRK kennt danach genau einen Löschweg.
   - Pro: es gibt keinen Weg durch KRK mehr, der Daten unwiederbringlich entfernt. Die Aufteilung, die der Nutzer auseinanderhalten musste, entfällt; zwei Tastenkombinationen werden frei.
   - Contra: eine Variante aus `Kommando` und eine aus `Art` zu entfernen fasst jede vollständige Fallunterscheidung darüber an. Auf Zielen ohne Papierkorb kann KRK danach gar nicht mehr löschen.

3. **Rückfrage für den Papierkorbweg, endgültiger Weg nur noch als Rückfallweg** — sichtbar ist ein Löschbefehl; scheitert der Papierkorb, löscht KRK nach einer zweiten Rückfrage endgültig.
   - Pro: KRK bleibt auf jedem Ziel handlungsfähig.
   - Contra: ein Papierkorb, der bei fehlender Anbindung zur Löschung wird, ist der schlimmste denkbare Rückfallweg; genau diesen Fall schließt `OhnePapierkorb` im Kern heute ausdrücklich aus. Der Nutzer hätte einen Weg zum endgültigen Verlust, den er nicht gewählt hat, und er stünde am Ende einer Kette, die er schon einmal bestätigt hat.

4. **Rückfrage erst ab einer Umfangsschwelle** — kleine Vorgänge laufen wie heute durch.
   - Pro: die Tastaturarbeit bleibt im Alltag unbeeinflusst.
   - Contra: die Schwelle wäre eine Zahl ohne Herleitung, und der Schadensfall zeigt, dass gerade der beiläufige Tastendruck den Schaden anrichtet. Eine einzelne wichtige Datei ist genauso verloren wie ein Baum.

## Constraints

- Die Antwort gilt gleichermaßen für `delete` und `cmd+delete`, weil beide dasselbe `Kommando` tragen.
- Sie muss für die Mehrfachauswahl tragen und für Ordner mit Inhalt.
- `trashItemAtURL:` scheitert auf Datenträgern ohne Papierkorb und auf manchen Netzlaufwerken. Jede Antwort, die den endgültigen Weg entfernt, muss sagen, was dort geschieht.
- Die Rückschritt-Regel aus `krk-ui/src/kommandos/rueckschritt.rs` bleibt nötig: eine Rückfrage bei jedem berichtigten Zeichen des Filtertextes wäre unbrauchbar.
- KRK führt keinen eigenen Rückgängig-Speicher; er wäre ein zweiter Papierkorb und liefe gegen die Maxime „supersimpel".

## Recommendation

Möglichkeit 2, und zwar in der Fassung, die auf Zielen ohne Papierkorb nicht löscht statt endgültig zu löschen. Sie ist die einzige, nach der die Zusage „kein Weg durch KRK führt zum unwiederbringlichen Löschen" ohne Einschränkung gilt, und eine Zusage mit Ausnahme ist an einer zerstörenden Handlung keine.

## Antwort des Nutzers

Die Antwort ist in zwei Klärungsrunden und einer Verschärfung entstanden.

**Am 260816-2144** hat der Nutzer verlangt, dass jede Datei-Löschfunktion durch eine Rückfrage gesichert wird.

**In der ersten Klärungsrunde** hat er vier Festlegungen getroffen. Erstens fragen beide Tasten des Papierkorbwegs nach, auch bei einer einzelnen Datei; „Abbrechen" ist vorbelegt, die Eingabetaste bricht ab, Cmd+Return räumt weg. Zweitens ist die laute Warnung dasselbe Blatt mit Warnzeichen, vollem Pfad, Zahl der betroffenen Einträge und dem Grund im Klartext; eine Bestätigung genügt, den Namen abzutippen verlangt KRK nicht. Drittens liegt die Umfangsschwelle bei 25 Einträgen im Unterbaum, gedeckelt gezählt: höchstens 26 zählen, dann „mehr als 25" melden, und die Warnung nennt die Zahl. Viertens lösen vier entscheidbare Zielarten die laute Form aus: außerhalb des Benutzerordners, unmittelbar im Benutzerordner, Netzlaufwerk, und unter `~/Library/CloudStorage/` oder `~/Library/Mobile Documents` nach Auflösung der Verknüpfungen.

**In der zweiten Klärungsrunde** sind vier weitere Festlegungen dazugekommen. `ctrl+delete` in der Lesezeichenleiste bekommt keine Rückfrage und bleibt wie heute, weil dort ein Name und ein Pfad verloren gehen und keine Daten. Ein Ordner, der selbst ein `.git` enthält, warnt laut, auch bei wenigen Einträgen; nicht jeder Pfad innerhalb eines Arbeitsbaums warnt. Ein Protokoll der Löschvorgänge entsteht in dieser Runde nicht und wird ein vorgesehener Circle. Der Warngrund steht in der Frage und nicht in der Erläuterung, also „18 Einträge aus einem Cloud-Ordner in den Papierkorb räumen?"; die Erläuterung trägt Pfad und Folgen.

**Am 260817 hat der Nutzer nachgeschärft**, wörtlich: „Löschen OHNE Papierkorb wird entfernt: alle Datei/Folder-Löschvorgänge gehen immer in den Papierkorb." Damit ist die Antwort Möglichkeit 2.

**Ergänzend am selben Tag zu Zielen ohne Papierkorb:** KRK meldet dort in der Statuszeile, dass das Ziel keinen Papierkorb führt, und löscht nicht. Wer dort löschen will, nimmt den Finder. Der Nutzer hat das ausdrücklich der Alternative vorgezogen, den endgültigen Codeweg für diesen Fall zu behalten, und die Begründung mitgegeben: es soll keinen Weg durch KRK zum unwiederbringlichen Löschen geben. Möglichkeit 3 ist damit ausdrücklich abgelehnt.

**Nachtrag vom 260817, bei der Abnahme des Specs.** Eine der Festlegungen der zweiten Klärungsrunde ist umgedreht worden: die `.git`-Prüfung sieht nach der Abnahme auch aufwärts, jeder Pfad innerhalb eines Arbeitsbaums warnt laut. Der Satz oben bleibt als Aufzeichnung des damaligen Standes stehen; verbindlich ist `shared/decisions/260817-0536_a_sieht-die-git-pruefung-nur-den-ordner-selbst-oder-auch-aufwaerts.md`. Zwei Folgefragen sind bei derselben Abnahme beantwortet worden, `f8` zeigt künftig auf den Papierkorb und eine gespeicherte `keymap.toml` mit der entfallenen Kennung wird wie heute verworfen; die Datensätze tragen dieselbe Kennung `260817-0536`.

## Was diese Antwort aufhebt

Sie hebt `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md` vollständig auf. Jener Datensatz hält beide Hälften der Aussage „Delete löscht in Papierkorb, FN+F8 endgültig"; nach dieser Runde stimmt keine davon. Die Umbenennung auf `_s_` samt Zeile `Superseded by:` ist ein Planschritt dieser Runde und steht als Fähigkeit C6 im Spec, zusammen mit den fünf weiteren Stellen, die die alte Aussage tragen.

---
Answered: `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` — ein Löschweg in den Papierkorb, Rückfrage vor jedem Vorgang mit vorbelegtem Abbrechen, laute Form bei sechs Auslösern, keine Löschung auf Zielen ohne Papierkorb, `Kommando::EndgueltigLoeschen` fällt.
Implemented: `472eb81`, `ee85950`, `792995a`, `82707ef` — Möglichkeit 2 steht vollständig am Baum. `472eb81` (Schritt 3): jeder Papierkorbvorgang fragt genau einmal nach, `Anwendungsdelegierter::loeschen_nach_rueckfrage` (`crates/krk-ui/src/appkit/anwendung.rs:4621`). `ee85950` (Schritt 6): ein Ziel ohne Papierkorb wird gemeldet statt gelöscht, `papierkorb::fuehrt_einen_papierkorb` vor dem Blatt und `loeschwarnung::ohne_papierkorb` (`crates/krk-ui/src/kommandos/loeschwarnung.rs:411`) in der Statuszeile. `792995a` (Schritt 11): die laute Form, `laut` aus der Länge der Warngründe. `82707ef` (Schritte 12 und 13, ein Commit): `Kommando::EndgueltigLoeschen` und `Art::EndgueltigLoeschen` fallen — `grep -rn "EndgueltigLoeschen" --include="*.rs" crates` liefert am 260818 keine Zeile —, und `resources/default-keymap.toml` verliert den Eintrag `endgueltig_loeschen`. Bewegt in Schritt 16 des Plans `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`.
Deferred:
Superseded by:
