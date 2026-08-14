# Der Editor-Einstieg braucht ein erreichbares Kürzel neben F4

**Filed by:** k1

`F4` öffnet den ausgewählten Eintrag des Dateifensters im eingebauten Editor, und die Taste ist in der Bedienung hakelig. Gebraucht wird eine zweite, besser erreichbare Kombination für dieselbe Funktion.

**Was der Baum heute trägt.** Drei Wege führen zum Editor, und nur einer davon hat kein Kürzel: `bearbeiten` steht auf `f4` allein (`resources/default-keymap.toml`), `editor_aus_vorschau` auf `cmd+e`, `fokus_editor` auf `shift+cmd+e`. Die `e`-Familie ist also schon angelegt, und die `opt+cmd`-Ebene ist mit `opt+cmd+n`, `opt+cmd+c` und `opt+cmd+g` in Gebrauch.

**Was eine Runde darüber zu klären hätte.** Der Kommentar an `bearbeiten` in der Belegungsdatei begründet ausdrücklich, dass es **kein** Cmd-Kürzel daneben braucht: der zweite Einstiegsweg sei keine zweite Tür auf dieselbe Handlung, sondern eine eigene Funktion mit eigener Quelle. Ein Nachtrag kippt diese Überlegung und muss sie ersetzen, nicht übergehen.

**Eine Vermutung zur Ursache, ungeprüft.** `F4` ist auf Apple-Tastaturen ab Werk mit Spotlight beziehungsweise Launchpad belegt; KRK sieht die Taste nur, wenn in den Systemeinstellungen „F1, F2 usw. als Standard-Funktionstasten verwenden" eingeschaltet ist. Trifft das zu, ist die Hakeligkeit keine Eigenschaft von KRK, und eine Runde sollte das zuerst messen, bevor sie eine Kombination wählt.

**Related:** `resources/default-keymap.toml` (Eintrag `bearbeiten`); `circles/260807-2116-eingebauter-editor-mit-textmarken` (die Runde, die den Editor gebaut hat)
