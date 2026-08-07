# Codedurchsicht — Turn 25 der Sitzung 260806-2257

**Absender:** coderev
**Umfang:** Diff `f9a0462..HEAD`, die zwölf Programmdateien des Auftrags.
`resources/` und `fusion-workbench/` ausgenommen.
**Grundlage:** gelesen wurden die geänderten Stellen samt ihrem Umfeld, dazu
`crates/krk-core/src/verzeichnis/leser.rs` vollständig,
`crates/krk-ui/src/tabs.rs` vollständig, die Aufrufstellen in
`crates/krk-ui/src/appkit/anwendung.rs` und `…/tabelle.rs`,
`crates/krk-ui/src/auffrischung.rs`, `crates/krk-ui/src/messmodus.rs` und
`xtask/src/release.rs`. `make check` wurde nicht wiederholt.

---

## Zusammenfassung

Die verzögerte Ersetzung im Ordnermodell (`5f2e45d`) ist an der Lesestelle
selbst sauber: der Ersatz wird genau einmal eingelöst, jeder Ausgang des Lesers
kommt bei `abschliessen` an, und ein Stapel des alten Laufs kann nicht mehr
ankommen. Der Bruch sitzt eine Schicht darüber. Drei Stellen in `krk-ui` lesen
den Modellbestand und seine Auswahl weiterhin so, als gehörten beide dem
laufenden Ordner; in der neuen Zwischenzeit gehören sie dem vorigen. Daraus
folgen die beiden schwersten Befunde, und beide haben dieselbe Ursache.

Die Änderung an der Messstrecke (`81d10c1`) hält. Die Grenzprüfung
(`4db66ed`) hält für alles, was heute im Baum steht, und hat zwei Löcher für
das, was morgen darin stehen könnte. Die beiden Sichtbarkeitsöffnungen sind in
Ordnung.

## Zahlen

| Gewicht | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 1 |
| Gering | 3 |

## Befunde

### Hoch — die Auswahl trifft den noch nicht abgelösten Bestand

`crates/krk-ui/src/appkit/tabelle.rs:1082-1103`. `eintrag_waehlen` fragt zuerst
den Modellbestand und erst danach, ob ein Lesevorgang läuft. Seit `5f2e45d`
sind beide Bedingungen in der Zwischenzeit zugleich wahr, und die erste
gewinnt: die Methode meldet `Gewaehlt`, setzt die Auswahl auf eine Zeile des
vorigen Laufs und hinterlegt keine `wunschauswahl`. Der erste Stapel räumt die
Auswahl weg, und sie kommt nicht wieder.

Deterministisch nach einem Stapel-Umbenennen
(`crates/krk-ui/src/appkit/anwendung.rs:2303-2317`): Auffrischung und
`eintrag_waehlen` laufen im selben synchronen Aufruf, der Ersatz steht also
sicher noch aus, und der erste neue Name kann im alten Bestand stehen — bei
einer Umnummerierung nach oben ist das der Normalfall. Die im Kommentar bei
`:2307-2311` ausgeschriebene Zusage, dass die Auswahl danach auf dem ersten
neuen Namen steht, hält dann nicht.

Als Rennen erreichbar auch über den Sprung aus der Zwischenablage (C10,
`…/tabelle.rs:1063-1067`) und über die Messstrecke
(`…/anwendung.rs:2559-2580`).

Gefilt als
`issues/260807-0800_o_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-und-die-auswahl-faellt-danach-ersatzlos.md`.

### Mittel — `auswahlname` hält die veraltete Modellauswahl für gültig

`crates/krk-ui/src/tabs.rs:192-201` und `:476-486`. `auswahlname` zieht
`modell.auswahl()` der `wunschauswahl` vor. In der Zwischenzeit ist diese
Auswahl veraltet, und `aktiven_neu_lesen` schreibt sie bedingungslos in die
`wunschauswahl` — über einen Namen hinweg, den ein Aufrufer kurz zuvor dort
vorgemerkt hat. Der neu angelegte oder umbenannte Eintrag bekommt die Auswahl
dann nicht.

*inference, nicht beobachtet.* Der Fall verlangt zwei `neu_lesen`-Aufrufe vor
dem ersten Stapel. Zwei Wege dorthin sind im Code angelegt: die Schleife über
die gemeldeten Pfade im FSEvents-Rückruf (`…/anwendung.rs:1236-1241`) und die
Schleife über `vorgang.ordner()` (`…/anwendung.rs:2303-2305`). Über zwei
FSEvents-Bündel allein ist er nicht zu erreichen: dazwischen liegen die 0,3 s
Sammelverzögerung aus `…/fsevents.rs:94`.

