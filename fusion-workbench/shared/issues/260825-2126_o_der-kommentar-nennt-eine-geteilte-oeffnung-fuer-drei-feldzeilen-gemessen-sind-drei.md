Der Kommentar nennt eine geteilte Öffnung für drei Feldzeilen, gemessen sind es drei

---

`resources/default-readers.toml:253` begründet, warum die tote Zeile „Projekt" stehen bleibt,
unter anderem damit, „weil sie sich ihre Öffnung mit den zwei Zeilen darunter teilt, also
nichts kostet". Das ist falsch. Ein Feldbaustein öffnet seine Datei immer selbst; drei
Feldzeilen über derselben `.fusion-setup` kosten drei Öffnungen und nicht eine. Dieselbe
Datei sagt es zweiunddreißig Zeilen weiter oben richtig
(`resources/default-readers.toml:220-222`: „zwei Feldbausteine über derselben Datei öffnen
sie zweimal").

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:220-222` und `:248-254`;
`crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_OEFFNUNGEN`, Doc-Kommentar);
`crates/krk-core/src/leseprofil/bausteine.rs` (Modulkopf, „Bei der Dateioeffnung faellt die
Wahl anders aus"); `shared/issues/260825-2044_o_die-zeile-projekt-…` (Möglichkeit 1)

## Was der Mechanismus sagt

Gemerkt wird der **Ort** und nicht die Datei. `Lauf` merkt jede Verzeichnislesung nach ihrem
aufgelösten Ort, so dass drei Zeilen über demselben Ordner einen Leselauf teilen. Für die
Dateiöffnung fällt die Wahl ausdrücklich anders aus, und der Doc-Kommentar von
`HOECHSTENS_OEFFNUNGEN` schreibt sie aus: „Eine Datei, die zwei Bausteine desselben Profils
lesen, wird zweimal geoeffnet. Das ist gewollt: so ist die Zahl der Oeffnungen aus dem Profil
ablesbar, naemlich eine je Feldbaustein."

## Was gemessen ist

Gemessen am 260825-2126 über `leseprofil::zusammenfassen_gezaehlt`, Baum `8478753`, gegen die
Werkbank dieses Projekts:

```
fusion-workbench            Leseläufe=3  Öffnungen=4
```

Das Profil „fusion-Werkbank: die Wurzel" führt fünf Feldzeilen: drei über `.fusion-setup`,
eine über `.active-circle`, eine über `orchestrator-live.md`. `.active-circle` steht an dieser
Werkbank nicht, kostet also null. Vier Öffnungen bei vier vorhandenen Zielen heißt: 3 + 0 + 1.
Teilten die drei sich eine, stünden dort zwei.

Gegenprobe an einem künstlichen Ordner mit allen drei Dateien
(`scratchpad/t2/a`, `.fusion-setup` und `orchestrator-live.md` vorhanden, `.active-circle`
nicht): ebenfalls vier Öffnungen bei einem einzigen Leselauf.

## Warum das zählt

Der Satz ist eine der zwei Begründungen dafür, dass eine Zeile stehen bleibt, die heute nichts
mehr liefert. Die andere Begründung — eine ältere Werkbank trägt das Feld noch — trägt für
sich. Die Kostenbegründung trägt nicht, und sie steht als „kostet nichts" da, wo in Wahrheit
eine von vierundzwanzig Öffnungen anfällt. Wer nach ihr rechnet, hält ein Profil mit sechs
Feldzeilen über einer Datei für so teuer wie eines mit einer.

**Derselbe Satz steht ein zweites Mal**, in `shared/issues/260825-2044_o_die-zeile-projekt-…`
unter „Möglichkeiten", Punkt 1: „Sie kostet eine Öffnung, die sie sich mit zwei anderen Zeilen
teilt, also nichts". Wer den einen berichtigt, berichtigt den anderen mit.

## Was zu tun wäre

Den Halbsatz in `resources/default-readers.toml:253` streichen oder berichtigen. Die Zeile
kostet eine Öffnung von vierundzwanzig — wenig, aber nicht nichts —, und der Grund, sie stehen
zu lassen, ist die ältere Werkbank und nicht der Preis. Dieselbe Berichtigung in
`260825-2044`.

**Schwere:** mittel. Eine Aussage im Handbuch, die der Datei an anderer Stelle widerspricht.
