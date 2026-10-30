# Die Ersatzangaben der Vorschau zeigen die Zeilenzahl einer Tabelle als Bytes

---
Scheitert das Lesen einer Tabelle in der Vorschau, erscheinen die Angaben des Eintrags. Deren Zeile „Größe“ formatiert `Eintrag::groesse` als Bytes, und in einer Datenbank ist das die Zeilenzahl. Eine Tabelle mit 1 234 Zeilen zeigt dann „1 KB“, eine Sicht „0 Bytes“.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Medium
**Cross-references:** 261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.md, 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md (Schritt 4 und 6)

## Befund

- `crates/krk-core/src/verzeichnis/leser.rs`, `archiveintrag_zeigen`: `Eintrag::mit_versteckt(name, eintrag.groesse, …)`, also die Zeilenzahl.
- `crates/krk-ui/src/appkit/tabelle.rs`, `archivangaben_des_eintrags`: `groesse: Some(eintrag.groesse)`.
- `crates/krk-ui/src/appkit/vorschau.rs`, `archiv_anzeigen`: `let ersatz = self.angabenzeilen(&eintrag.angaben)`. `angabenzeilen` schreibt `Text::MetadatenGroesse` über `groesse_text`, eine Byteangabe.
- `crates/krk-ui/src/vorschaumodell.rs`, `archiveintrag_laden`: jeder Lesefehler endet in `Inhalt::Hinweis(ersatz)`. Erreichbar unter anderem über `Verzeichnisfehler::Belegt` (Schreiber hält die Sperre über `WARTEZEIT` hinaus) und `Lesefehler::VonAussenVeraendert` (Schreiber übergibt während des Lesens).

Die Erhebung aus Schritt 4 des Plans (`grep -n '\.groesse'` über drei Dateien des Kerns) erreicht `krk-ui` nicht. Die Regel in `.claude/rules/coding-was-man-nicht-sieht.md` nennt für die Oberfläche allein `groessenzelle` und `masseinheit`.

## Erwartet

Die Ersatzangaben eines Eintrags einer Datenbank nennen die Zeilen mit `Zahlwort::SqliteZeilen` oder lassen die Größe weg. Entschieden wird am Ort (`datenbankort`), nicht an einem Systemaufruf.

## Abnahme

Eine Probe baut die Ersatzangaben für eine Tabelle und eine Sicht einer Datenbank und prüft, dass keine Byteangabe aus der Zeilenzahl entsteht. Dazu eine Erhebung aller Leser von `Eintrag::groesse` in `crates/krk-ui/src` in einem Archivtab, mit Einordnung jedes Treffers in der Regeldatei.

**Resolved:** 261024, code-implementer. `Fernangaben::groesse` trägt die Einheit am Wert (`Groessenangabe::Bytes`/`Zeilen`, `crates/krk-ui/src/vorschaumodell.rs`); `archivangaben_des_eintrags` nennt in einer Datenbank die Zeilen einer Tabelle aus dem gespeicherten Verzeichnis (`datenbankzeilen_am`, dieselbe Quelle wie die Größenspalte) und für eine Sicht keine Größe, `angabenzeilen` schreibt Zeilen mit `Zahlwort::SqliteZeilen`. Probe `die_ersatzangaben_einer_tabelle_nennen_zeilen_und_einer_sicht_keine_groesse` (`appkit/tabelle.rs`). Die Erhebung der Leser von `Eintrag::groesse` in `crates/krk-ui/src` steht im Bericht des Laufs; die Einordnung in der Regeldatei `.claude/rules/coding-was-man-nicht-sieht.md` ist nicht geschrieben.
