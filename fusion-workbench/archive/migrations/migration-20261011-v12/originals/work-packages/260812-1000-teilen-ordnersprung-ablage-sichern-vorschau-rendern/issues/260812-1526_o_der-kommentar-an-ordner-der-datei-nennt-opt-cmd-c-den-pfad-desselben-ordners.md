Der Kommentar an `ordner_der_datei` sagt, `opt+cmd+c` kopiere den Pfad desselben Ordners — es ist ein anderer

---

`resources/default-keymap.toml:250-251` begründet die Wahl von `opt+cmd+o` mit der
Nachbarschaft zu `opt+cmd+c`:

> Dieser Befehl liefert einen Ordner und steht deshalb hier; er ist der unmittelbare
> Nachbar von opt+cmd+c, das den Pfad **desselben Ordners** kopiert.

Die beiden Befehle sprechen nicht über denselben Ordner. `opt+cmd+c`
(`ordnerpfad_kopieren`) nimmt den Ordner, den das aktive Dateifenster **anzeigt**:
`Tabelle::ordnerpfad_kopieren` liest `self.angezeigter_ordner()`
(`crates/krk-ui/src/appkit/tabelle.rs:964-966`). `opt+cmd+o` (`ordner_der_datei`) nimmt
den Ordner **über der angezeigten Datei**, und die kommt aus Vorschau oder Editor:
`Anwendungsdelegierter::ordner_der_datei_zeigen` bildet `datei.parent()` aus dem Pfad,
den `angezeigtedatei::welche` liefert (`crates/krk-ui/src/appkit/anwendung.rs:2393-2404`,
`crates/krk-ui/src/angezeigtedatei.rs:46-58`).

Beide fallen genau dann zusammen, wenn die angezeigte Datei schon im angezeigten Ordner
liegt — und dann ist der Sprung ein Leerlauf. Der Befehl besteht für den anderen Fall.
Die Zusage im Kommentar ist damit in genau der Lage falsch, für die es den Befehl gibt.

---

**Herkunft:** Directive dieser Runde, Commit `95b2dfa` (Schritt 2 des Plans
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`).

**Woher der Satz stammt:** wörtlich aus Möglichkeit 1 des Entscheidungsdatensatzes
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`
und aus dessen Antwortabschnitt vom 260812-1105. Der Datensatz ist als umgesetzt
geschlossen und trägt denselben Fehler; die Wahl der Kombination hängt nicht daran,
die Begründung im Kommentar schon.

**Nicht betroffen:** die Kombination selbst. `opt+cmd+o` war frei (am 260812-1526
über alle `tasten`-Listen nachgezählt), und der Platz des Blocks neben `ordner_aufwaerts`
stimmt.

**Empfehlung:** die Halbzeile berichtigen, ohne die Begründung zu verlieren. Der
tragende Zusammenhang ist die Sorte Handlung, nicht die Selbigkeit des Ordners, etwa:
„… der unmittelbare Nachbar von `opt+cmd+c`, das einen Ordnerpfad in die Zwischenablage
gibt, wo dieser Befehl einen Ordner in die Liste holt." Den Entscheidungsdatensatz
dabei nicht rückwirkend umschreiben: er zeichnet den Stand vom 260812-1105 auf
(Ortsregel für Aufzeichnungen, `CLAUDE.md`, Abschnitt „Bindende Grundlage").
