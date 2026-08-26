# Vollbaum-Durchsicht R13: die dreizehn übrigen Module von `krk-ui` und `tests/syntaxkiste.rs`

**Reviewed-range:** `004ff72..c13bf1c`
**Not-opened:** none
**Sender:** coderev
**Datum:** 260826-1445
**Verification:** 14 Dateien vollständig gelesen (13.342 Zeilen unter `crates/krk-ui/src/`, 168 in `tests/syntaxkiste.rs`), dazu zum Gegenlesen `crates/krk-core/tests/gemeinsam/mod.rs`, die Wurzel-`Cargo.toml`, `crates/krk-ui/Cargo.toml` und Ausschnitte aus `appkit/anwendung.rs`, `appkit/ereignisse.rs`, `appkit/belegungsansicht.rs`, `appkit/textmerkmale.rs`, `krk-core/src/tasten/belegung.rs`. Jede Zeilenangabe am Baum abgelesen und ein zweites Mal gegengelesen. Nichts übersetzt, nichts geändert. Der Quelltext hat sich seit `004ff72` nicht geändert; alle Commits der Spanne tragen Werkbankdateien.

## Zusammenfassung

Die dreizehn Module sind sauber gebaut; die beiden Pflichtstellen `bereich_des_kommandos` und `schiebt_auffrischung_auf` sind vollständig und ohne Auffangzweig, und die Zusagen aus `CLAUDE.md` zu `hervorhebung.rs`, `syntect`/`two-face`, `fenstertitel.rs` und `main.rs` halten am Baum. Was fehlt, ist an drei Stellen eine Messstelle oder ein Vorbehalt, den die Prosa behauptet: die Geschwindigkeit der Hervorhebung ist nirgends messbar, die Spannenstrecke des Messmodus kennt den Vordergrund nicht, und zwei Proben versprechen mehr Deckung, als ihre Beispiele tragen.

## Zahlen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 8 |

Zehn Defektdatensätze unter `shared/issues/260826-1442_o_*`, eine `Also seen`-Zeile an `shared/issues/260826-1223_o_die-nutzerdatei-setzt-den-zusteller-frei-…`.

## Die Zusagen aus `CLAUDE.md`, je Datei geprüft

### `hervorhebung.rs` — das Fortschreiben (`3596e16`)

**Hält.** Je Zeile im Abstand `ZUSTANDSABSTAND = 32` ein `Haltepunkt` mit `Zerlegerstand` (`ParseState` + `ScopeStack`, `:604-627`), Wiedereinstieg am letzten Haltepunkt mit `zeile <= gleiche_anfangszeilen` (`:833-836`). Die drei Fälle der Aufgabe:

- **Abweichung vor dem ersten Haltepunkt:** kann nicht eintreten, der erste Haltepunkt steht immer auf Zeile 0 (`:1097-1112`, Zerleger ist dort `Some`); `partition_point(...).checked_sub(1)?` fiele bei leerer Liste auf `None` und damit auf den vollen Durchgang (`:809-811`).
- **Datei wird kürzer:** `gleiche_endzeilen` ist auf `hoechstens - gleiche_anfangszeilen` gedeckelt (`:823-829`), also zählt keine Zeile doppelt; der Wiederanschluss rechnet `alte_zeile = nummer + zeilenzahl - neue_zahl` (`:907`) und verschiebt Zeilentafel, Haltepunkte und beide Stücklisten um `stelle - alte_stelle` (`:1235-1259`). Der Grenzfall „neuer Text ist Präfix des alten“ lässt einen Haltepunkt auf `zeile == zeilenzahl` stehen; der ist der Stand am Textende und in der nächsten Runde nur erreichbar, wenn er zutrifft. Kein Defekt.
- **Mehrzeiliges Konstrukt über den Haltepunkt hinweg:** der Anschluss verlangt Gleichheit des ganzen `Zerlegerstand` (`:916`, `PartialEq` über `ParseState` samt Kontextstapel und `ScopeStack`); ein geöffneter Blockkommentar oder eine offene Zeichenkette ändert den Stand an jedem folgenden Haltepunkt, also läuft die Rechnung bis zum Ende (`das_fortschreiben_traegt_ueber_viele_haltepunkte`, `:1924-1946`, und die Fälle „Anführungszeichen“, „Blockkommentar“ in `:1873-1879`).

