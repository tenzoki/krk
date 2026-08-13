# Planner: Nachtrag zum Plan der Runde 7 nach der dritten Diagrammprüfung

**Datum:** 260813-0245
**Agent:** planner (user-direct, autonom — keine Rückfrage an den Nutzer)
**Anlass:** Spruch `acceptable` der Diagrammprüfung `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/reviews/260813-0220-conceptrev-plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`, ein Befund vor der Ausführung und fünf mitlaufende

## Was geändert wurde

Eine Datei, Ort und Name unverändert, Marker `_o_` unverändert:
`circles/260813-0100-…/planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`.

Kopfzeile `**Diagrammprüfungen:**` um die dritte Prüfung erweitert, angehängter Abschnitt `## Nachtrag vom 260813-0233: was die dritte Diagrammprüfung geändert hat`. Die Historie ist nicht umgeschrieben; der Abschnitt zu den ersten zwei Prüfungen steht als Aufzeichnung seines Standes und trägt nur einen Verweis auf den Nachtrag.

Kein neuer Entscheidungsdatensatz, kein neuer Defekt. Die Begründung dafür steht unten.

## Der Befund vor der Ausführung

**Die Zahl drei war falsch, und der Plan hat sie an drei Stellen als Zusage geführt.** Am Baum nachgezählt statt vom Prüfer übernommen: `ersthelfer_gehoert_appkit` hat heute genau eine Aufrufstelle, `crates/krk-ui/src/appkit/ereignisse.rs:488`, dazu die Erklärung ab `:536`. Nach dem alten Entwurf von S2 und S6 wären es zwei geworden, denn `kommando_ausfuehren` und `validateMenuItem:` rufen beide `self.lage()`.

**Auf zwei gesetzt worden ist die Probe trotzdem nicht.** Eine Aufruferzahl ist gegen die Zusage „eine Frage, kein Doppelbau" in beide Richtungen blind: eine zweite Klassenprüfung an anderer Stelle lässt sie grün, ein weiterer berechtigter Frager lässt sie rot, und der billigste Weg zurück ins Grüne wäre dann das Streichen eines Frägers.

**Der Baum liefert die richtige Bauform selbst.** Drei Prüfungen lesen heute den Quellbaum: `es_gibt_genau_einen_menuebauer` und `allein_diese_datei_baut_den_freigabewaehler` (`crates/krk-ui/src/appkit/teilen.rs:414-470`) und `das_vorschaumodell_weiss_von_der_einfaerbung_nichts` (`crates/krk-ui/src/appkit/vorschau.rs:1240-1263`). Alle drei zählen Erklärungen, Dateien oder das Vorkommen eines Namens; keine zählt Aufrufer. S2 nimmt dieselbe Form: `fn ersthelfer_gehoert_appkit` genau einmal im Baum, `isKindOfClass(` in genau einer Datei.

## Die Änderung am Entwurf

**Der Abgriff fragt nach dieser Runde nicht mehr nach dem Ersthelfer.** Die alte Fassung von S2 baute an die Stelle des frühen Ausstiegs eine ausdrückliche Wache im Zweig `Nachschlag::Sprungmarke`, also eine zweite Stelle mit derselben Frage in einer Runde, die zweite Stellen einspart. Die neue Fassung reicht beide Ausgänge an die Senke und lässt `lage()` die drei Eingaben einmal erheben.

Nachgerechnet, mit Fundstellen:

- Der Zweig `Eingabe::Zeichen` von `eingabe_ausfuehren` fragt heute schon `blatt_steht()` und `fokus()` (`crates/krk-ui/src/appkit/anwendung.rs:2064-2088`) und bekommt allein den dritten Wert dazu.
- Das Verhalten bleibt in allen drei Ausgängen des Nachschlags gleich. Der Preis ist nachgerechnet: ein Tastendruck bis zur Senke kostet drei Eigenschaftsabfragen wie heute, ein Tastendruck in ein Textfeld drei statt einer plus einen Nachschlag in der Belegung, eine unbelegte Kombination eine weniger. Die Bedeutung für L1 ist eine Größenordnungsschätzung und keine Messung. `inference:`
- `Tastenabgriff::einrichten` verliert den Parameter `ist_editorflaeche` (eine Aufrufstelle, `anwendung.rs:1740`), `abgriff_aufsetzen` eine schwache Referenz, und `ereignisse.rs` bekommt den Editor nicht mehr hereingereicht. Modulkopf und `CLAUDE.md` sagen beide, dass die Datei den Editor nicht kennen soll; danach stimmt der Satz ohne Einschränkung.
- `ersthelfer_gehoert_appkit` behält genau eine Aufrufstelle, `lage()`. Erklärungszählung und Aufruferzählung liefern damit dasselbe Ergebnis, und die Zusage braucht keinen Stellvertreter mehr.

## Die fünf übrigen Befunde

Alle abgearbeitet, keiner zurückgewiesen. Die Zuordnung steht als Tabelle im Nachtrag des Plans.

Beim Nachlesen zu Befund 2 ist ein Fehler aufgefallen, den die Prüfung nur streift: S12 verwies für die Aufrufstellen von `Ablage::laden` und `Ablage::sichern` auf „die fünf Stellen aus dem Aufrufbild". Das Bild zeigt die fünf Benutzer von `Ablage::durchgang`, die Aufrufstellen der zwei Methoden sind sechs und eine andere Menge. S12 führt sie jetzt einzeln mit Pfad und Zeile auf.

## Drei Punkte aus dem Nachlesen am Baum

- Der Protokollmodus `--tasten-protokoll` zeigt nach S2 auch Tastendrücke in ein Textfeld, weil `protokollieren` hinter dem frühen Ausstieg steht.
- `quelldateien` und `einsammeln` sind privat im Prüfmodul von `teilen.rs:374-412`; die Runde braucht sie in mindestens drei weiteren Prüfmodulen. S2 gibt ihnen einen gemeinsamen Ort, statt sie dreimal abzuschreiben.
- `sprungmarke_tippen` bleibt unverändert (`crates/krk-ui/src/appkit/tabelle.rs:1134-1147`); sein `false` ist keine Ausnahme von der Regel aus S3, sondern ihre Anwendung.

## Warum kein Entscheidungsdatensatz

Die Wahl zwischen Erklärungszählung und Aufruferzählung sähe nach einer projektweiten Konvention aus und wäre dann ein Datensatz. Nachgezählt bindet sie nichts Neues: die drei vorhandenen Baumleseproben zählen bereits Erklärungen, und die Drei in S2 war der Ausreißer. Der Plan beschreibt damit die bestehende Praxis, statt eine neue zu setzen.

## Werkzeug

Alle vier Mermaid-Blöcke sind nach der Änderung mit `mmdc` 11.16.0 über `npx` nach PNG gerendert und angesehen. Das erste Bild ist in seiner ersten Fassung sperrig geraten, weil `WEITER` im Kasten des Abgriffs lag und die Kanten aus dem Delegierten dorthin zurückliefen; der Knoten steht jetzt zwischen den Kästen, und die drei Schichten liegen sauber untereinander. Die Kante vom bedienbaren Menüeintrag führt bewusst auf den Rumpf und nicht auf die Raute `A1`, sonst zeigte der Graph einen Ring, den es der Sache nach nicht gibt.

Kein `cargo`-Lauf, kein Vordergrundlauf, kein Bündelbau. `target/KRK.app` ist unangetastet.
