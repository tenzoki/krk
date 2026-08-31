# Schritt 14: CLAUDE.md, die Runde 23 und die Absätze, die sie falsch macht

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 14
**Kriterien:** C9.10

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`.

---

## Die Erhebung über CLAUDE.md

Gefahren ist das Programm aus Schritt 11 mit dem Trägerwortsatz, den Schritt 12 erweitert hat,
also die Vereinigung beider Sätze: Zahlwörter von `zwei` bis `zwoelf` in Grund- und
Ordnungsform, mit und ohne Umlaut, dazu `genau ein…`, bis zu zwei Wörter Abstand, und als
Trägerwörter `Bereich`, `Fokus`, `Wirkungsbereich`, `Spalte`, `Schalter`, `Ankreuzfeld`,
`Beschriftung`, `Kasten`, `Rahmen`, `Teilbaum`, `NSBox`, `Fläche`, `Rang`, `Wert`,
`fokussierbar`, `Feld`, `Kästchen`, `Häkchen`, `Knopf`, `Zelle`, `Umschalter`, `Sichtbarkeit`,
`Breite`. Das Zwei-Zeilen-Fenster der Vorlage bleibt wirkungslos, weil in dieser Datei jeder
Absatz eine Zeile ist.

| Erhebung | Treffer |
|---|---|
| vor der Arbeit | 7 in 4 Zeilen |
| nach der Arbeit | 5 in 3 Zeilen |

**Die fünf verbliebenen Treffer sind gelesen und richtig**: die neue Tabellenzeile 23 („sechster
Bereich"), `:81` („der Editor ist der fünfte Bereich der Fensterzeile", von Schritt 11 schon
bestätigt), die zwei Aussagen zu `Kontextbefehl` und `Art` in `:141` und der neue Satz in `:145`
über den sechsten Bereich.

**Der Ausführungszweig zu `Kontextbefehl` ist nachgeschlagen und nicht angenommen.**
`anwendung.rs:6618-6620` führt drei Zweige und keinen Auffangzweig; „ein vierter Wert hält damit
den Bau an" bleibt wahr. Der Defekt `shared/issues/260831-1212_o_kontextmenue-rs-behauptet-eine-feldbreite-…`
betrifft die Feldbreite in `kontextmenue.rs` und nicht diese Aussage.

---

## Was angefasst ist

**1. Die Rundentabelle** bekommt die Zeile 23 mit dem Circle-Verzeichnis und dem Gegenstand, in
der Form der 22 Zeilen davor.

**2. Die Baumskizze**: `crates/krk-core/` nennt jetzt `git/` neben `text/` und
`stapelumbenennen/`.

**3. `:81`, die drei Anzeigen der Editor-Runde.** „gelten allen fünf Bereichen" ist gefallen; der
Satz sagt jetzt „gelten über den Editor hinaus" und trägt keine Zahl. Er war doppelt unrichtig:
die Bereiche sind sechs, und die Zeilennummern gelten ohnehin nur zweien von ihnen, was die
Klammer im selben Satz schon sagte.

**4. Der Absatz über die gewachsenen Aufzählungen.** Er nennt `Bereich`, `Fokus` und neu
`Spalte`, und für alle drei steht **keine Zahl** mehr, sondern der Zählbefehl
`awk '/pub enum Bereich/,/^}/' …`. Der Grund steht daneben und ist die eigene Regel der Datei:
die Zahl fünf war am 260825 nachgezählt, am 260829 unverändert und am 260831 falsch. Die
Kopfzeile behauptet nicht mehr, jede dieser Aufzählungen halte den Bau an, wenn eine Stelle
fehlt; sie sagt, dass der Bau an den ausgeschriebenen Fallunterscheidungen hält, und der Schluss
des Absatzes verweist für den Rest auf den neuen Absatz unter „Was man nicht sieht".

**5. Der Absatz zu `syntect`, `two-face` und `zip`** bekommt `gix` mit dem Unterschied, der ihn
trägt: hier ist `default-features = false` Sparsamkeit und nicht die Bedingung der Einbindung,
weil der Vorgabesatz von `gix` kein C hereinzöge. Dazu die fünf gewählten Merkmale, der Hinweis,
dass `max-performance` / `max-pure` in 0.87 nichts mehr umschaltet, und die Begründung der
festgenagelten Fassung `"0.87"`. Die C-Freiheits-Zeile desselben Absatzes ist **nicht** angefasst;
sie steht, wie Schritt 13 sie gesetzt hat.

**6. Der Absatz über den Ereignisabgriff bleibt bei zwei und sagt es.** Die Runde 23 hat einen
sechsten Bereich gebaut und trotzdem keine dritte eigene Textfläche angemeldet: die Fläche der
Einzelheiten ist ein `NSTextField` als mehrzeiliges Etikett, nimmt den Ersthelferrang nicht an,
und die Frage nach der Anmeldung stellt sich für sie nicht. Der Preis, kein Markieren und kein
Kopieren, steht dabei. Beleg im Baum: `ist_eigene_textflaeche` (`anwendung.rs:2774-2787`) prüft
weiter genau zwei Flächen, und der Modulkopf von `appkit/git.rs` schreibt den Grund aus.

**7. `:145`, die zwei Stellen, die Schritt 11 namentlich liegengelassen hat.** „die fünf
Rahmenfarben der Aufteilung" und „eine reine Funktion über die fünf Fokuswerte" sind beide
gefallen. Statt der neuen Zahl steht die zahlfreie Form: die Klammer sagt ohnehin „ein `NSBox` je
Bereich", und der Fenstertitel läuft über „die Werte von `Fokus`". Nachgeschlagen:
`Aufteilung::rahmen` ist `[Retained<NSBox>; 6]`, `fenstertitel::titel` verzweigt über sechs Werte.

**8. Die Git-Anbindung liegt nicht mehr außerhalb aller Runden.** Der Absatz unter „Bindende
Grundlage" führte sie in einer Reihe mit der KI-Anbindung und dem Browser und stützte das auf
„am 260815 trägt `Kommando` keine einzige Git-Variante". Er sagt jetzt, dass die lesende Stufe A
gebaut ist und die vier Operationen des Kurztexts (Hinzufügen, Committen, Verwerfen,
Versions-Schieberegler) weiter ausstehen.

**9. Neu unter „Was man nicht sieht": die Bauformen einer `ALLE`-Liste.** Vier Zeilen: `ALLE.map`
hält den Bau, ein Literal und ein `[0.0; 6]` halten ihn nicht und laufen beim Start auf
`index out of bounds`, ein fester Parameter hält gar nichts. Dazu der Preis dieser Runde, neun
von Hand nachgezogene Stellen ohne eine einzige Meldung des Übersetzers, und der Zeiger auf die
offene Nutzerfrage `shared/decisions/260826-1811_*`. Der Absatz steht hinter dem zu
`Kontextbefehl`, weil jener sich rückwärts auf „den Absatz darüber" beruft.

---

## Eine Aussage, die ich zuerst falsch geschrieben hatte

Der erste Entwurf von Punkt 8 schrieb: „Dass kein Weg im Baum ins Repository schreibt, hält die
Erhebung `grep -rn 'write_changes' crates/`, die keine Fundstelle liefern darf." Nachgeprüft
liefert sie **zwei** Fundstellen, beide Modulköpfe unter `krk-core/src/git/`, die ausschreiben,
dass sie es nicht rufen. Der Satz ist ersetzt durch die Aussage, die stimmt: `krk-core/src/git/`
ruft `Outcome::write_changes` an keiner Stelle. Der Widerspruch zwischen C3.8 („null Treffer")
und C10.3 („Treffer, die die Lesestelle nennen") ist als
`issues/260830-1614_o_c3-8-verlangt-null-treffer-…` schon gefilt und bleibt offen; dieser Schritt
hat ihn nur nicht in CLAUDE.md hineingeschrieben.

---

## Bewusst stehengelassen

- **Der Absatz über die Ablage** (`Datei::ALLE`, `~/Library/Application Support/KRK/`): diese
  Runde legt keine achte Ablagedatei an.
- **Der Kurztext „Worum es geht"**, der die auf vier Operationen beschränkte Git-Anbindung nennt:
  er ist die Kurzfassung der Directive der Runde 1 und beschreibt das Ziel, nicht den Stand.
- **Der Untergrenzen-Absatz** („in jeder Datei unter `crates/krk-ui/src/appkit/` außer zweien"):
  nachgezählt, `appkit/git.rs` trägt den Abschnitt, und ohne ihn stehen weiter allein
  `koordinaten.rs` und `mod.rs`.
- **Die `unsafe_code`-Zeile** und **„Workspace mit vier Mitgliedern"**: beide nachgeschlagen und
  von dieser Runde nicht berührt.
- **Die C-Freiheits-Zeile in `:87`**: Schritt 13, hier nicht ein zweites Mal angefasst.

---

## Was schon vor dieser Runde nicht mehr stimmte

Beim vollständigen Lesen der Datei ist mir keine weitere unrichtige Aussage aufgefallen. Die zwei
Stellen, die es waren, `:145` mit „fünf Rahmenfarben" und „fünf Fokuswerte", sind seit Schritt 1
dieser Runde unrichtig und damit ihr Werk, nicht älter. Die Zahl fünf für `Bereich` und `Fokus`
in `:85` trug einen Stempel („am 260825 nachgezählt und am 260829 unverändert") und war zum
Zeitpunkt ihres Stempels richtig.

---

## Dateien

`/Users/k1/Projects/productive/krk/CLAUDE.md` — 11 hinzugefügte, 7 entfernte Zeilen. Keine
weitere Datei angefasst, keine Codezeile, keine Belegung, keine der drei Aufzeichnungen aus
Schritt 15.

**Kein whole-tree-git-Kommando abgesetzt.** Kein Commit; der Orchestrator committet.
