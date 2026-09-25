# Das Merkzeichen gehört seinem Punkt und nicht seinem ersten Kind

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Issues:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-2019_c_das-merkzeichen-liegt-im-bereich-des-ersten-kindes-und-wird-fett-kursiv-fest-oder-eingefaerbt.md` (behoben); `…/260812-2019_c_ein-leerer-listenpunkt-zeigt-sein-rohes-bindestrich-zeichen-und-verliert-seinen-einzug.md` (behoben); `…/260812-2019_c_das-merkzeichen-eines-aeusseren-punktes-wird-vom-woertlichen-quelltext-eines-inneren-eingeloest.md` (Zusage berichtigt, Ausgabe festgeschrieben); `…/260812-2019_c_ohne-umgebungszeichen-laeuft-auch-auf-dokumentebene-und-nimmt-dort-einzug-weg-der-inhalt-ist.md` (behoben, Rest abgetrennt); `…/260812-2019_c_die-aufzaehlung-inhaltsart-wird-nur-ueber-matches-gelesen-und-haelt-den-bau-nicht-an.md` (behoben); `…/260812-2019_c_der-geschlossene-deckungs-datensatz-nennt-die-aufzaehlung-inhalt-sie-heisst-inhaltsart.md` (berichtigt per Nachtrag); neu abgelegt `…/260812-2140_o_ohne-umgebungszeichen-nimmt-innerhalb-eines-elements-mehr-einzug-weg-als-die-umgebung-wiederholt.md`
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 466, nachher 478

*Der Zeitstempel folgt der Uhr; siehe den offenen Datensatz
`260812-1805_o_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`.*

---

## Eine Datei, sechs Datensätze, vier Änderungen am Verhalten

Alles steht in `crates/krk-ui/src/markdown.rs`; keine zweite Datei ist
angefasst. Die sechs Datensätze der Durchsicht vom 260812-2019 hängen an drei
Stellen derselben Zerlegung, und drei von ihnen fallen mit je einem Griff weg.

## Der Hauptbefund: der Nachzug beim Einlösen

`Zerlegung::oeffnen` setzt `Offen::anfang` auf `self.stelle`. Öffnete sich ein
Kind als erstes in einem Listenpunkt, so stand `self.stelle` noch **vor** dem
Merkzeichen, denn das ist seit `c35f8b1` ein Wunsch und kein geschriebener
Text. `merkzeichen_einloesen` schrieb es später und erhöhte `self.stelle`,
zog aber keinen `Offen::anfang` nach — anders als `absetzen`, das genau das
für die Umbrüche tut. Der Bereich des Kindes deckte damit das Merkzeichen mit
ab, und `crate::appkit::textmerkmale` setzte über ihn eine Schrift: der
Aufzählungspunkt wurde fett, kursiv, festbreit, überschriftsgroß oder
eingefärbt und unterstrichen.

`merkzeichen_einloesen` läuft jetzt Punkt für Punkt statt über eine gesammelte
Zeichenkette und rückt nach dem Schreiben die Einträge **hinter** dem Punkt
nach, deren `anfang` noch auf der Stelle davor steht. Der Punkt selbst und
alles außerhalb behalten ihren Anfang; der Einzug seiner Listenzeile soll das
Merkzeichen mitnehmen. Darin unterscheidet sich der Nachzug von dem in
`absetzen`, wo der Abstand keinem der offenen Elemente gehört und deshalb alle
nachrücken. Beide Nachzüge stehen jetzt nebeneinander im Modulkopf, mit dem
Unterschied dazwischen.

## Der leere Listenpunkt

Ein Punkt ohne jeden Inhalt fiel in den wörtlichen Zweig von `schliessen` und
gab sein rohes `- ` samt Zeilenumbruch heraus, ohne `Listenzeile` und damit
ohne Einzug. In derselben Liste standen `• ` und `- ` nebeneinander.

Der Datensatz hielt offen, ob die Frage „trägt der Quellbereich mehr als das
Merkzeichen" mechanisch entscheidbar ist. Sie ist es, und ohne die
Merkzeichenlängen zu vermessen: der Quellbereich eines Punktes fängt bei
seinem Merkzeichen an, und CommonMark lässt darauf Leerraum oder das
Zeilenende folgen. Das erste durch Leerraum abgetrennte Stück ist deshalb
immer das Merkzeichen. `traegt_nur_sein_merkzeichen` fragt allein, ob dahinter
noch eines kommt — eine Zeile, keine Aufzählung von `-`, `*`, `+`, `1.`, `1)`.

Trifft es zu, löst `schliessen` den Wunsch ein statt wörtlich auszugeben. Die
Gegenseite — der Punkt mit Verweisdefinition, entschieden in
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`
— bleibt unangetastet, und die Grenze zwischen beiden ist jetzt selbst
gemessen.