**Messstelle:** keine. `formatieren` ist `cfg(test)` (`:1362`), `krk-bench` kennt die Größe nicht, und die sieben Zahlen im Modulkopf (`:77-131`) sind von niemandem wiederholbar. **Medium**, `…die-syntaxhervorhebung-hat-keine-messstelle…`.

Daneben zwei Low: die zwei Fehler der Kiste werden ungleich behandelt — `parse_line`-Fehler lässt den Stand fallen (`:1135`), `apply`-Fehler behält ihn und hebt ihn auf (`:1139-1142`, `:1203`) —, und die Vierzehn-Fälle-Probe wirft den Fallnamen mit `let _ = name;` weg (`:1895`).

`syntect` liefert unter `regex-fancy` bei überschrittener `backtrack_limit` keinen Fehler nach oben, sondern behandelt das Muster als Nichttreffer (Verhalten der Kiste, in `syntect-5.3.0/src/parsing/regex.rs`; `inference:` aus dem Quelltext der Kiste, nicht gemessen). Der `Err(_)`-Zweig ist damit für die eingebundenen Sprachdefinitionen unerreichbar, wie `:950-959` sagt.

### `Cargo.toml` und `tests/syntaxkiste.rs`

**Hält.** `syntect` steht mit `default-features = false` und `parsing, default-syntaxes, default-themes, dump-load, regex-fancy` (`Cargo.toml:1033-1039`), `two-face` mit `syntect-fancy` (`:1060`). Der Code ruft genau das: `two_face::syntax::extra_newlines` (`hervorhebung.rs:382`, `syntaxkiste.rs:28`), `ThemeSet::load_defaults` (`hervorhebung.rs:388`, `default-themes`), `SyntaxSet::load_defaults_newlines` allein in `syntaxkiste.rs:56` (`default-syntaxes`, wie `Cargo.toml:324-328` begründet), `ScopeRegionIterator`/`Highlighter`/`ParseState` (`parsing`). Kein `html`, kein `plist-load`, kein `yaml-load` im Baum. `syntaxkiste.rs` misst fünf Dinge: die vier Endungen, dass `syntect` allein kein TOML führt, den Rückfall auf `Plain Text`, zwei Tafeln mit verschiedenem Vordergrund, und drei Farben je Sprache; die Geschwindigkeit ausdrücklich nicht (`:10-13`).

### `markdown.rs` — Vorschau und Quellbezug

**Hält.** Feindliches Markdown: ein `javascript:`-Verweis wird zu Farbe und Unterstreichung ohne Klickwirkung (`Behandlung::Verweis`, `:711-716`), ein Bild mit beliebigem Pfad erscheint als sein Quelltext und wird nicht geladen (`:641-644`, `ein_bild_erscheint_als_sein_quelltext`), unbalancierte Klammern entscheidet `pulldown-cmark`, tiefe Listen: `tiefe()` sättigt auf `u8::MAX` (`:1377-1380`), `textmerkmale.rs:445` klemmt den Einzug auf `EINZUGSGRENZE`. Kein Rekursionsabstieg, die Zerlegung ist eine Ereignisschleife.

Quellbezug: Kachelung aus `Abschnitt`en, die auf beiden Seiten lückenlos wachsen, weil `kacheln` die eine Stelle ist, an der `gelesen` und `stelle` vorrücken (`:1061-1069`). Umbrüche: der Quellumbruch hinter einem Block wird als `Ersetzt`-Abschnitt mit leerem Text abgetragen (`gelesen_bis`, `:1253-1259`), die erzeugten Abstände als `Erzeugt` mit leerer Quelle (`:1165-1173`); `SoftBreak`/`HardBreak` schreiben `"\n"` und sind `Woertlich`, wenn die Quelle genau `"\n"` trägt, sonst `Ersetzt` (`:1203-1207`). Eine Auswahl über eine Absatzgrenze liefert deshalb `A\n\nB` aus `A\n\nB\n` — am Code nachvollzogen, von `die_auswahl_ueber_alles_liefert_die_quelle_vollstaendig` gemessen. Ein Low: die Kachelungsprobe nennt sich Totalitätsbeweis und deckt die Auffangregel nicht (`:2473-2557`).

