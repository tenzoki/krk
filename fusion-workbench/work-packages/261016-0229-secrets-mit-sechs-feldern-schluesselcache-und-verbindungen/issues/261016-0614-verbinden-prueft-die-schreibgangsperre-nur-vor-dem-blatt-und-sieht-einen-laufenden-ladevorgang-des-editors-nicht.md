„Verbinden…“ prüft die Schreibgangsperre nur vor dem Blatt und sieht einen laufenden Ladevorgang des Editors nicht
---
`Editormodell::schreibgang_gesperrt` antwortet über den Stand, den der Editor schon übernommen hat. Läuft beim Anstoß von „Verbinden…“ noch das Festlegen einer PIN für eine leere `secrets.txt` (Ableitung auf dem Arbeitsfaden), antwortet sie über die vorige Datei, das Blatt geht auf, und danach fragt niemand mehr. Damit entsteht über ein enges Zeitfenster derselbe Zustand, den `261016-0532-verbinden-an-einer-leeren-im-editor-offenen-secrets-txt-legt-eine-zweite-pin-fest.md` behoben hat: zwei PINs, zwei Schlüssel, ein Editor, der seine Datei nicht mehr öffnet.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0532-verbinden-an-einer-leeren-im-editor-offenen-secrets-txt-legt-eine-zweite-pin-fest.md, 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Entscheidungen 6 und 15)

## Defekt

Am Quelltext auf `9d1b111` gelesen; der Ablauf ist **inference** aus den Zweigen und nicht am Bündel gefahren.

- Die Sperre wird genau einmal gefragt, in `verbindung_erfragen` (`crates/krk-ui/src/appkit/anwendung.rs:9080-9088`), bevor das Blatt aufgeht. `verbindungsschluessel_holen` (`:9121-9159`) und `verbindung_anhaengen` (`:9163-9176`) fragen sie nicht wieder, obwohl zwischen ihnen das Blatt „Verbinden“ und gegebenenfalls das PIN-Blatt stehen.
- `schreibgang_gesperrt` (`crates/krk-ui/src/editormodell.rs:1331`) liest `self.abweichung` und `self.schutz`. Beide setzt allein `uebernehmen` am Ende eines Ladevorgangs (`editormodell.rs:1674-1677`). Während der Ableitung beim Festlegen gilt noch der Stand der vorigen Datei, und `laedt_noch` (`editormodell.rs:1740`) fragt die Sperre nicht.

Ablauf: `f4` auf eine leere `secrets.txt`, PIN P1 festgelegt, Argon2 läuft auf dem Faden. In dieser Spanne steht kein Blatt, das Kontextmenü ist erreichbar. „Verbinden…“ sieht den Editor noch mit der vorigen Datei, die Sperre sagt `None`, das Blatt geht auf. Der Editor übernimmt die leere Datei mit K1 und `kopf_auf_platte: false`. Nach „Anlegen“ ist die Datei auf der Platte weiter leer, also verlangt `geheimnisweg` ein zweites Festlegen (P2, Salz S2), und der Gang schreibt mit K2. `anhaengen_einziehen` lädt den Editor mit K1 neu, und das scheitert (`anwendung.rs:9245-9250`). Weiter wie in Schritt 4 und 5 des zitierten Datensatzes.

Das Zeitfenster ist so lang wie eine Ableitung auf dem Referenzgerät; die Schwere ist deshalb Low, die Folge ist dieselbe wie im behobenen Befund.

## Richtung der Behebung

Eine Regel, an zwei Stellen gefragt statt an einer:

1. `schreibgang_gesperrt` sperrt auch, solange ein Ladevorgang läuft, dessen Ziel `secrets.txt` nennt (dritte Lage neben `Ungesichert` und `OhneKopf`, mit eigenem Satz).
2. `verbindung_anhaengen` fragt die Sperre unmittelbar vor dem Start des Schreibgangs ein zweites Mal und meldet sie wie `verbindung_erfragen`. Die Frage vor dem Blatt bleibt, damit der Nutzer nicht erst Felder füllt.

## Akzeptanz

- Eine Probe am Modell hält: zwischen `oeffnen` einer leeren `secrets.txt` mit PIN und dem Einziehen ihres Ausgangs antwortet `schreibgang_gesperrt` nicht `None`.
- Eine Quelltextprobe hält, dass `verbindung_anhaengen` die Sperre vor `verbinden::anhaengen` fragt (etwa als weitere Nadel in `der_verbindungsweg_reisst_nirgends_ab`).
- `make check` grün.

---
Resolved: `Editormodell::schreibgang_gesperrt` (`crates/krk-ui/src/editormodell.rs`) sperrt mit der dritten Lage `Schreibgangsperre::Laedt` (Satz `Text::VerbindenEditorLaedt` in `schluessel.rs`, `ALLE` 649 -> 650, und den drei Tabellen), solange ein Ladevorgang `secrets.txt` liest (neues Feld `Ladevorgang::geheimnisse`) oder eine gelesene verschluesselte Datei fuer die Nachfrage aus C4 zurueckgehalten wird; geprueft vor `haelt_geheimnisse`, also auch bei einer zuvor gehaltenen gewoehnlichen Datei. `Anwendungsdelegierter::verbindung_anhaengen` (`crates/krk-ui/src/appkit/anwendung.rs`) fragt die Sperre ein zweites Mal unmittelbar vor `verbinden::anhaengen`. Proben: `ein_laufendes_lesen_von_secrets_txt_sperrt_den_schreibgang` (editormodell.rs) und eine weitere Nadel samt Reihenfolgepruefung in `der_verbindungsweg_reisst_nirgends_ab`. `HowTo.md`, Abschnitt „Eine Verbindung anlegen“: dritte Zeile der Tabelle und der Satz zur zweiten Pruefung.
