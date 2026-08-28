CLAUDE.md nennt für `Wirkungsbereich` sieben Werte; der Baum trägt acht

---

`CLAUDE.md:81` sagt: „Am 260825 nachgezählt: `Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) trägt sieben Werte". Seit `2aee690` (Runde 20, Schritt 2) trägt die Aufzählung acht: `Vorschau` steht zwischen `Navigator` und `Ueberall`, mit den drei Zoombefehlen als Trägern. Nachzuzählen mit `awk '/^pub enum Wirkungsbereich/,/^}/' crates/krk-core/src/tasten/belegung.rs | grep -cE '^\s+[A-Z][A-Za-z]*,'`.

**Die zweite Meldung der Coder, die Zahl der eigenen Textflächen sei veraltet, trifft nicht zu.** `CLAUDE.md:137` sagt „Es sind seit der Runde 14 zwei"; `Anwendungsdelegierter::ist_eigene_textflaeche` (`crates/krk-ui/src/appkit/anwendung.rs:2608-2620`) vergleicht weiterhin genau zwei Flächen, die des Editors und die Textanzeige der Vorschau. Der PDF-Betrachter ist dort nicht angemeldet, und das ist nach Constraint 6 des Specs richtig: `PDFView` ist keine `NSTextView`, kein `NSTextField` und kein `NSText`, also gehört sein Ersthelfer nicht AppKit, und eine Anmeldung wäre gegenstandslos. Die Zahl zwei bleibt richtig.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Betroffen:** `CLAUDE.md:81` (eine Zahl); kein Code
**Schwere:** Medium (normative Fläche, dieselbe Sorte Zahl ist in dieser Datei viermal in vier Tagen falsch geworden, `shared/issues/260812-2253_*`)

Der Plan der Runde 20 (`planning/260828-0712_*`, Entscheidung 4 und Risikotabelle) hat den Fehler vorausgesagt und dem Kurator zugewiesen, weil `curator` nicht in der Executor-Menge steht. Dieser Datensatz macht ihn für das Tor von `/fusion:cleanup` auffindbar. Fix: entweder „acht Werte" mit dem Datum des Nachzählens, oder die Zahl durch das `awk`-Kommando ersetzen, wie die Datei es für `Kommando` und `Art` schon tut. Der Absatz zu den zwei Textflächen bleibt unverändert.

Offener Nachbar: `shared/issues/260826-0149_o_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-an-der-vorschau.md` nennt nach dieser Runde eine Neuerung mehr (den Betrachter); dieser Datensatz hier betrifft allein die Zahl.