### `belegungsmodell.rs` — `bereich_des_kommandos`

**Vollständig: 79 von 79, ohne Auffangzweig.** Die Arme von `:226-401` gegen `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs` verglichen: gleiche Menge (Dateilisting 27, Dateioperationen 13, Tabs 4, Vorschau 3, Leiste und Fokus 8, Fenster 7, Anwendung 4, Editor 13). Die sechs Textbefehle stehen als Zeichenketten in `bereich` (`:210-218`), deren `_ => None` fängt `jede_kennung_hat_einen_funktionsbereich` für die Auslieferung; eine unbekannte Kennung aus einer Nutzerdatei erreicht `nach_bereichen` nicht, weil `Belegung::bauen` sie schon abweist (`krk-core/src/tasten/belegung.rs:1423`, `UnbekannteFunktion`).

**Code-Stand zur offenen Frage 260813-0053 (welche Tasten):** Möglichkeit 1 ist gebaut, seit Runde 7. `zeichen_anhaengen` nimmt jedes Zeichen aus `traegt_ein_dateiname`, die Leertaste eingeschlossen (`:701-709`); „Zuweisen“ liegt auf Cmd+T, „Fertig“ auf Cmd+Eingabe (`appkit/belegungsansicht.rs:22,27,710,745`). Der Datensatz steht trotzdem `_o_` mit leeren Zeilen, der Abgleich der Runde 7 hat es notiert. **Low**, `…die-frage-welche-tasten…-ist-seit-runde-7-gebaut-und-steht-noch-offen`.

### `belegungsausgabe.rs` — der freigesetzte Befehl

**Angezeigt, nicht verschwiegen, aber vage.** Eine Nutzerdatei mit `gehalten_von = "menue"` an `kopieren` landet in `wirkung` im Auffangzweig (`:357`) und schreibt „(von KRK nicht eingeordnet)“ in die Spalte „Wirkt in“; gemessen von `eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig` (`:875-919`). Die Zeile erscheint, weil die Funktion eine Kombination trägt. Was keine der drei Oberflächen sagt: dass der Befehl damit unerreichbar ist — die Belegungsansicht schreibt sogar „(Kürzel des Menüs)“ (`belegungsmodell.rs:530-536`), obwohl das Menü ihn grau und ohne Kommando führt (`menuemodell.rs:295-300`). Als `Also seen` an den bestehenden Datensatz `260826-1223_*_die-nutzerdatei-setzt-den-zusteller-frei…` gehängt.

### `menuemodell.rs` — Obermenüs und Vollständigkeit

**Code-Stand, nicht entschieden:** neun Obermenüs, eines je besetztem `Funktionsbereich`, in dessen Reihenfolge und mit dessen Namen (`aufbau`, `:234-254`); das Menü geht über `belegungsmodell::nach_bereichen`, also über die **Belegung**, nicht über `Kommando::KENNUNGEN` (`:237`). Eine zweite Ordnung gibt es nicht; die Umsortierung „Anwendung vorn, Fenster hinten“ und die Umbenennung „Textbefehle → Bearbeiten“ stehen in `Funktionsbereich` selbst (`belegungsmodell.rs:84-99, 160-176`) und wirken damit in allen drei Abnehmern. Zu beiden offenen Datensätzen (`260813-0053_*_wie-viele-obermenues…`, `260813-0159_*_darf-das-menue…`) ist das der gebaute Stand.

