# Schritt 9: der siebte Inhalt, und der Arbeitsfaden bekommt die Profile mit

**Datum:** 260824-1612 bis 260824-1650
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel D, Schritt 9
**Baumstand vorher:** Schritt 6 versioniert (`abe1a31`), Schritt 8 eingegangen (`4516f4e`), `make check` grün

---

## Auftrag

`Inhalt::Zusammenfassung` als siebten Wert setzen, `zeigt_dateitext` und `laden` darauf
einordnen, und die Profile als `Arc<Profile>` bis auf den Arbeitsfaden der Vorschau
durchreichen. Dazu die Proben zu C2.5, C2.6 und C4.7.

## Was entstanden ist

### In `crates/krk-ui/src/vorschaumodell.rs`

| Stelle | Zeile | Was |
|---|---|---|
| Modulkopf, Abschnitt „Die Zusammenfassung ist der vierte Weg und die vierte Antwort nicht" | 63 | ordnet die Zusammenfassung in die Dreiteilung aus C6 ein und begründet den Arbeitsfaden |
| `Inhalt::Zusammenfassung` | 295 | der siebte Wert, mit A6 am Doc-Kommentar |
| `Ladevorgang::starten` | 330 | nimmt `profile: Arc<Profile>` und reicht es an `laden` |
| `Vorschaumodell::datei_anzeigen` | 495 | nimmt `profile: Arc<Profile>` |
| `zeigt_dateitext` | 555 | eigener Zweig, Antwort `false`, Grund am Zweig |
| `laden` | 676 | nimmt `profile: &Profile`; der Zusammenfassungszweig steht vor dem Rückgabezweig für Ordner und Verknüpfungen |

Der Zusammenfassungszweig sitzt **innerhalb** des Zweigs `metadaten.typ != Typ::Datei` und
vor dessen `return`. Damit tragen beide Zusagen an einer Stelle: `zusammenfassen` ist für
eine Datei nicht erreichbar (C2.6), und ohne Treffer fällt der Weg in genau den Zweig
zurück, der vor dieser Runde der einzige war (C2.5). Ein zweiter Zweig daneben entsteht
nicht.

Warum die Profile als `Arc` und nicht als Kopie reisen, steht an `Ladevorgang::starten`:
derselbe Grund wie bei den Bytes von `Inhalt::Bild`, und dazu die übersetzten regulären
Ausdrücke, die ein Klon je Auswahl ein zweites Mal anlegte.

### Die Proben

| Probe | Zeile | Kriterium |
|---|---|---|
| `ein_ordner_ohne_treffer_zeigt_weiter_alle_sechs_metadatenangaben` | 1270 | C2.5 |
| `eine_datei_unter_einem_treffenden_pfadmuster_zeigt_weiter_ihren_inhalt` | 1349 | C2.6 |
| `ein_erkannter_ordner_zeigt_die_zeilen_seines_profils` | 1305 | die positive Seite, samt Kopfzeile aus A6 |
| `zusammenfassen_hat_einen_rufer_und_der_haengt_am_arbeitsfaden` | 1426 | C4.7 |
| `allein_der_text_einer_datei_traegt_zeilennummern` | 1187 | um den siebten Wert erweitert |

Die Proben bauen ihre Profile über `profile_aus` aus TOML-Text und nicht über
`Profil::neu`, damit sie denselben Weg nehmen wie der Nutzer mit seiner `readers.toml`.

Die Probe zu C2.5 läuft ausdrücklich gegen ein **nicht leeres** Profilbündel: ein leeres
liefe an der Erkennung ohnehin vorbei und sagte nichts darüber, was ein Profil tut, das
seinen Ort nicht findet.

Die Probe zu C2.6 trägt eine Gegenprobe am selben Bündel: das Muster `'werkbank'` trifft
sowohl die drei Dateien als auch den Ordner darum, und der Ordner liefert die
Zusammenfassung. Ohne sie sagte die Probe nur, dass das Muster nirgends trifft.

C4.7 ist eine Aussage über den Baum und an keinem Rückgabewert abzulesen. Die Probe
schließt die Kette mit zwei Zählungen über `crate::quellbaum`: in `crates/krk-ui` ruft
genau eine Stelle `zusammenfassen`, und sie steht in `vorschaumodell.rs`; und in dieser
Datei ruft außerhalb des `#[cfg(test)]`-Moduls genau eine Stelle `laden`, nämlich der
Rumpf des Arbeitsfadens. Gezählt wird allein `krk-ui`: `krk-core` erklärt `zusammenfassen`
und prüft es in eigenen Proben, und beides sind keine Rufer der Oberfläche. Die
verbleibende Blindheit — ein Aufruf unter anderem Namen über `use … as` — steht am
Doc-Kommentar der Probe.

## Abweichung vom Auftrag: drei Zeilen in `appkit/vorschau.rs`

Der Auftrag nennt als Datei allein `vorschaumodell.rs` und verlangt zugleich ein grünes
`make check`. Beides zusammen geht nicht: ein siebter `Inhalt` hält den Bau an drei
Stellen an, die alle in `crates/krk-ui/src/appkit/vorschau.rs` liegen und alle zu Schritt
10 gehören. Der Übersetzer hat genau diese drei genannt:

1. `anzeigen` — vollständige Fallunterscheidung ohne Auffangzweig
2. `einzufaerben` — dieselbe Bauart
3. `datei_anzeigen` — die neue Stelle im Aufruf ans Modell

Gesetzt ist die jeweils kleinste Antwort, jede mit einem Kommentar, der Schritt 10 nennt:
`anzeigen` ruft `text_zeigen(&zusammenfassung.als_text())`, `einzufaerben` nimmt den Wert
in seine vorhandene `None`-Liste auf, und `datei_anzeigen` übergibt `Arc::default()`, also
„keine Profile". Nicht angerührt sind die Doc-Kommentare, der Modulkopf, das Merkfeld
`profile`, `profile_setzen` und die zwei Zählproben — das ist der Gegenstand von Schritt
10, und der eigene Zweig für `einzufaerben` samt Begründung steht dort noch aus.

Die Übergangsfassung ist zur Laufzeit heute wirkungslos: die Profile kommen erst mit
Schritt 11 in die Anwendung, bis dahin ist der übergebene Satz leer und ein Ordner zeigt
seine Metadaten wie vor der Runde.

## Abnahme

`make check` — Exit 0. Die 25 Proben von `vorschaumodell` laufen grün, darunter die vier
neuen.

## Was offen bleibt

Schritt 10 und Schritt 11, unverändert nach Plan. Schritt 10 findet die drei Stellen in
`appkit/vorschau.rs` übersetzbar vor und schreibt seine Begründungen, seinen Modulkopf,
das Merkfeld und die zwei Zählproben dazu.
