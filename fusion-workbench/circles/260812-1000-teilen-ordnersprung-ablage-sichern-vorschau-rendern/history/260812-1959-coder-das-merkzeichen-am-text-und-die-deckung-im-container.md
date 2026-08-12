# Das Merkzeichen bleibt bei seinem Text, und die Deckung reicht in den Container

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Issues:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1920_c_in-einer-losen-liste-steht-das-merkzeichen-allein-auf-seiner-zeile.md` (behoben); `…/260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md` (behoben, mit benannter Grenze)
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 457, nachher 466

*Der Zeitstempel ist die Uhr und nicht die Reihenfolge: er steht vor
`260812-2040`, obwohl er dessen Verschlechterung behebt. Der Grund ist der
offene Datensatz `260812-1805_o_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`.*

---

## Zwei Defekte, eine Stelle

Beide Datensätze der Durchsicht hängen an `crates/krk-ui/src/markdown.rs`, und
sie hängen aneinander: der erste ist mit dem aufgeschobenen Merkzeichen
behoben, und genau dieses Aufschieben ist die Voraussetzung dafür, dass der
zweite Satz der Deckung an einen Listenpunkt herankommt, der nichts als eine
Verweisdefinition enthält. Wer sie einzeln angefasst hätte, hätte zweimal
gebaut.

Alle gemessenen Ausgaben beider Datensätze sind vor der Behebung am Baum
nachgemessen worden, in einer temporären Probe mit `markdown::rendern`; jede
stimmte zeichengenau. Die Probe ist danach wieder heraus.

## Defekt 1: das Merkzeichen ist ein Wunsch geworden

`Zerlegung::punkt_oeffnen` schrieb das Merkzeichen sofort. In einer **losen**
Liste — einer mit Leerzeilen zwischen den Punkten, und das ist jede, deren
Punkte mehr als eine Zeile tragen — schiebt `pulldown-cmark` zwischen Punkt
und Text einen Absatz, der zwei Umbrüche verlangt; das Merkzeichen stand
danach allein auf seiner Zeile.

Gewählt ist **Zuschnitt 2** des Datensatzes: das Merkzeichen wird am `Offen`
des Punktes vorgemerkt (`Offen::merkzeichen`) und in `Zerlegung::schreiben`
eingelöst, nach `absetzen` und vor dem Text. Damit steht es unmittelbar vor
dem Zeichen, zu dem es gehört, und liegt weiter innerhalb des Bereichs seiner
Listenzeile — der Einzug nimmt es mit. Eingelöst werden alle ausstehenden
Merkzeichen von außen nach innen, denn `- - tief` trägt zwei.

**Zuschnitt 1 ist verworfen**, weil ein sofort geschriebenes Merkzeichen die
Länge eines leeren Punktes von null verschieden macht und ihn damit dem
dritten Satz der Deckung entzieht — also genau der Weg, auf dem der zweite
Datensatz sonst offen geblieben wäre.

## Defekt 2: die Deckung fragt jetzt nach der Art des Elements

`luecke_bis` kehrte zurück, sobald irgendein Element offen stand. Die Prämisse
dahinter — die Lücken in einem Element sind seine Auszeichnungszeichen —
stimmt für einen Verweis und nicht für einen Containerblock.

Die Unterscheidung, die trägt, ist die von CommonMark zwischen Container- und
Blattblock. Sie steht als `Inhaltsart` an jedem `Offen`: `Bloecke` für
Zitatblock, Liste und Listenpunkt, `Zeichen` für Absatz, Überschrift,
Quelltextblock, Betonung und Verweis. `luecke_bis` fragt danach statt nach
`offen.is_empty()`, und `schliessen` gibt bei `Bloecke` zusätzlich heraus, was
zwischen dem letzten Kind und dem Ende ungelesen blieb. Letzteres braucht es,
weil `luecke_bis` beim Endereignis nicht greifen kann: dessen Quellbereich
beginnt am **Anfang** des Elements.

Der Name trägt sein `-art`, weil `vorschaumodell::Inhalt` in derselben Kiste
schon etwas anderes heißt.

`ohne_umgebungszeichen` nimmt einer Lücke zeilenweise, was ihre Umgebung auf
jeder Zeile wiederholt: `>` und Einzug. Ohne diesen Griff stünde zwischen zwei
Absätzen eines Zitats das nackte `>` seiner Leerzeile. Das Merkzeichen einer
Liste steht **nicht** in dieser Menge, denn `-` und `1.` können der Anfang
einer Zeile sein, die dasteht.

## Die Grenze ist benannt, statt zugesagt

Gedeckt ist jetzt alles außer dem **Vorspann** eines Containerblocks, also
allem von seinem Anfang bis zum ersten Byte, das darin gelesen wird. Dort
steht sein Merkzeichen. Eine Verweisdefinition, die sich dorthin verirrt,
fällt mit heraus:

```
"- [ref]: http://a.example\n\n  Text\n"   -> "• Text"
"> [ref]: http://a.example\n>\n> Zitat\n" -> "Zitat"
```

Das ist keine Verschlechterung gegenüber `a9e1149` — dort war der ganze
Container ungedeckt —, sondern der Rest. Er steht an drei Stellen: im Modulkopf
unter „Wo die Deckung endet", am Doc-Kommentar von `luecke_bis` und in der
Probe `im_vorspann_eines_elements_endet_die_deckung`, die beide Ausgaben
festschreibt. Damit gibt es keine Zusage mehr, die weiter reicht als der Code.
Die Grenze zu verschieben verlangt eine Regel, die das Merkzeichen des
Containers vom Quelltext davor trennt, und die ist nicht mechanisch zu haben.

## Neun neue Proben

Jeder gemessene Fall beider Datensätze steht jetzt als Probe in `markdown.rs`,
die lose Liste eingeschlossen — dass sie fehlte, ist der eigentliche Befund
des ersten Datensatzes:

- `eine_lose_liste_haelt_ihr_merkzeichen_bei_seinem_text` (mit beiden Bereichen)
- `eine_lose_geordnete_liste_haelt_ihre_nummer_bei_ihrem_text`
- `ein_punkt_aus_zwei_bloecken_haelt_sein_merkzeichen`
- `zwei_punkte_uebereinander_tragen_beide_ihr_merkzeichen`
- `eine_verweisdefinition_hinter_dem_absatz_eines_punktes_bleibt_stehen`
- `ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen`
- `eine_verweisdefinition_am_ende_eines_zitats_bleibt_stehen`
- `ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text`
- `im_vorspann_eines_elements_endet_die_deckung`

Keine vorhandene Probe musste in ihrer Erwartung nachgezogen werden; alle 31
Markdown-Proben liefen unverändert durch, `die_zeichen_eines_gerenderten_elements_bleiben_weg`
eingeschlossen. Das ist die Gegenprobe, die ein zu weit gezogener zweiter Satz
der Deckung gebrochen hätte.

## Nicht angefasst

`crates/krk-ui/src/hervorhebung.rs` — die Lösung verlangte es nicht. Die zwei
Markdown-Proben dort laufen unverändert durch.