**Kann ein Kommando ohne Menüeintrag existieren?** Nur, wenn es in `resources/default-keymap.toml` fehlt. Das hält `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (`krk-core/src/tasten/belegung.rs:1708`) und für die Ausgabe `mit_kommando == KENNUNGEN.len()` (`belegungsausgabe.rs:756-760`); eine Nutzerdatei kann keine Funktion streichen, weil `vom_nutzer` jede ungenannte unbelegt hinzunimmt (`belegungsausgabe.rs:483-487`). Ein Kommando ohne Eintrag im Menü ist damit über die Auslieferung ausgeschlossen, über den Ausführungszweig (`kommando_ausfuehren`, Auffangzweig) weiterhin nicht — das ist die Falle aus `CLAUDE.md`, und dieses Modul hat sie nicht.

### `auffrischung.rs` — `schiebt_auffrischung_auf`

**Vollständig: 6 von 6, ohne Auffangzweig** (`:332-341` gegen `pub enum Art`: Kopieren, Verschieben, InDenPapierkorb, UmbenennenImStapel, Zippen, Entpacken). **Code-Stand zur offenen Frage 260807-0010 (kann der Aufschub entfallen):** die Voraussetzung ist erfüllt und im Modul selbst ausgeschrieben — `:322-331` sagt, die leere Liste sei seit dem 260807 an der Lesestelle geschlossen (`Ordnermodell::lesevorgang_beginnen` ersetzt erst mit dem ersten Stapel), und der Aufschub beantworte nur noch, „ob eine Auffrischung während des Vorgangs überhaupt lohnt“. Er ist also Kostenfrage, nicht Richtigkeitsfrage. Ob er entfällt, hängt an der ungemessenen Kostenseite; die zweite Kostenstelle steht daneben (`gleicher_ordner`, `canonicalize` je Pfad, Low).

Ein Low: `die_gemaechlichen()` trägt drei von fünf nicht aufschiebenden Arten und behauptet, eine neue fiele auf (`:796-812`).

### `messmodus.rs` — die Sendeseite

**Wo `NICHT_IM_VORDERGRUND` entschieden wird: hier**, in `messung_unmoeglich` (`:739-745`) aus `Sitzungslage::im_vordergrund`, das die Oberfläche füllt; `krk-bench` liest nur den Abbruchtext. **Kann die Sendeseite eine Zahl liefern, ohne im Vordergrund zu sein?** Auf der Sitzungsstrecke nein: der Vorbehalt steht vor jeder Größe, und `Anweisung::Abbruch` beendet mit 4, ohne `ausgeben` (`anwendung.rs:7838-7841`); das deckt sich mit dem Befund von R15. Auf der Spannenstrecke ja und nein zugleich: `Zustand` trägt dort keine Vordergrundangabe (`:376-386`), L2, L3 und L10 werden im Hintergrund vollständig gemessen, L1 setzt dann den synthetischen Pfeil ab, den `zulaessig` ohne Schlüsselfenster abweist, und nach zehn Sekunden bricht `haengt` mit der Geduldsmeldung ab — die vierzig Werte fallen weg, und die Meldung nennt die falsche Ursache. **Medium**, `…die-spannenstrecke-hat-keinen-vordergrundvorbehalt…`. Synthetische Tastendrücke gehen über `postEvent_atStart` (`ereignisse.rs:593`); `osascript` kommt im Baum nicht vor.

Ein Low: `ordner_a()` fällt still auf `/` zurück, und `pruefen` nimmt den Plan an (`:240-245`).

### `pruefordner.rs` — die dritte Fassung

Prozesskennung und Laufnummer sind mit `gemeinsam/mod.rs` gleich gebaut; auseinander ist das Abräumen: einstufig `let _ = remove_dir_all` (`:129-133`) gegen zweistufig mit zurückgedrehten Rechten (`mod.rs:208-242`). `pfadeingabe.rs:214-221` setzt `0o000` und räumt von Hand „bevor die Probe fehlschlagen kann“ — die Handarbeit, die der Kern abgeschafft hat. **Low.** `wegwerfordner.rs` habe ich nicht geöffnet; R15 hat es gelesen.

### `fenstertitel.rs`, `angezeigtedatei.rs`, `spalten.rs`, `quellbaum.rs`

`titel` ist eine erschöpfende Fallunterscheidung über `Fokus` ohne Auffangzweig (`:85-93`), `Fokus::ALLE` ist `[Fokus; 5]` (`kommandos/fokus.rs:150`), und die Probe `jeder_fokuswert_bekommt_seinen_pfad` deckt alle fünf. `angezeigtedatei::welche` ist über vier Eingaben vollständig und überschneidungsfrei; die Tafel prüft acht Lagen. `Spalte` hat vier Werte, `beschreibbar` ist ein `match` und kein `matches!`. `quellbaum::aufrufstellen` zieht Kommentare, Erklärung und längere Namen ab und benennt seine Blindheit (`use … as`). Nichts zu melden.

### `main.rs` — Einstieg

`#![deny(unsafe_code)]` steht in Zeile 1; die Ausnahme in `appkit/mod.rs` ist im Modulkopf begründet. „Siebzehn Module neben `appkit`“ (`:17`): nachgezählt, siebzehn ohne die zwei `cfg(test)`-Module, stimmt. **Ablage nicht lesbar:** im gewöhnlichen Start läuft KRK mit `Sitzung::default()` weiter und stellt die Meldung in die Statuszeile (`anwendung.rs:1571-1580`), in den Messaufgaben bricht es mit `eprintln!` und Rückgabewert 4 ab (`:1537-1546`); ein Fehler in der Befehlszeile erreicht den Nutzer über `eprintln!` und Rückgabewert 2 (`main.rs:128-131`). Ein Fehler vor dem ersten Fenster erreicht den Nutzer also auf der Standardfehlerausgabe, nicht in einem Fenster — für ein über den Finder gestartetes Bündel heißt das: gar nicht. Das ist eine Eigenschaft der Startphase und kein neuer Befund; der gewöhnliche Start bricht dort nicht ab.