Gefilt als
`issues/260807-0800_o_auswahlname-haelt-die-veraltete-modellauswahl-fuer-gueltig.md`.

### Gering — der Kommentar der Spalte `Typ` zitiert den falschen Datensatz

`crates/krk-ui/src/appkit/tabelle.rs:137-146` verweist für den Entscheid über
den Zelleninhalt auf den Sortierungsdatensatz. Der hält den Entscheid nicht;
er steht in
`issues/260806-1723_c_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md`,
und der Sortierungsdatensatz weist die Aussage ausdrücklich von sich.

Gefilt als
`issues/260807-0800_o_der-kommentar-der-spalte-typ-zitiert-den-falschen-datensatz.md`.

### Gering — die Grenzprüfung kennt nur die drei `src`-Bäume und nur die wörtliche Schreibweise

`xtask/src/release.rs:57-61` und `:225-243`. Cargo übersetzt je Kiste außer
`src/` auch `tests/`, `benches/`, `examples/` und `build.rs`; `krk-ui` ist die
einzige Kiste mit `objc2`-Abhängigkeiten und hätte dort einen ungeprüften
Baum. Zweitens suchen beide Prüfungen die Zeichenfolge `objc2` im Quelltext;
eine in `Cargo.toml` umbenannte Abhängigkeit oder ein `extern crate … as …`
gehen daran vorbei, und die `Cargo.toml`-Dateien liest die Prüfung nicht.

Gefilt als
`issues/260807-0800_o_die-appkit-grenzpruefung-kennt-nur-src-baeume-und-nur-die-woertliche-schreibweise.md`.

### Gering — zwei Leistenmodell-Proben benutzen feste Prüfordnernamen

`crates/krk-ui/src/leistenmodell.rs:655` (neu) und `:627` (vorher). Im selben
Baum steht die strengere Form mit Prozesskennung und `Drop`
(`crates/krk-ui/src/messmodus.rs:1683-1694`).

Gefilt als
`issues/260807-0800_o_zwei-leistenmodell-proben-benutzen-feste-pruefordnernamen-unter-tmp.md`.

## Nachgeprüft und in Ordnung

Diese fünf Punkte standen im Auftrag und tragen keinen Befund.

**Der Ersatz wird nie zweimal eingelöst.** `ersatz_einloesen` kehrt bei
`!ersatz_ausstehend` sofort zurück; das Kennzeichen fällt vor der Arbeit.

**Kein Ausgang des Lesers bleibt den Ersatz schuldig.** `lesen_und_senden`
(`crates/krk-core/src/verzeichnis/leser.rs:191-241`) liefert für jeden Ausgang
ein `Some(Abschluss)` — den gescheiterten `oeffnen`, den Abbruch, den Fehler
mitten im Lauf und den vollständigen Lauf —, und `lesefaden` schickt daraufhin
`Fertig`. `None` liefert die Funktion allein dann, wenn der Empfänger fort ist,
und dann hört ohnehin niemand mehr zu. Auf der Hauptfadenseite setzt
`einzug_je_tab` bei `Fertig` `abschliessen` ab
(`crates/krk-ui/src/tabs.rs:628-641`), und `Tabliste::abbrechen` tut dasselbe
für das Schließen des Fensters. Die einzige Ausnahme ist der bereits
zurückgestellte tote Netzpfad
(`issues/260805-0000_d_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`):
dort hängt `Schwungleser::oeffnen`, und die Liste zeigt jetzt den alten Stand
statt einer leeren — für denselben Ordner ist das eher besser als schlechter.

**Ein alter Stapel kann nicht mehr ankommen.** Nachgeprüft, nicht geglaubt:
`lesen_starten` setzt `tab.lesevorgang = None`
(`crates/krk-ui/src/tabs.rs:590`), das gibt den `Receiver` frei, und der alte
Arbeitsfaden bricht an `sender.send(…).ok()?` ab
(`crates/krk-core/src/verzeichnis/leser.rs:238` und `:258`). Der neue Kanal
entsteht erst danach in `Lesevorgang::starten`.