## Die Dokumentebene und die Laufzeit

`luecke_bis` rief `ohne_umgebungszeichen` unbedingt, also auch bei leerem
`offen`, wo keine Umgebung etwas wiederholt; der Einzug einer Fortsetzungszeile
ging dort verloren. Der Aufruf hängt jetzt an `self.offen.is_empty()` — keine
neue Frage, sondern die Grenze, an der Satz 1 und Satz 2 der Deckung ohnehin
auseinandergehen. Auf Dokumentebene steht wieder das `trim()` der Vorfassung,
als `Cow::Borrowed` und damit ohne Zuweisung.

**Das hat die Laufzeit zurückgeholt.** Der alte Aufruf legte für jede Lücke
einen `Vec<&str>` und zwei `String` an, und auf Dokumentebene ist jede
Blockgrenze einer Datei eine Lücke. Gemessen auf einer 1,05-MB-Quelle,
Profil `release`, `markdown::rendern` mitsamt einem Stub für
`crate::hervorhebung` in ein Prüfprogramm kopiert, drei Fassungen abwechselnd
gefahren, bestes von zwölf Läufen zu je bestem von fünf:

| Fassung | 1,05 MB |
|---|---|
| `f401dcc` (vor dem Deckungsumbau) | 18,5 ms |
| `c35f8b1` (Stand vor diesem Turn) | 23,0 ms |
| jetzt | **20,9 ms** |

Rund die Hälfte der Verschlechterung, die die Durchsicht mit 17 Prozent
beziffert hat, ist damit weg. Die Zahl im Modulkopf — „19 bis 30 ms fuer
1,05 MB" — hält und ist deshalb nicht nachgezogen; der Abstand zur oberen
Kante ist von etwa einer auf gut neun Millisekunden zurückgewachsen. Die
absoluten Zahlen sind mit denen der Durchsicht nicht unmittelbar vergleichbar,
weil die Quelle eine andere ist; vergleichbar ist das Verhältnis der drei
Fassungen, und sie sind auf derselben Quelle und in derselben Sitzung
gemessen.

## Die Aufzählung hält den Bau jetzt an

Die beiden `matches!(eintrag.inhalt, Inhaltsart::Bloecke)` sind durch
`Inhaltsart::deckt_luecken` ersetzt, eine Methode mit erschöpfendem `match`.
Aus zwei Lesestellen ist eine Entscheidung geworden, und eine dritte Variante
hält den Bau an, statt still als „nicht gedeckt" durchzulaufen.

## Der Befund, der eine berichtigte Zusage ist und keine Änderung

`- - [ZIEL]: …` liefert weiterhin `• - [ZIEL]: …`. Der Doc-Kommentar an
`schliessen` sagte pauschal zu, diese Form entstehe nicht; er sagt jetzt, was
gilt. Die Ausgabe ist richtig: das `• ` ist das Merkzeichen des äußeren
Punktes, dessen `- ` im Vorspann liegt und sonst nirgends herauskäme, das `- `
daneben das des inneren, das mit dessen Quellbereich mitkommt. Beide
Merkzeichen der Quelle stehen genau einmal da. Die Einlösung im wörtlichen
Zweig zu unterdrücken — der Zuschnitt des Datensatzes — ließe das äußere
ersatzlos verschwinden.

## Die Proben messen jetzt Bereiche

Das ist der Kern des Auftrags und der Grund, aus dem zwei Turns hintereinander
je eine Verschlechterung eingeschleppt haben. Der Ausgabetext war in allen
acht gemessenen Fällen des Hauptbefunds **richtig**; falsch war allein der
Bereich. Eine Probe, die `"• fett"` erwartet und über die Bereiche schweigt,
hätte den Defekt durchgelassen — und genau das taten alle 38.

Zwölf neue Proben, davon acht mit ausdrücklicher Bereichsprüfung:

- `eine_auszeichnung_am_anfang_eines_punktes_deckt_das_merkzeichen_nicht` —
  fünf Auszeichnungsarten in einer Tabelle (fett, kursiv, Quelltext,
  Überschrift, Quelltextblock), je Bereich der Auszeichnung **und** Bereich
  der Listenzeile,
- `ein_verweis_am_anfang_eines_punktes_faerbt_das_merkzeichen_nicht` — die
  Einfärbung, die als einzige keine `Auszeichnung` ist,
- `eine_nummer_am_anfang_eines_punktes_wird_nicht_mit_ausgezeichnet`,
- `zwei_merkzeichen_liegen_gestaffelt_ausserhalb_der_auszeichnung`,
- `eine_lose_liste_haelt_ihre_auszeichnung_hinter_dem_merkzeichen` — die lose
  Liste ist der Fall, den Turn 3 nicht maß, die Auszeichnung am Anfang der,
  den Turn 4 nicht maß; beide zusammen in einer Probe,
- `kein_merkzeichen_liegt_im_bereich_eines_stueckes`,
- `ein_punkt_ohne_jeden_inhalt_zeigt_sein_gerendertes_merkzeichen`,
- `ein_leerer_punkt_traegt_dasselbe_merkzeichen_wie_seine_nachbarn`,
- `zwei_leere_punkte_uebereinander_tragen_beide_ihr_merkzeichen`,
- `der_leere_punkt_und_der_woertliche_zweig_trennen_sich_am_inhalt`,
- `ein_innerer_punkt_ohne_zeichen_steht_neben_dem_merkzeichen_des_aeusseren`,
- `auf_dokumentebene_bleibt_der_einzug_einer_zeile_stehen`.

**`kein_merkzeichen_liegt_im_bereich_eines_stueckes` ist der Gurt um die ganze
Klasse und nicht ein weiterer Fall.** Sie läuft über dreizehn Quellen und
verlangt, dass kein Bereich, der kein Absatzmerkmal ist, mit einem gerenderten
Merkzeichen beginnt — weder mit dem `• ` noch mit einer Nummer samt Punkt und
Leerzeichen. Nur `Auszeichnung::Listenzeile` darf es. Sie hätte den Defekt
gefangen, ohne dass jemand den einzelnen Fall hätte nennen müssen.

**Jede der drei Verhaltensänderungen ist gegen ihren Vorzustand
gegengeprüft.** Der Nachzug, der Zweig für den leeren Punkt und der
Dokumentebenen-Zweig wurden nacheinander probeweise wieder herausgenommen; es
schlugen sieben, drei und eine Probe fehl. Keine Änderung steht ohne eine
Probe da, die ohne sie rot wird.

## Keine vorhandene Probe ist abgeschwächt

Im Prüfmodul steht keine gelöschte und keine geänderte Zusage; alle 38
vorhandenen Proben laufen unverändert durch, darunter
`ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen`,
`im_vorspann_eines_elements_endet_die_deckung` und
`die_zeichen_eines_gerenderten_elements_bleiben_weg`.

## Was offen bleibt

`ohne_umgebungszeichen` nimmt **innerhalb** eines Elements weiterhin jeden
führenden Leerraum weg und nicht nur so viel, wie die Umgebung wiederholt. Das
ist der Rest der zweiten Messung jenes Datensatzes; er ist als
`260812-2140_o_ohne-umgebungszeichen-nimmt-innerhalb-eines-elements-mehr-einzug-weg-als-die-umgebung-wiederholt.md`
abgelegt, mit einem Zuschnitt und der Frage, die vor ihm zu messen wäre.
Gewicht niedrig.

## Buchführung

Die sechs Datensätze der Durchsicht tragen je einen `Resolved:`-Absatz und den
Marker `_c_`. Der Namensbefund ist **nicht** durch Umschreiben behoben: die
Abschlussnotiz von
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`
steht, wie sie geschrieben wurde, und trägt einen datierten Nachtrag am Ende.
Er nennt den richtigen Namen `Inhaltsart` samt Grund und hält daneben fest,
welche zwei Sätze jener Notiz sich durch diesen Turn überholt haben.

Alle Verweise auf Datensätze in den neuen Absätzen stehen mit vollem
Dateinamen und `.md`, nicht in Kurzform — siehe
`shared/issues/260810-1851_c_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`.