### `#[must_use]`

Sieben in `markdown.rs`, drei in `belegungsmodell.rs`, keines in den elf übrigen. Kandidaten nach der Projektregel („ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe“): `auffrischung::ordner_neu_lesen` und `datentraeger_verloren` (liefern Zahlen, die der Rufer werten soll), `hervorhebung::linkfarbe`, `fortschreiben`, `Einfaerbungsvorgang::abholen`, `messmodus::Messlauf::naechster_schritt` und `bildgrenze`. Ich habe sie nicht einzeln als Defekt gefiltert: der Datensatz `260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use…` führt die Lücke schon als Muster über Kisten hinweg, und eine Zeile je Datei hier wäre die elfte Fassung desselben Befunds.

## Querschnitt

1. **Proben, die mehr versprechen als ihre Eingaben:** die Kachelungsprobe (`markdown.rs:2547`), `die_gemaechlichen()` (`auffrischung.rs:796`), die Vordergrundprobe der Sitzungsstrecke, die nichts über die Spannenstrecke sagt (`messmodus.rs:1906`). Dreimal steht die Zusage im Doc-Kommentar und nicht in der Probe.
2. **Zahlen ohne Messstelle:** der Modulkopf von `hervorhebung.rs`, `ZUSTANDSABSTAND`, die `canonicalize`-Kosten in `auffrischung.rs`. Alle drei sind Kostenaussagen, an denen eine Zusage oder eine offene Entscheidung hängt.
3. **Der Vordergrund als Voraussetzung ist nur auf einer von zwei Strecken modelliert.** `CLAUDE.md` spricht von „der Messstrecke“ im Singular.

## Empfohlene Reihenfolge

Kein Auslieferungsblocker. Zuerst die zwei Medium: die Messstelle für die Hervorhebung (sie ist Voraussetzung der Messrunde) und der Vordergrundvorbehalt der Spannenstrecke (er kostet heute zehn Sekunden und eine falsche Meldung je Fehlstart). Danach die drei Proben-Lows, dann das Abräumen des Prüfordners, die übrigen nach Gelegenheit. Der Datensatz 260813-0053 ist Reconciler-Arbeit, kein Code.