**Die Ankündigung des Ersatzes ist richtig geschnitten.**
`ersetzt_beim_naechsten_stapel` fragt `!sichtreihenfolge.is_empty()` und nicht
den Eintragsbestand. Sind alle Einträge des vorigen Ordners ausgeblendet, ist
die Tabelle leer, `noteNumberOfRowsChanged` genügt, und die Ansicht hat keine
Auswahl zu retten. Der Fall `fertig` in
`crates/krk-ui/src/appkit/tabelle.rs:1543-1552` fängt den leeren und den
unlesbaren Ordner mit `reloadData`.

**Der Abbruchweg der Messstrecke ist richtig eingehängt.**
`Handlung::Auswaehlen` steht in der Schrittliste nur hinter
`Warten(Bedingung::AktivZeigt(…))` (`crates/krk-ui/src/messmodus.rs:841-842`),
und diese Bedingung verlangt `!liest_aktiv && zeilen_aktiv > 0`
(`:584-586`). `Auswahlversuch::Unbekannt` heißt dort also wirklich "der Name
steht nicht in einer fertig gelesenen Liste" und nie "der Lesevorgang läuft
noch". Es gibt keinen Zustand, in dem `Unbekannt` der gewöhnliche Weg wäre.

**Die beiden Sichtbarkeitsöffnungen sind eng genug.**
`Leistenquelle::gueltigkeit_nachziehen` steht jetzt auf `pub`; `mod leiste` ist
innerhalb von `appkit` privat, und `appkit/mod.rs:98` exportiert allein
`anwendung::starten`. Über die Kistengrenze ist damit nichts erreichbar, und
alle Nachbarmethoden desselben Blocks tragen schon `pub`.
`bundle::wurzel` steht auf `pub(crate)` und hat außerhalb von `bundle` einen
einzigen Verbraucher, die Probe `die_grenzpruefung_laeuft_am_baum_gruen`; die
Produktionsbahn nimmt `vorlage.wurzel` (`xtask/src/release.rs:77`). Ein
`#[cfg(test)]` wäre um eine Stufe enger, in einem Bauwerkzeug ohne
öffentliche Schnittstelle ist das kein Befund.

## Was zusammengenommen sichtbar wird

**Ein Zustand, drei Leser.** Die Umstellung hat einen neuen Zustand eingeführt —
"der Bestand gehört noch dem vorigen Lauf, die Generation schon dem neuen" —
und ihn genau dort ausgeschrieben, wo er entsteht. Die Leser des Modells sind
ihm nicht gefolgt. Drei nachgewiesene:
`eintrag_waehlen` (`…/tabelle.rs:1086`), `auswahlname` (`…/tabs.rs:193`) und
`alle_namen` (`…/tabelle.rs:1112`, die Kollisionsprüfung des
Stapel-Umbenennens). Der dritte ist heute harmlos, weil das Dateisystem laut
`…/anwendung.rs:1897-1900` ohnehin die Wahrheit über vergebene Namen hält.

Die saubere Antwort ist eine gemeinsame: eine Frage an das Modell, ob sein
Bestand noch dem vorigen Lauf gehört, und drei Leser, die sie stellen. Drei
einzelne Sonderfallzweige wären genau die Sammlung, die die Maxime
"supersimpel" ausschließt.

**Die Kante in `auffrischung.rs` ist enger geschlossen als der Kommentar sagt.**
`crates/krk-ui/src/auffrischung.rs:171-180` sagt zu, die Meldelawine sei "an
der Lesestelle" beantwortet. Für die leere Liste stimmt das. Was die Lawine
sonst kostet, bleibt: `lesen_starten` legt je Meldung einen Arbeitsfaden an und
verwirft den vorigen (`crates/krk-core/src/verzeichnis/leser.rs:114-117`, mit
`expect` beim Fehlschlag des Anlegens), und die beiden Befunde oben treffen
genau die Spanne, die eine Lawine vervielfacht. Der offene Datensatz
`decisions/260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`
stellt die richtige Frage; die Antwort "der Aufschub kann entfallen" trägt
nach dieser Durchsicht nicht, solange die beiden Befunde offen sind.

## Reihenfolge

1. Der Hoch-Befund vor einer Auslieferung. Er ist deterministisch und
   widerspricht einer Zusage, die im Code steht.
2. Der Mittel-Befund gleich mit: dieselbe Ursache, dieselbe Datei-Nachbarschaft,
   und eine gemeinsame Antwort ist billiger als zwei.
3. Die drei Gering-Befunde als Aufräumarbeit, in beliebiger Reihenfolge.
4. `decisions/260807-0010_o_…` erst danach beantworten; die Grundlage der
   Antwort ändert sich mit den beiden ersten Punkten.
