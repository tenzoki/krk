# Durchsicht: Turn 1 der Runde 16 — `regex`, die siebte Ablagedatei und zwei gedeckelte Lesewege

**Reviewed-range:** `278a008..b76800b`
**Not-opened:** none

**Datum:** 2026-08-24, 10:14
**Sender:** coderev
**Maßstab:** `planning/260824-0613_o_spec-…` (C1, C3, C6), `planning/260824-0640_o_plan-…`
(Schritte 1, 2, 4, 13), `CLAUDE.md`
**Übernommen aus früheren Durchsichten:** nichts. `bin/fusion-review-coverage` meldet
`carried=(not recorded)`; keine frühere Durchsicht dieses Circles hat ein `**Not-opened:**`
geführt. Das heißt „nicht aufgezeichnet" und nicht „nichts offen".

---

## Summary

Vier Commits, zwei davon mit Code. Der Bau ist sauber: `cargo build`, `cargo test`
(1.465 Proben), `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` laufen
grün, und jede der vier gemessenen Aussagen in der Begründung zu `regex` hält am Baum. Kein
Verhalten ist gekippt: `lesen` liefert nach dem Umbau denselben Bestand, `anlesen` prüft den
Typ weiter am Deskriptor, und die zwei zusammengezogenen Zeitschranken-Hüllen tragen ihren
alten Meldetext Zeichen für Zeichen. **Drei Befunde sind neu abgelegt, keiner hält den nächsten
Schritt auf**, und der schwerste ist kein Codefehler, sondern eine Anweisung im Bestand, die
seit Schritt 2 in die falsche Richtung zeigt.

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 2 |
| Niedrig | 1 |

Dazu eine Berichtigung an einem offenen fremden Datensatz, ohne eigene Datei.

---

## Befunde nach Thema

### Thema 1: die gewachsene Aufzählung und die Prosa daneben

**M1 — Vierzehn Prosastellen der Ablage sagen weiter „vier", und ein offener Datensatz schützt
drei davon ausdrücklich.** Schwere: mittel.
Datensatz: `issues/260824-1014_o_vierzehn-prosastellen-der-ablage-sagen-weiter-vier-und-ein-offener-datensatz-schuetzt-drei-davon.md`

