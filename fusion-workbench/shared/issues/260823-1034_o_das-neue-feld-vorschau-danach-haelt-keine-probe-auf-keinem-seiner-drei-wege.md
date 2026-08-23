Das neue Feld `vorschau_danach` hält keine Probe, auf keinem seiner drei Wege

---

`28cbb7b` führt `Anlass::EditorSchliessen { vorschau_danach: bool }` ein und begründet das Feld
im Doc-Kommentar mit einer Zusage, die niemand prüft: „sagt der Nutzer ‚Abbrechen', bleibt der
Editor stehen, und die Vorschau darf ihn dann gerade nicht verdraengen."

`grep -rn vorschau_danach crates` liefert sechs Treffer, alle in
`crates/krk-ui/src/appkit/anwendung.rs`, keiner in einem Prüfmodul.

---

**Am Baum gelesen.** Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit.

## Die drei ungeprüften Zusagen

1. **`opt+cmd+e` übergibt `false`** (`anwendung.rs:3158`). Ein `true` dort ließe die alte
   Kombination die Vorschau einblenden — eine Bedeutungsänderung an einer ausgelieferten
   Kombination einer abgenommenen Runde, also genau das, was der Kommentar bei
   `editor_umschalten` in `resources/default-keymap.toml:862-871` ausschließt.
2. **Der Rückweg übergibt `true`** (`anwendung.rs:7032`).
3. **`anlass_unterbleibt` liest das Feld nicht** (`anwendung.rs:6924`: `Anlass::EditorSchliessen
   { .. } => {}`). Wer dort später `if vorschau_danach { … }` ergänzt, dreht die Wahl des
   Nutzers vom 260823-0942 um, und nichts wird rot.

Der Übersetzer hält keine der drei. Ein `bool` an einer Aufrufstelle vertauscht übersetzt; ein
`{ .. }`, das zu `{ vorschau_danach }` wird, auch.

## Warum das hier prüfbar ist, obwohl AppKit im Spiel ist

Dieser Baum hat für genau diese Lage ein Werkzeug: die Quelltextproben. `sichtbarkeitsproben`
(`anwendung.rs:8244`) und `fokusnachzugproben` (`anwendung.rs:8164`) lesen den eigenen Quelltext
über `crate::quellbaum::quelldateien` und halten Zusagen fest, die „ohne Fenster nicht messbar,
aber am Baum ablesbar" sind. Der Doc-Kommentar von `sichtbarkeitsproben` sagt den Grund selbst:
„Was ohne Fenster pruefbar bleibt, ist die Verdrahtung."

Die drei Zusagen oben sind Verdrahtung. Eine Probe in derselben Form — der Rumpf von
`anlass_unterbleibt` nennt `vorschau_danach` nicht; der Zweig `Kommando::EditorSchliessen` ruft
`editor_schliessen(false)`; `Rundweg::ZurueckInDieDateiliste` ruft `editor_schliessen(true)` —
kostet wenige Zeilen und schließt die Lücke, die der Übersetzer offen lässt.

## Verhältnis zur Nachbarschaft

`crates/krk-ui/src/kommandos/rundweg.rs` ist vorbildlich geprüft: sechs Proben, darunter eine
Aufruferzählung und eine, die den Wirkungsbereich gegen die Regel hält. Die Lücke liegt
ausschließlich dort, wo die reine Regel in den Delegierten übergeht — also genau an der Naht, an
der `rueckschritt.rs` mit seiner Vorlage dieselbe Frage schon einmal beantwortet hat.

**Schwere:** Medium.

**Filed by:** coderev
