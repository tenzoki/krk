# Consultation: Schlussdurchsicht „Menü Home und einstellbarer Ort“ vor Nutzerprüfung und Auslieferung 2.0.0

**Date:** 2026-09-26 16:55
**Status:** Complete
**Requested by:** Kai Stalmann über den Orchestrator; Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`

## Question

Ist die gebaute Erweiterung „Menü Home und einstellbarer Ort“ (Commits `b8a5421..67e3293`) zusammen mit dem Grundpaket reif für die Nutzerprüfung und die Auslieferung als 2.0.0? Sieben Einzelfragen: (1) Deckung von H1 bis H3, (2) Umsetzung von M1 bis M5, S1, S3, S4, (3) Sicherheit von `secrets.txt` bei einem Ortswechsel im Betrieb und ohne gültigen Ort, (4) der Schreibweg in `settings.toml`, (5) der Satz über die Stellung fehlender Funktionen einer eigenen Belegung, (6) Auslieferbarkeit und die Liste der Nutzerprüfungen, (7) der offene Entscheid `260926-1506_*_…`.

## Kurzurteil

**Die Erweiterung ist gebaut, wie beabsichtigt, und am Baum grün.** `make check` läuft auf HEAD `67e3293` mit allen fünf Kommandos durch: 2189 Proben bestanden, keine rot, 17 ignoriert, `clippy` und `cargo doc` ohne Warnung. Jede Korrektur aus der Zweitlesung steht im Code, und zwar in der beabsichtigten Bauform. Einen Weg, auf dem `secrets.txt` durch einen Ortswechsel oder durch einen fehlenden Ort im Klartext auf die Platte käme, habe ich nicht gefunden. Es bleibt ein schmaler Restfall, den der Spec ausdrücklich offenlässt.

**Vor der Auslieferung fehlen vier Dinge, und keines davon ist Code:** die Nutzerprüfung, die Antwort auf `260926-1506_*_…`, zwei ungenaue Sätze in `HowTo.md` und `README.md` samt einem dritten, der dem Code widerspricht, und ein sauberer Arbeitsbaum.

| # | Frage | Urteil |
|---|---|---|
| 1 | H1–H3 gedeckt? | Ja. Jedes am Baum nachweisbare Kriterium hat Code und Probe oder eine Quelltextprobe; der Rest ist ausdrücklich Nutzerprüfung. Keine Lücke, die vor der Nutzerprüfung zu schließen wäre. |
| 2 | M1–M5, S1, S3, S4 | Alle wie beabsichtigt umgesetzt. S2 bewusst nicht übernommen, mit tragfähigem Grund. |
| 3 | `secrets.txt` | Tragfähig. Kein Klartextweg gefunden. Ein Restfall: frühe Startausgänge schützen `~/krkhome` statt des eingestellten Orts. |
| 4 | `settings.toml` | Ein Schreiber, robust, von 13 Proben gehalten. Kein Weg gefunden, auf dem Nutzerinhalt verloren geht. |
| 5 | Stellung fehlender Funktionen | Bestätigt. `HowTo.md:83` und `README.md:103-105` sind ungenau, `HowTo.md:481-484` ist schon richtig. Wortlaut unten. |
| 6 | 2.0.0 auslieferbar? | Ja, nach den vier Punkten oben. Major ist nach `README.md` richtig. Prüfliste unten, 20 Zeilen. |
| 7 | Entscheid 1506 | Weiter Möglichkeit 1. Die Antwort sollte den gebauten Schutzort mit aufnehmen. |

## Context

- HEAD `67e3293`. Der Arbeitsbaum trägt eine geänderte Datei, `fusion-workbench/orchestrator-events.jsonl`.
- `Cargo.toml` führt `1.11.1`, der jüngste Tag ist `v1.11.1`, dazwischen liegen 54 Commits (`git log --oneline v1.11.1..HEAD`).
- `make check` habe ich auf HEAD selbst gefahren. Das Protokoll liegt im Arbeitsverzeichnis dieser Sitzung und endet auf `alle fuenf gruen`, `exit 0`.
- Nachgelesen habe ich `crates/krk-ui/src/heimgriff.rs`, `crates/krk-core/src/heimordner/ort.rs`, `crates/krk-core/src/ablage/einstellungen.rs`, `crates/krk-core/src/ablage/atomar.rs` (Kopf), `crates/krk-core/src/ablage/mod.rs` (`beiseite_legen`), `crates/krk-ui/src/appkit/blaetter/ortwahl.rs`, `crates/krk-ui/src/appkit/blaetter/mod.rs`, `crates/krk-core/src/tasten/belegung.rs` (`bauen`), `crates/krk-ui/src/belegungsmodell.rs` (`nach_bereichen`) und in `crates/krk-ui/src/appkit/anwendung.rs` den Start, F2, die Ortswahl und `abbrechen`. Dazu die Liste aller in `b21b3e7..HEAD` hinzugekommenen Funktionen und Proben.
- Die eigene Belegung dieses Geräts (`~/Library/Application Support/KRK/keymap.toml`, Stand 13. September) nennt `notizzettel` an Zeile 459 und keine der übrigen acht Kennungen des Bereichs „Home“.

## Analysis

### 1. Deckung von H1 bis H3

**Jedes am Baum nachweisbare Kriterium ist mit Code und Probe gedeckt; alles andere steht ausdrücklich als Nutzerprüfung im Spec.** Die tragenden Proben, nach Fähigkeit:

- **H1:** `der_bereich_home_fuehrt_genau_diese_befehle_in_dieser_folge` (`belegungsmodell.rs`), `home_ist_das_zweite_obermenue` und `innerhalb_eines_obermenues_bleibt_die_reihenfolge_der_datei` (`menuemodell.rs`), `der_abschnitt_home_fuehrt_die_funktionen_des_bereichs` (`belegungsausgabe.rs`). Dass jeder Befehl genau einmal in der Leiste steht, folgt aus der Bauform: `nach_bereichen` ordnet jede Funktion genau einem Bereich zu (`belegungsmodell.rs:977-1005`).
- **H2:** die Tafeln `ort_lesen_nimmt_genau_die_zulaessigen_formen`, `notizort_folgt_wert_und_ersetzungsgrund`, `startzeile_nennt_den_fehler_des_werts_und_schweigt_zum_schaden_der_datei`, `ortswechsel_meldet_allein_einen_anderen_ort`, `zu_merken_behaelt_den_alten_ort_wenn_keiner_gilt`, `der_schutzort_ist_der_gemerkte_sonst_der_vorgabeort`, `die_leichte_form_entsteht_allein_unmittelbar_im_benutzerverzeichnis`, `die_genaue_frage_gilt_am_eingestellten_ort`, `die_alten_zettel_kommen_allein_am_vorgabeort`, `ein_fehlender_oberer_ordner_legt_nichts_an`, `die_meldungen_nennen_den_eingestellten_ort`, `eine_session_toml_merkt_den_notizordner`, dazu `die_regeln_gelten_am_eingestellten_ort_und_am_alten_nicht` und `die_eigenschaft_folgt_dem_eingestellten_ort` für Vorschau und Inhaltsfilter. Die Quelltextproben `der_start_kennt_den_vorgabeort_allein_im_messmodus`, `der_griff_steht_vor_der_ersten_tabliste` und `f2_nennt_den_fehler_und_legt_ohne_ort_nichts_an` halten die Startreihenfolge und F2.
- **H3:** 13 Proben am Schreibweg (Frage 4), `gehaltene_notizdatei_erkennt_die_drei_dateien_am_gefragten_ort`, `der_ortswahlbefehl_kommt_bei_stehendem_blatt_nicht_durch`, `die_ortswahl_hat_genau_eine_rufkette`, `die_ortswahl_fragt_vor_dem_schreiben_und_setzt_den_griff_vor_dem_nachzug`, `ein_ortswechsel_liest_die_tabs_auf_altem_und_neuem_ort_neu_und_sonst_keinen`, `ein_ortswechsel_laedt_allein_die_tabs_mit_einer_eintragsdatei_neu`, dazu für M1 `abbrechen_nimmt_den_griff_erst_nach_der_naemlichkeitsfrage`, `verdeckt_und_steht_ruft_steht` und `der_schlitz_wird_an_genau_einer_stelle_geleert`.

**Zwei Kriterien hält allein eine Quelltextprobe, nicht eine Verhaltensprobe.** Das sind das Vormerken der Sitzung nach dem Wechsel (H3, „merkt den Ort in der Sitzung“) und `esc` am Ordnerdialog. Beides lässt sich unter `libtest` nicht bauen, weil der Anwendungsdelegierte ohne laufende Anwendung nicht entsteht. Beide stehen deshalb in der Prüfliste unten. Das ist die Lage, die der Plan selbst benennt, und keine Lücke des Baus.

**`HowTo.md:557` widerspricht dem Code.** Der Satz lautet „Übernommen wird genau einmal, nämlich in dem Augenblick, in dem ein `f2` den Vorgabeort `~/krkhome/` selbst anlegt.“ Die Bedingung im Code ist `ordner_angelegt && heim.ist_vorgabeort()` (`crates/krk-core/src/heimordner/bereitstellen.rs:400`), ohne Merker. Übernommen wird also **bei jedem** Anlegen des Vorgabeorts, und genau das beschreibt der offene Defekt `260926-1527_*_…`. Der zweite Halbsatz des Satzes stimmt, „genau einmal“ nicht. Die Anleitung reist im Releasepaket mit, deshalb gehört der Satz vor der Auslieferung berichtigt. Vorschlag: „Übernommen wird in dem Augenblick, in dem ein `f2` den Vorgabeort `~/krkhome/` selbst anlegt, und zwar jedes Mal: wer `~/krkhome` löscht und mit `f2` neu anlegen lässt, bekommt die Zettel ein zweites Mal. Wer das nicht will, legt `note-1.txt` und `note-2.txt` nach der ersten Übernahme selbst beiseite.“ Damit ist zugleich der zweite Abschlussweg des Defekts `260926-1527` erfüllt, „als Lage angenommen“.

### 2. M1 bis M5, S1, S3, S4

| Punkt | Beabsichtigt | Gebaut | Beleg |
|---|---|---|---|
| M1 | `abbrechen` nimmt einen Griff nur, wenn sein Blatt das anhängende ist | So gebaut. `Blattgriff::steht` ist herausgelöst, `verdeckt_und_steht` ruft sie. Liegt ein fremder Griff im Schlitz, gibt `abbrechen` `false` zurück und lässt den Schlitz stehen | `blaetter/mod.rs:675-686`, `anwendung.rs:6990-7003` |
| M2 | Der Start liefert den Ort auf jedem Ausgang | So gebaut. Die zwei frühen Ausgänge liefern `EinstellungenUngelesen`, allein der Messmodus den Vorgabeort | `anwendung.rs:2006`, `:2067`, `:2177` |
| M3 | „Kein Ort“ allein bei `Beschaedigt` und `NichtLesbar`, Vorgabeort bei `NichtAnlegbar`, die Meldung nennt zwei Wege | So gebaut | `ort.rs:209-239`, Meldung `ort.rs:122-124` |
| M4 | Kein Handgriff „F1, `cmd+r`“ | Gestrichen. Spec Zeile 55 und `HowTo.md` warnen ausdrücklich davor | Spec Zeile 55, `HowTo.md` bei den Eintragsbefehlen und bei „PIN ändern“ |
| M5 | Spec H2.5, H3 und „Decisions made“ nachziehen | Nachgezogen | Spec Zeilen 25, 96, 131, 165 |
| S1 | Die Abweisung fragt auch den gewählten Ort | Schritt 4 von `ort_uebernehmen`, vor dem Schreiben | `anwendung.rs` in `ort_uebernehmen`, gehalten von `die_ortswahl_fragt_vor_dem_schreiben_…` |
| S3 | Eine verknüpfte `settings.toml` wird nicht ersetzt | `symlink_metadata` zuerst, `Schreibhindernis::Verweis` auch für einen verwaisten Verweis, mit der Zeile zum Eintragen | `einstellungen.rs:384-387`, Proben `eine_verknuepfte_settings_toml_wird_nicht_ersetzt`, `ein_verwaister_verweis_ergibt_ebenso_verweis_und_legt_nichts_an` |
| S4 | „Derselbe Ort“ entscheidet sich am Wert in der Datei | `derselbe` wird im Schreibweg unter der Sperre gefragt; die Tafel unterscheidet vier Ausgänge, darunter das Zurückschreiben | `einstellungen.rs:418-422`, Tafel in `ort_uebernehmen` |

**S2 ist bewusst nicht übernommen, und der Grund trägt.** Der Spec zählt „kein Text“ zu den unzulässigen Werten und nicht zu den Dateischäden. Mit `Spanned<String>` würde `notizordner = 5` die ganze Datei beiseitelegen. Die größere Form `Ortswert` ist korrekt und von `ein_notizordner_ohne_text_beschaedigt_settings_toml_nicht` gehalten. Ein Nebengewinn: „Ort wählen…“ ersetzt einen Wert ohne Text durch einen gültigen (`der_schreibweg_ersetzt_einen_wert_ohne_text`).

**Zwei Abweichungen beim Bau sind Verbesserungen und keine Verschiebungen.** Die kanonische Form wird gegen den Ablageordner lexikalisch **und** gegen dessen `canonicalize` gehalten. Und `ortwahl::zeigen` ruft den Abschluss auch beim Abbruch, damit `blatt_geschlossen` jedes Mal nachholt, was während des Dialogs liegen blieb. Beides steht in der Abweichungsnotiz von Schritt 3.3.

### 3. Sicherheit von `secrets.txt`

**Die Schutzort-Regel ist tragfähig.** Der Griff hält den Ort und, allein für einen Fehler, einen Schutzort (`heimgriff.rs:53-59`). `lesen` liefert den geltenden Ort und sonst den Schutzort (`heimgriff.rs:125-131`). `lage` liefert allein den Ort oder seinen Fehler (`heimgriff.rs:138-140`). Nachgeprüft habe ich, wer welche Frage stellt, mit `grep -rn "heimgriff::lesen\|heimgriff::lage" crates/krk-ui/src`:

- `lage` fragen genau zwei Stellen: F2 (`anwendung.rs:5118`) und `ort_uebernehmen` (`anwendung.rs:5260`) für die Frage, ob ein Wechsel vorliegt. Keine davon legt am Schutzort an. Die Probe `f2_nennt_den_fehler_und_legt_ohne_ort_nichts_an` hält außerdem, dass F2 `lesen` gar nicht ruft.
- Alle Schutzregeln fragen `lesen` und sehen damit den Schutzort: der Klartextzweig des Editors (`editormodell.rs:1338`, `:1352`), der Dateityp beim Übernehmen (`editormodell.rs:1388`), die Erkennung über Gerät und Inode (`appkit/editor.rs:2851`), die Vorschau (`appkit/vorschau.rs:1072`, `:1103`, `:1582`), der Inhaltsfilter (`tabs.rs:600`), die Wiederherstellung und das Schreiben der Sitzung (`anwendung.rs:8446`, `:9435`).
- Einen fest verdrahteten Vorgabeort gibt es außerhalb von Proben nur noch im Messmodus (`anwendung.rs:2006`), laut `grep -rn "des_benutzers\|im_benutzerverzeichnis(" crates/*/src`.

**Beim Wechsel im Betrieb habe ich keinen Klartextweg gefunden.** Die drei Stellen, an denen ein solcher Weg entstehen könnte:

- **Der Editor** kann während des Wechsels keine Datei des alten oder des neuen Orts halten. Die Frage `gehaltene_notizdatei` steht vor dem Dialog und zweimal danach, beide Male vor dem Schreiben. Sie fängt `secrets.txt` auch unter einer dritten Schreibweise, über `haelt_geheimnisse` (`ort.rs:395-410`). Eine laufende PIN-Abfrage ist ein Blatt, und „Ort wählen…“ kommt während eines Blattes nicht durch.
- **Die `secrets.txt` am alten Ort** ist danach eine gewöhnliche Datei. Ist sie nicht leer, beginnt sie mit der Kennung `KRKSEC` (`heimordner/tresor.rs:124`) und trägt dahinter Chiffrat. **Inference:** Zufällige Bytes sind mit überwältigender Wahrscheinlichkeit kein gültiges UTF-8. Der Editor weist sie dann als „kein Text“ ab (`text/datei.rs:553-556`) und kann sie weder zeigen noch als Klartext sichern.
- **Tabs und Vorschau** auf altem und neuem Ort lesen sofort neu (`ort_wechseln`, Nachzug nach `heimgriff::ersetzen`). Der Inhaltsfilter liest danach am alten Ort das Chiffrat, wie es der Entscheid `260926-0050_*_wie-weit-reicht-der-inhaltsfilter-…` für jede Stelle außerhalb des Notizordners hinnimmt.

**Ohne gültigen Ort: F2 legt nichts an, der Schutz bleibt.** Gilt kein Ort, setzt der Start den Schutzort auf den in `session.toml` gemerkten Ort, sonst auf `~/krkhome` (`anwendung.rs:1450-1454`, `ort.rs:350-358`). Der gemerkte Ort bleibt bei einem Fehler stehen (`ort::zu_merken`, `ort.rs:328-333`), so dass auch ein zweiter Start mit beschädigter Datei denselben Ordner schützt.

**Ein Restfall bleibt, und er ist schmal.** Auf den zwei frühen Ausgängen des Starts (Ablageordner nicht zu öffnen, Schreibsperre nicht zu nehmen) gibt es keine gelesene Sitzung. `sitzung_laden` liefert dort `Sitzung::default()` (`anwendung.rs:2067`, `:2177`), also ist nichts gemerkt, und der Schutzort wird `~/krkhome`, auch wenn der Nutzer `/Volumes/X/notizen` eingestellt hat. Für diese eine Sitzung gilt dort dann nur, was für jede andere Stelle außerhalb des Notizordners gilt. Eine befüllte `secrets.txt` ist Chiffrat und öffnet nicht als Text. Eine **leere** `secrets.txt` am eingestellten Ort ginge über den Klartextweg, wenn der Nutzer sie in dieser Sitzung von Hand öffnet und beschreibt. Das ist der Fall, den der Spec unter „Constraints“ ausdrücklich offenlässt (Spec Zeile 182). Die Lage selbst ist schon eine Störung, und die Statuszeile meldet sie. Eine billige Abhilfe gibt es nicht, weil der gemerkte Ort genau in der Datei steht, die sich nicht lesen ließ. **Empfehlung:** hinnehmen und in `HowTo.md` beim Satz über den Schutz einen Halbsatz ergänzen. Ein Defekt wäre zu viel Aufwand für einen Fall, der zwei Störungen zugleich braucht.

### 4. Der Schreibweg in `settings.toml`

**Es gibt genau einen Schreiber, und er kann den Inhalt des Nutzers nicht zerstören.** `notizordner_schreiben` (`einstellungen.rs:378-436`) hat genau einen Rufer, `ort_uebernehmen`; die Proben `die_ortswahl_hat_genau_eine_rufkette` und die Zählprobe aus Schritt 3.3 halten das. Neben der Anlage beim ersten Start (`anlegen_falls_fehlt`, `einstellungen.rs:504-510`, schreibt nur, wenn die Datei fehlt) schreibt kein anderer Weg die Datei. Ein `grep` nach `Datei::Einstellungen` findet sonst nur Leser und das Neuerungsmodul.

Die Sicherungen, der Reihe nach:

1. **Ein Verweis wird nie ersetzt** (`einstellungen.rs:385-387`), auch kein verwaister.
2. **Nichts Beschädigtes wird geschrieben.** Kein UTF-8, kein gültiges TOML, ein unbekannter Schlüssel oder ein `notizordner`, der nicht als Einzelwert dasteht, ergeben `Beschaedigt` (`einstellungen.rs:391-416`). `ist_einzelwert` (`:445-448`) fängt den Fall ab, dass der Leser für `notizordner.x` oder `[notizordner]` den Bereich des Schlüssels meldet statt den eines Werts.
3. **Ersetzt wird genau der Byte-Bereich des Werts** (`:423-427`). Die Probe `der_schreibweg_ersetzt_allein_den_wertbereich_in_jeder_schreibweise` hält Kommentar hinter dem Wert, `'…'`, `"""…"""`, BOM und CRLF.
4. **Die zweite Lesung** (`pruefen`, `:482-498`) verlangt, dass das Ergebnis lädt, `notizordner == wert` trägt und jeden anderen Wert unverändert lässt. Sonst wird nichts geschrieben.
5. **Atomar**, mit übertragenen Rechtebits (`atomar.rs`, Kopf). Scheitert das Schreiben, bleibt die alte Datei stehen.
6. **Eine beschädigte Datei beim Start wird kopiert und nicht verschoben.** `beiseite_legen` schreibt eine Kopie neben die Datei (`ablage/mod.rs:861-885`), und das Original bleibt, wo es ist. Der Nutzer berichtigt also seine eigene Datei, nicht die Auslieferungsfassung.

**Die Restpunkte sind benannt und klein.** Erweiterte Attribute, Zugriffslisten und harte Verweise der ersetzten Datei gehen verloren, wie bei jeder Ablagedatei (`atomar.rs`, Kopf). Ein Textprogramm mit offenem Puffer kann den Wert beim eigenen Sichern zurückschreiben; `HowTo.md` sagt das (Zeilen 690-694). Zwischen `symlink_metadata` und `rename` kann ein fremdes Programm einen Verweis an die Stelle legen; der Modulkopf nennt diese Lücke (`einstellungen.rs:71-74`). Das Anhängen am Ende gilt nur, solange die Datei keine Tabelle kennt; auch das steht im Modulkopf (`:56-61`), und die zweite Lesung fängt den Fall ab, bevor geschrieben wird. **Keiner dieser Punkte gehört vor die Auslieferung.**

### 5. Die Stellung fehlender Funktionen einer eigenen Belegung

**Der Befund des Dokumentbearbeiters stimmt.** `Belegung::bauen` übernimmt zuerst die Funktionen der eigenen Datei in deren Reihenfolge. Danach hängt es jede Funktion, die die Datei nicht nennt, unbelegt **ans Ende der ganzen Liste**, in der Reihenfolge der Auslieferung (`tasten/belegung.rs:1871-1884`). `nach_bereichen` gruppiert diese Liste nach Bereich und behält dabei die Reihenfolge (`belegungsmodell.rs:977-1005`). In jeder Gruppe stehen also erst die Funktionen, die die eigene Datei dort nennt, und dahinter die fehlenden in der Folge der Auslieferung. Auf diesem Gerät nennt die eigene Datei unter „Home“ allein `notizzettel`. Die acht übrigen folgen in der Folge der Auslieferung, und „Ort wählen…“ steht deshalb an zweiter Stelle, genau wie im Spec.

**Welche Sätze stimmen und welche nicht:**

- `HowTo.md:481-484` ist schon richtig: „hinter den Befehlen, die die eigene Datei dort nennt“.
- `HowTo.md:83` („hängt KRK unbelegt hinten an ihre Gruppe“) ist nicht falsch, aber unvollständig. Man liest ihn leicht so, dass jede neue Funktion ganz ans Ende kommt. Wenn mehrere fehlen, stimmt das nicht.
- `README.md:103-105` („unbelegt hinten an ihre Gruppe an … und nicht an seinem Platz“) ist in seinem zweiten Teil falsch. Fehlen alle Nachbarn mit, steht der Befehl genau an seinem Platz, wie hier.
- Der Plan sagte in Schritt 3.3 „hinter ‚PIN ändern‘“ und in Schritt 3.5 „hinten an seine Gruppe“. Die Abweichungsnotiz von 3.3 hat das schon eingeordnet. Der Plan ist Aufzeichnung und bleibt, wie er ist.

**Der richtige Satz**, für die Tabellenzeile in `HowTo.md:83`:

> Die ausgelieferten Tastenkombinationen. KRK hängt die Funktion unbelegt an: in ihrer Gruppe hinter die Funktionen, die die eigene Datei dort nennt, und fehlen mehrere, untereinander in der Folge der Auslieferung. Über das Hauptmenü bleibt sie erreichbar.

Für `README.md:103-105`:

> KRK hängt eine Funktion, die die eigene Datei nicht nennt, unbelegt an: in ihrer Gruppe hinter die Funktionen, die die eigene Datei dort nennt, und fehlen mehrere, untereinander in der Folge der Auslieferung. Der Befehl steht im Hauptmenü und tut, was er soll, aber ohne Tastenkombination und nicht unbedingt an dem Platz, den die Auslieferung ihm gibt.

### 6. Auslieferbarkeit als 2.0.0 und die Prüfliste

**Major ist nach `README.md`, `### Versionsstufen`, richtig, und zwar schon wegen des Grundpakets.** Zwei Auslöser der Stufe greifen. Erstens ändert sich die Bedeutung eines Tastenbefehls: `notizzettel` hieß „Notizzettel anzeigen“ und öffnete das Notizblatt, jetzt heißt er „Notizordner öffnen“ und führt in einen Ordner (`git diff v1.11.1..HEAD -- resources/default-keymap.toml`). Zweitens werden `note-1.txt` und `note-2.txt` unter `~/Library/Application Support/KRK/` nicht mehr gelesen, wie sie geschrieben wurden (`e6d7bc7`). Die Erweiterung bringt neue Fähigkeiten dazu (Minor), die in der Major-Stufe aufgehen. Die Zahl lautet also **2.0.0**.

**Was vor der Auslieferung zu tun ist**, der Reihe nach:

1. **Die Nutzerprüfung**, nach der Liste unten.
2. **Eine Antwort auf `260926-1506_*_…`.** Der Plan verlangt sie vor der Auslieferung von Stufe 2 (`## Where this work stops`).
3. **Drei Sätze berichtigen:** `HowTo.md:557` (Frage 1), `HowTo.md:83` und `README.md:103-105` (Frage 5). Nach dem Plan ist dafür der `code-implementer` zuständig, der Schritte 1.3, 2.5 und 3.6 geschrieben hat; der `document-editor` kann es ebenso.
4. **Den Arbeitsbaum säubern.** `fusion-workbench/orchestrator-events.jsonl` ist geändert, und Station 1 von `cargo xtask release` bricht an jeder geänderten verfolgten Datei ab (`README.md`, Absatz nach `### Versionsstufen`).
5. **Nach der Prüfung** die Marker von Spec und Plan und den Zustand des Arbeitspakets nachziehen, über `state-auditor` oder `/fusion:reconcile`.

**Nicht vor der Auslieferung nötig:** die offenen Defekte `260926-0257` (ein ungelesenes Feld), `260926-0840` (`cmd+z` im Umbenennungsfeld; die Nutzerprüfung sieht nach), `260926-1012` (eine Probe, die einmal von drei Läufen fehlschlug; der heutige Lauf war grün) und `260926-1527`, sobald der Satz in `HowTo.md` die Wiederholung nennt. Außerdem die Restpunkte aus Frage 3 und 4.

**Die Prüfliste.** Grundpaket und Erweiterung zusammengelegt, Doppeltes gestrichen, nach Risiko geordnet. Jede Zeile nennt Handlung und erwartetes Ergebnis, damit ein Agent mit Bildschirmsteuerung sie später fahren kann. Die Kürzel sind die der Auslieferung; bei einer eigenen Belegung geht jeder Befehl auch über das Menü. `<ORT>` ist der jeweils geltende Notizordner, `<KRK>` ist `~/Library/Application Support/KRK`. Zeilen 2 bis 5 ändern `settings.toml`; Zeile 0 sichert sie vorher.

```text
 0 Vorbereitung: KRK beenden; cp -R "<KRK>" ~/krk-sicherung; cp -R ~/krkhome ~/krkhome-sicherung; KRK aus einem Terminal im Vordergrund starten.
 1 Geheimnis: F2; secrets.txt wählen, F4, PIN 1234 zweimal, Eintrag mit Text GEHEIM123, cmd+s. Terminal: head -c6 <ORT>/secrets.txt = KRKSEC; grep -c GEHEIM123 <ORT>/secrets.txt = 0. Editor schließen, F4: falsche PIN -> „PIN falsch oder Datei verändert“; richtige -> Tabelle.
 2 Kein Ort bei Schaden: KRK beenden; in <KRK>/settings.toml die Zeile tippfehler = 1 anhängen; starten. Erwartet: Statuszeile nennt den Schaden. F2: Grund und zwei Wege, kein Tab; ls ~/krkhome und ls <ORT> unverändert. Tab auf <ORT> von Hand öffnen, secrets.txt wählen: Vorschau zeigt nur den Hinweis; F4 verlangt die PIN.
 3 Weg heraus ohne Neustart: Zeile aus 2 entfernen; Home -> Ort wählen…, <ORT> wählen. Erwartet: F2 öffnet <ORT>. KRK neu starten: keine Meldung zum Ort.
 4 Unzulässiger Wert: notizordner = "notizen" eintragen, neu starten: Statuszeile nennt „notizen“; F2 öffnet nichts. Dann notizordner = "~/gibtsnicht/x", neu starten, F2: Meldung nennt Ort und Grund, ls ~/gibtsnicht scheitert. Wert zurücksetzen.
 5 Wechsel im Betrieb: Home -> Ort wählen…; Dialog beginnt bei <ORT>; neuen Ordner ~/krk-test anlegen und wählen. Statuszeile nennt neuen und alten Ort; ls ~/krk-test leer. F2: Tab mit leerer notes.txt, tasks.txt, secrets.txt (notes.txt leer, auch wenn note-1/2 Text tragen).
 6 settings.toml unberührt: diff ~/krk-sicherung/settings.toml "<KRK>/settings.toml" zeigt allein die Zeile notizordner; alle Kommentare stehen.
 7 Alter Ort ist gewöhnlich: Tab auf den alten Ort, notes.txt wählen: Vorschau zeigt Text, nicht gerendert; alte secrets.txt mit F4: keine Tabelle, kein Klartext („kein Text“ o. ä.). Ein Tab, der vorher schon auf dem alten Ort stand, zeigt das ohne Neuwahl.
 8 Abweisung: in ~/krk-test secrets.txt mit PIN öffnen; Ort wählen…: kein Dialog, Statuszeile „Zuerst secrets.txt im Editor schließen …“.
 9 esc und Blätter: Editor schließen; Ort wählen…, esc: Dialog zu, keine Meldung. Eine Datei umbenennen (shift+F6) und bestätigen, dann Ort wählen…, esc: Dialog zu. Ort wählen…, cmd+w, dann cmd+q: KRK verhält sich wie bei jedem Blatt; diff aus 6 unverändert.
10 Verweis: Ort wählen… über ~/Dropbox (oder einen anderen Verweis) wählen. Notieren, welche Form in settings.toml steht. F2 findet den Tab; notes.txt dort gerendert, über den Weg durch den Verweis und über den aufgelösten Pfad.
11 Rückweg: Ort wählen… <ursprünglicher ORT>; neu starten: keine Ortsmeldung; zweimal F2 ergibt einen Tab. Danach ~/krk-test löschen.
12 Menü: „Home“ ist das zweite Obermenü und nicht hinter der Kamerakerbe; Folge: Notizordner öffnen, Ort wählen…, Eintrag hinzufügen, bearbeiten, nach oben, nach unten, löschen, Aufgabe abhaken oder öffnen, PIN ändern. Unter „Anwendung“ und „Editor“ keiner davon. Beim Start keine Abweisung der eigenen keymap.toml.
13 Ausgrauung: mit einer gewöhnlichen Textdatei im Editor sind die sechs Eintragsbefehle und PIN ändern ausgegraut; jeder Eintrag wirkt über das Menü wie über seine Taste.
14 F1: Abschnitt „Home“ vorhanden; Ort wählen… eine freie Taste geben, „Fertig“, Taste öffnet den Dialog. cmd+r NICHT drücken.
15 Seitenwege: KRK mit --tasten-protokoll aus dem Terminal starten, PIN eingeben, in secrets.txt tippen: nur (verdeckt). grep -c GEHEIM123 "<KRK>/bookmarks.toml" = 0. In einer Zelle von secrets.txt „teh “ und "x" tippen, sichern, neu öffnen: unverändert.
16 Filter: in <ORT> Content an, GEHEIM123 tippen: kein Treffer auf secrets.txt. shift+cmd+h: secrets.txt bleibt sichtbar.
17 PIN ändern (shift+cmd+p), neu starten: nur die neue PIN öffnet secrets.txt.
18 Tabellen: tasks.txt drei Aufgaben, mittlere hoch, Kästchen ab- und aufhaken, eine löschen, cmd+z, sichern. notes.txt: mehrzeilig mit return, cmd+return übernimmt, „## x“ im Text abgewiesen. Vorschau beider Dateien hell und dunkel.
19 Bekannter Defekt 0840: Datei im Editor öffnen und tippen; im Dateifenster Umbenennen beginnen, ohne Tippen cmd+z: Verliert der Editor sein Getipptes? Ergebnis notieren.
```

Die Liste hat 20 Zeilen, die Vorbereitung eingeschlossen. **Nicht in der Liste** ist das Löschen von `~/krkhome`, um die Übernahme der Zettel noch einmal zu sehen: mit echten Notizen darin ginge Arbeit verloren. Die Zeile 5 prüft stattdessen, dass an einem anderen Ort nichts übernommen wird, und die Wiederholung am Vorgabeort ist als Defekt `260926-1527` belegt.

### 7. Der offene Entscheid `260926-1506_*_…`

**Ich stimme weiter Möglichkeit 1 zu, in der geschärften Fassung, die gebaut ist.** Die Begründung trägt unverändert: bei einer beschädigten Datei weiß KRK nicht, ob der Nutzer einen anderen Ort eingestellt hat, und ein Ersatzort legte dort Dateien an, wo er nicht mehr notiert.

**Der Bau hat die Möglichkeit um einen Teil erweitert, den der Datensatz nicht nennt.** Das ist der Schutzort: F2 legt nichts an, aber die Regeln für `secrets.txt` gelten am zuletzt gemerkten Ort weiter, sonst an `~/krkhome` (Plan, Abweichung bei Schritt 2.4). Damit übernimmt Möglichkeit 1 für den Schutz den Gedanken aus Möglichkeit 3, ohne deren Preis für F2. Der „Preis“ im Datensatz wird dadurch kleiner. Ein Tippfehler an `terminal` legt F2 still, schaltet aber den Schutz der Geheimnisdatei nicht ab. **Empfehlung:** Der Nutzer beantwortet mit Möglichkeit 1, und die `Answered:`-Zeile nennt den Schutzort ausdrücklich, damit Datensatz und Code dasselbe sagen. Ohne diesen Zusatz stünde im Datensatz eine Regel, die schwächer ist als die gebaute.

## Recommendations

**Vor der Auslieferung:**

1. **Nutzerprüfung** nach der Liste unter Frage 6. Wer: der Nutzer, später ein Agent mit Bildschirmsteuerung.
2. **`260926-1506_*_…` beantworten**, Möglichkeit 1 mit dem Schutzort in der `Answered:`-Zeile. Wer: der Nutzer; den Datensatz zieht danach `analyst` oder der Orchestrator nach.
3. **Drei Sätze berichtigen**, Wortlaut unter Frage 1 und 5: `HowTo.md:557`, `HowTo.md:83`, `README.md:103-105`. Wer: `code-implementer` (Executor der Anleitungsschritte im Plan) oder `document-editor`. Danach `260926-1527` als Lage angenommen schließen.
4. **`orchestrator-events.jsonl` eintragen**, damit Station 1 nicht abbricht.
5. **Auslieferung als 2.0.0**, `./release.sh 2.0.0`, auf ausdrücklichen Auftrag.

**Später:**

- In `HowTo.md` beim Schutz-Absatz ergänzen, dass die Regel auf den zwei frühen Startausgängen `~/krkhome` schützt, weil die Sitzung dann nicht gelesen ist (Frage 3, Restfall).
- `260926-0840` nach dem Befund aus Zeile 19 der Prüfliste bewerten; `260926-0257` und `260926-1012` wie in der Schlussdurchsicht des Grundpakets empfohlen.
- Die Einmaligkeit der Zettelübernahme, wenn der Nutzer sie will (`260926-1527`, erster Abschlussweg).
- Ein voller Abnahmelauf der zehn Zeitzusagen nach der Auslieferung. L4 und L7 stehen laut `CLAUDE.md` ohnehin daneben, und die Erweiterung fügt dem Start ein Obermenü und ein Sitzungsfeld hinzu.

## Open Questions

- [ ] Die Antwort des Nutzers auf `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`.
- [ ] Ob `NSOpenPanel` einen gewählten symbolischen Verweis selbst auflöst und welche Form deshalb in `settings.toml` landet; Zeile 10 der Prüfliste zeigt es (`ortwahl.rs`, Modulkopf, „Was hier nicht nachgelesen ist“).
- [ ] Ob `esc` den Ordnerdialog nach einem liegengebliebenen Blattgriff erreicht; M1 ist am Quelltext gehalten, das Verhalten zeigt erst Zeile 9.
- [ ] Ob elf Obermenüs auf dem Bildschirm des Nutzers Platz finden (Zeile 12).

## Sources

- Spec `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`, Plan `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md`, beide unter `work-packages/260925-2356-f2-oeffnet-krkhome-statt-notizfenster/plans/`
- Zweitlesung `260926-1520-zweitlesung-home-menue-und-einstellbarer-ort.md` und Schlussdurchsicht `260926-1047-schlussdurchsicht-f2-krkhome.md` (diese Ablage)
- Entscheide `260926-1447_*_bekommt-krkhome-ein-eigenes-menue-und-einen-einstellbaren-ort.md`, `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`, `260926-0050_*_wie-weit-reicht-der-inhaltsfilter-…`
- Defekte `260926-1527_*_…`, `260926-0840_*_…`, `260926-0257_*_…`, `260926-1012_*_…`
- Code: `crates/krk-ui/src/heimgriff.rs`, `crates/krk-core/src/heimordner/{ort,bereitstellen,tresor}.rs`, `crates/krk-core/src/ablage/{einstellungen,atomar,mod}.rs`, `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/src/text/datei.rs`, `crates/krk-ui/src/{belegungsmodell,menuemodell,editormodell,tabs}.rs`, `crates/krk-ui/src/appkit/{anwendung,editor,vorschau}.rs`, `crates/krk-ui/src/appkit/blaetter/{mod,ortwahl}.rs`
- Dokumente: `HowTo.md` (Zeilen 83, 481-484, 555-566, 690-708), `README.md` (Zeilen 103-105, `### Versionsstufen`), `resources/default-keymap.toml`
- Lauf: `make check` auf `67e3293`, 2189 bestanden, 0 fehlgeschlagen, 17 ignoriert, alle fünf Kommandos grün
- `~/Library/Application Support/KRK/keymap.toml` (nur gelesen, Zeile 459)