Schritt 2 hat `Datei::ALLE` auf sieben und die TOML-Dateien auf fünf gebracht. Nachgezogen sind
`pfade.rs` und zwei Doc-Kommentare in `tests/ablage.rs`. Offen bleiben `mod.rs:1` („sechs
Dateien"), `mod.rs:4`, `:59`, `:117-118`, `:143`, `:241`, `:549`, `:645`, `:769` und sieben
Stellen in `tests/ablage.rs` (`:53`, `:70`, `:126`, `:1049`, `:1079`, `:1558`, `:1577`).
Schritt 8 des Plans deckt davon genau eine ab (`mod.rs:4`).

Der eigentliche Schaden ist eine Umkehrung: `shared/issues/260821-1023_o_…` schreibt aus,
`mod.rs:59`, `:549` und `:645` seien **richtig** und dürften nicht mitgezogen werden. Das
stimmte bei seiner Erhebung und stimmt seit Schritt 2 nicht mehr. Wer jenen Datensatz nach
seinem Wortlaut abarbeitet, lässt genau die drei Stellen stehen, die diese Runde falsch gemacht
hat. Die am 260824-0940 angehängte `Also seen`-Zeile nennt die Umkehrung nicht.

Eine der vierzehn ist mehr als eine Zahl: `mod.rs:241` begründet `Grund::ZuGross` mit „die vier
TOML-Dateien schreibt KRK selbst" — `readers.toml` schreibt KRK gerade nicht selbst, und
`pfade.rs:148-152` sagt es an derselben Aufzählung ausdrücklich. Der Schluss bleibt richtig
(ihn trägt `Zugang::text_laden`), die genannte Begründung nicht.

**Was in Ordnung ist.** Die drei Fallunterscheidungen über `Datei` — `dateiname`, `format`,
`leerbefund` — sind vollständig und ohne Auffangzweig geblieben, der Übersetzer hält sie, und
im ganzen Baum gibt es keine vierte Stelle, die über `Datei` verzweigt. `Datei::Leser` steht wie
geplant hinter `Datei::Einstellungen` und vor den zwei Zetteln. Zur Laufzeit ändert der neue
Wert nichts: `Datei::ALLE` hat außerhalb der Proben keinen Rufer.

### Thema 2: der dritte Leseweg und das Kriterium, das ihn nicht kennt

**M2 — C3.14 nennt `bis_zur_grenze_lesen` als den Leseweg, und Schritt 4 hat `anlesen`
gebaut.** Schwere: mittel.
Datensatz: `issues/260824-1014_o_c3-14-nennt-bis-zur-grenze-lesen-als-den-leseweg-und-schritt-4-hat-anlesen-gebaut.md`

C3.14 (`spec:208`) und der Constraints-Abschnitt des bindenden Datensatzes
`decisions/260824-0541_a_wie-zieht-der-baustein-…` nennen beide `bis_zur_grenze_lesen` als den
Leseweg der Zusammenfassung. C6.6 verlangt, dass Titel und Feld aus den gelesenen Bytes
entstehen; `bis_zur_grenze_lesen` weist über der Grenze ab (`datei.rs:633-635`) und liefert gar
keine. Beide Kriterien sind im Wortlaut nicht zugleich erfüllbar. Der Plan begründet `anlesen`
aus C6.6, nennt C3.14 aber an keiner Stelle, und kein Datensatz hält den Widerspruch fest.

Die prüfbare zweite Hälfte von C3.14 hält unverändert: `anlesen` geht durch dieselbe eine Tür
`sys::ohne_warten_oeffnen`, prüft am `fstat` des offenen Deskriptors und liest über `take`
(`datei.rs:683-708`). Eine zweite Tür entsteht nicht, `krk-core` führt weiter kein `libc`, und
`blockierend_stellen` liegt in der Tür selbst (`verzeichnis/sys.rs:827`), gilt also für alle
drei Hüllen.

**L1 — Zwei Doc-Kommentare in `datei.rs` tragen eine Messung an einem Werkbankdatensatz im
Präsens.** Schwere: niedrig.
Datensatz: `issues/260824-1014_o_zwei-doc-kommentare-in-datei-rs-tragen-eine-messung-an-einem-werkbankdatensatz-im-praesens.md`

`datei.rs:145-146` und `:678-679` behaupten im Präsens „der groesste Circle-Datensatz dieser
Werkbank ist 119.614 Bytes gross, und seine Zeile `## Directive` steht bei Byte 222". Beides
stimmt heute (nachgemessen). Die zitierte Datei ist der `_d_`-Datensatz des abgesagten
Web-Betrachters, also der wahrscheinlichste Archivkandidat der Werkbank. Der Plan schreibt für
die Schwesterzahlen selbst, dass Werkbankzahlen in keiner Probe stehen dürfen; diese eine ist im
ausgelieferten Quelltext gelandet.

### Thema 3: der Zip-Befund aus Schritt 2

**Bestätigt, mit zwei Berichtigungen** — als `Also seen` an
`issues/260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`
angehängt, keine eigene Datei.

`toml_dateien()` liefert seit Schritt 2 fünf Werte, `ersetzungen_der_toml_dateien` vier, und
`zip` (`tests/ablage.rs:1093`) kürzt still auf vier. Der Befund stimmt. Zwei Dinge stehen im
Datensatz nicht richtig oder gar nicht:

- Er nennt zweimal **Schritt 7** als den Schritt, der den Ladeweg baut. Es ist Schritt 8
  (`ablage/leseprofile.rs`, Bündel C); Schritt 7 ist die Auslieferungsfassung beim `ontocoder`.
  Der Schwesterdatensatz `260824-0955_o_die-files-zeile-eines-planschritts-…` führt es richtig.
- Die Paarung ist heute nur deshalb noch die richtige, weil `Datei::Leser` als **letzte**
  TOML-Datei in `Datei::ALLE` steht. Wer die Reihenfolge dort ändert, bekommt statt der stillen
  Kürzung eine falsch gepaarte Zusicherung mit irreführendem Meldetext.

**Weitere Stellen dieser Art gibt es nicht.** Vierzehn `zip`-Aufrufe stehen im Baum; die
übrigen dreizehn laufen über Felder fester, typgeprüfter Länge (`fokus.rs:419`,
`zulaessigkeit.rs:428-429`, `tabelle.rs:5047-5048`, `messen.rs:2401`, `:2479`) oder über
Iteratoren derselben Quelle (`stapelumbenennen/vorschau.rs:84-85`, `editor.rs:998`, `:1017`,
`:5212`, `ablage.rs:510`). Die Tafelproben tragen zusätzlich eine Zählzusicherung
(`geprueft == 280`).

---

## Was geprüft wurde und gehalten hat

Diese Punkte sind einzeln gegen den Baum gelesen und tragen keinen Befund.

**Die vier Aussagen der Begründung zu `regex` sind gemessen und stimmen.**
`Cargo.lock` wächst von 97 auf 98 Pakete (`git show 278a008:Cargo.lock` gegen den heutigen
Stand), und der einzige neue Eintrag ist `regex` 1.13.1. `regex-automata` 0.4.18,
`regex-syntax` 0.8.11, `aho-corasick` 1.1.5 und `memchr` 2.8.3 standen bereits in genau diesen
Fassungen im Baum, sämtlich über `fancy-regex` → `regex-automata`. `cargo tree --workspace -e
normal,build` findet weder `cc` noch `onig` noch einen Namen auf `-sys`; `windows-sys` steht
wie zuvor allein in `Cargo.lock`. `fancy-regex` 0.16.2 setzt `backtrack_limit` in der Vorgabe
auf `1_000_000` (`src/lib.rs:582`) und liefert `RuntimeError::BacktrackLimitExceeded`
(`src/error.rs:91`). `regex` 1.13.1 trägt `rust-version = "1.65"` und die Vorgabemerkmale
`std`, `perf`, `unicode`; `perf-backtrack` schaltet `regex-automata/nfa-backtrack` ein, dessen
Lauf beschränkt ist und besuchte Zustände vormerkt.

**Die Form der Begründung hält die der übrigen Einträge.** Verweis auf den entscheidenden
Datensatz, die verworfene Alternative mit ihrem messbaren Ausschlussgrund, der Satz „keine
bestehende Abhängigkeit leistet das", die gezählten Pakete, die Merkmalswahl mit Begründung und
die Mindestfassung — dieselbe Gliederung wie bei `icu_collator`, `syntect` und
`pulldown-cmark`.

**`lesen` hat sein Verhalten behalten.** Der Rumpf ist `lesen_hoechstens(pfad, usize::MAX)`, und
die Probe `lesen_liefert_denselben_bestand_wie_der_hoechste_deckel` hält es gegen den Bestand
statt gegen die Reihenfolge, die der Leser niemandem zusagt. Der Deckel greift im Abschluss von
`naechster_schwung` und nicht über der fertigen Liste. `abgeschnitten` trägt die stärkere
Lesart „es wurde etwas weggelassen": fällt der Deckel auf eine Schwunggrenze, holt der Lauf
einen `getattrlistbulk(2)` mehr und meldet erst danach — genau der Fall, den
`ein_deckel_genau_auf_dem_bestand_meldet_kein_abschneiden` festhält. Diese Wahl ist die
Voraussetzung dafür, dass ein Aufrufer auf ein `abgeschnitten == false` eine negative Antwort
stützen darf, und sie folgt derselben Regel wie `sys::ist_deskriptormangel` seit der Runde 10.

**`#[must_use]` hat keine Lücke.** Beide neuen Rückgaben sind `Result`, das die Zusage schon
trägt; `lesen_hoechstens` und `anlesen` können nicht still fallengelassen werden. Weder
`leser.rs` noch `datei.rs` noch `pfade.rs` führt heute überhaupt ein `#[must_use]`, es ist also
nichts zurückgefallen. Ein `#[must_use]` an `Lesestand` würde die eine Gefahr, die der Typ
abwehrt — ein Aufrufer greift `.eintraege` und übergeht `.abgeschnitten` —, nicht abwehren:
Feldzugriff ist eine Benutzung.

**Die zusammengezogene Zeitschranke ändert nichts.** `mit_zeitschranke` erzeugt für beide
bestehenden Rufer denselben Meldetext wie vorher, Zeichen für Zeichen: `was` steht an der
Stelle, an der zuvor das Literal stand, und der Rest der Zeichenkette ist unverändert. Faden,
Kanal und `recv_timeout` sind dieselben. Drei Rufer, wie der Doc-Kommentar sagt.

**Der Modulkopf von `tests/text.rs` zählt richtig.** Drei neue Proben stehen neben den zwölf
Fällen und den vier der Runde 11; Schritt 4 hat sechs Proben zugesagt und sechs geliefert
(drei in `tests/verzeichnis.rs`, drei in `tests/text.rs`).

**Schritt 13 ist gefahren.** Der Berichtigungsabsatz steht im Datensatz
`decisions/260824-0541_a_…` (`**Berichtigung 260824-0910**`), der Wortlaut der Cons-Aufzählung
steht unangetastet daneben, der Marker ist auf `_a_` geblieben, und der Defekt
`260824-0600_*_…-er-fuehrt-eine.md` steht auf `_c_`.

**Die Zahlen des Plans stimmen.** `_d_circle.md` des Web-Betrachters ist 119.614 Bytes groß und
der größte Circle-Datensatz der Werkbank; `## Directive` steht bei Byte 222; `datei.rs:605-637`
im Stand `278a008` ist der Rumpf von `bis_zur_grenze_lesen` samt abweisendem Zweig. Fünfzehn
Runden sind geschlossen (zehn `_b_`, fünf `_c_`), die Bezeichnung „Runde 16" für die laufende
ist damit konsistent gezählt.

---

## Querschnitt

**Ein Muster, zweimal in dieser Runde:** eine Aussage, die zum Zeitpunkt ihrer Niederschrift
gemessen war, wird im Präsens festgeschrieben und veraltet mit dem Bestand. Bei M1 ist es die
Schutzanweisung eines offenen Datensatzes über drei Codestellen, bei L1 eine Dateigröße im
Quelltext. Beide Male ist die Datum-und-Herkunft-Form die Abhilfe, die dieses Projekt für
Kostenangaben in der Wurzel-`Cargo.toml` schon fährt („Am 260824 auf diesem Geraet erhoben").

**Der Plan zieht Prosa nur dort nach, wo eine Datei ohnehin angefasst wird.** Schritt 2 hat
`pfade.rs` mitgezogen, weil er dort schrieb; `ablage/mod.rs` stand nicht in seiner
`Files:`-Zeile und blieb stehen. Schritt 8 nennt eine einzige Prosastelle jener Datei. Das ist
dasselbe Muster, das schon `issues/260824-0955_o_die-files-zeile-eines-planschritts-…` für die
Testdateien beschreibt: die `Files:`-Zeile eines Schritts wird als Arbeitsbereich gelesen, und
was nicht darin steht, geschieht nicht.

**Die Konventionen der Runde sind eingehalten**, wo sie messbar sind: eine Tür zu den Bytes,
kein `libc` in `krk-core`, vollständige Fallunterscheidungen ohne Auffangzweig, deutsche
Bezeichner, `#![deny(unsafe_code)]` unverändert, kein neuer Eintrag im
Untergrenzen-Abschnitt nötig (unter `crates/krk-ui/src/appkit/` ist nichts entstanden).

---

## Reihenfolge

**Kein Auslieferungshindernis.** Diese Runde liefert nichts aus, und keiner der drei Befunde
hält einen Planschritt auf.

1. **M1 vor Schritt 8.** Wer Schritt 8 fährt, fasst `ablage/mod.rs` ohnehin an; die vierzehn
   Stellen dort und in `tests/ablage.rs` gehören in denselben Durchgang. Die Berichtigung an
   `shared/issues/260821-1023_o_…` gehört davor, sonst arbeitet der nächste Leser nach der
   falschen Anweisung.
2. **M2 vor dem Rundenabschluss.** Es ist Buchführung an Spec und Datensatz und keine
   Bauarbeit; sie muss stehen, bevor `## Where this Circle stops` die sechsundfünfzig Kriterien
   abhakt.
3. **L1 bei Gelegenheit**, spätestens beim nächsten Anfassen von `text/datei.rs`.

---

**Abgleich 260824-1852.** Alle drei Befunde dieser Durchsicht stehen als Datensätze unter
`issues/` dieser Runde und tragen den Marker geschlossen (`_c_`): `260824-1014_*_c3-14-nennt-…`,
`260824-1014_*_vierzehn-prosastellen-der-ablage-…` und
`260824-1014_*_zwei-doc-kommentare-in-datei-rs-…`. Die Räumung liegt in `06dbb4c`; C3.14 des
Specs trägt seither die Berichtigung vom 260824-1224. Nachgelesen ist die Sache und nicht der
Marker: `crates/krk-core/src/text/datei.rs` führt `anlesen` (`:691`) an derselben Tür
`sys::ohne_warten_oeffnen` (`:692`) wie `lesen` und `bis_zur_grenze_lesen`. Der Text dieser
Durchsicht bleibt unverändert.
