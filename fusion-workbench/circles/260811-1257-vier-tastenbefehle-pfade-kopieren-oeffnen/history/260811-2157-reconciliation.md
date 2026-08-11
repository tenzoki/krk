# Abschluss-Abgleich der Runde 4: Vier Tastenbefehle für Pfade, das Öffnen und Cmd+W

**Datum:** 2026-08-11, 21:57
**Agent:** `reconciler`, Bereich `code`
**Circle:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (aktiv, `_t_`)
**Anker:** `55a4afa` (HEAD bei Sitzungsbeginn) bis `b2a6c2e`, 16 Commits
**Status:** Complete

---

## Das Ergebnis in drei Sätzen

**Der Baum deckt jede Behauptung, die diese Sitzung aufgestellt hat.** Fünf Planschritte, zwölf
geschlossene Defektdatensätze und sieben beantwortete Entscheidungsfragen sind einzeln gegen den
Programmtext gelesen, nicht aus den Datensätzen übernommen; keine einzige Behebung fehlt im Code.
`make check` läuft grün, mit 795 bestandenen Proben in 16 Zielen und null Warnungen unter
`-D warnings`.

**Abgenommen ist die Runde damit nicht.** Alle 62 Abnahmekriterien des Specs stehen auf `- [ ]`, und
der Abgleich hakt keines ab. 23 davon trägt der Baum schon heute, 39 kann nur ein Mensch am
laufenden Bündel sehen.

**Gefunden hat der Abgleich vier Dinge**, keines davon am Verhalten der vier Befehle: eine halb
gelaufene Behebung, zwei Abweichungen zwischen Plan und Baum, fünf veraltete Stellen in `CLAUDE.md`
und eine Lücke im Ereignisprotokoll, die einen neuen Datensatz bekommen hat.

---

## 1. Die fünf Planschritte

Jeder trägt `[DONE]`, jeder ist am Code nachgelesen. Die Belege mit Datei und Zeile stehen im
`## Reconciliation Log` des Plans; hier steht das Ergebnis.

| Schritt | Gegenstand | Stand |
|---|---|---|
| S1 | drei Funktionen in der Belegung, an vier Stellen nachgetragen | trägt |
| S2 | die beiden Pfadkopierer und die Schreibseite der Zwischenablage | trägt |
| S3 | Öffnen mit dem Standardprogramm auf der Eingabetaste | trägt |
| S4 | der Doppelklick auf eine Zeile des Dateifensters | trägt |
| S5 | Cmd+W schließt den aktiven Tab aus jedem Fokus | trägt |

**Die harten Zahlen sind maschinell nachgezählt und nicht abgeschrieben.** `resources/default-keymap.toml`
führt 74 Blöcke `[[funktion]]` und 82 Einträge in den `tasten`-Listen, wie die Kopfzeile bei `:33`
sagt. `Kommando` trägt 68 Varianten, und dieselbe 68 steht als Länge im Typ von `KENNUNGEN`
(`crates/krk-core/src/tasten/belegung.rs:488`), wo der Übersetzer sie hält. `Wirkungsbereich` trägt
unverändert sieben Werte. `Kommando::wirkungsbereich` und `bereich_des_kommandos` tragen je null
`_`-Zweige.

**Die offene Frage des Plans ist beantwortet, und mit einem Beleg statt einer Annahme.** S4 sollte
klären, ob `NSTableView` die Eigenschaft `target` schwach führt. Der `SAFETY`-Block bei
`crates/krk-ui/src/appkit/tabelle.rs:2300-2340` zitiert die erzeugte Bindung
(`objc2-app-kit-0.3.2/…/NSControl.rs:91-93`, „This is a weak property") und den SDK-Kopf
(`NSControl.h:24`, `@property (nullable, weak) id target;`). Der Haltering bleibt offen; das
schwach haltende Zwischenobjekt, den zweiten Ausgang des Schrittes, braucht der Weg nicht.

### Zwei Abweichungen zwischen Plan und Baum

Beide sind begründet, beide unschädlich, beide gehören genannt.

1. **Aus einer Funktion sind drei geworden.** Der Plan führt unter `## Frage 6` eine gemeinsame
   öffentliche Funktion `nichts_betroffen() -> String` für beide Befehle. Im Baum stehen zwei
   öffentliche Funktionen, `nichts_zu_kopieren()` und `nichts_zu_oeffnen()`, über einem privaten
   Rumpf `nichts_betroffen(verb)` (`crates/krk-ui/src/kommandos/operationen.rs:858`). Ursache ist der
   Durchsichtsbefund `issues/260811-1916_*_der-satz-fuer-die-leere-menge-sagt-nicht-dass-nichts-zu-kopieren-war.md`:
   C2 verlangt im Wortlaut, dass die Statuszeile sagt, **dass nichts zu kopieren war**. Der Plan ist
   an dieser Zeile überholt, und die Änderung ist die richtige.
2. **Eine Signatur ohne Rückgabewert.** S3 und S4 nennen `mit_standardprogramm_oeffnen(&self, pfade:
   &[PathBuf]) -> bool`. Im Baum liefert sie nichts (`tabelle.rs:940`). Kein Aufrufer braucht den
   Wert: der Zweig in `kommando_ausfuehren` läuft ohnehin in den gemeinsamen Rückgabewert `true`, und
   der Doppelklick ist kein Kommando. Eine Abweichung in der Form, keine in der Sache.

### Ein Abnahmekriterium, das zu eng gefasst ist

S3 sagt zu, `grep -rn "NSWorkspace" crates/krk-ui/src` finde vier Stellen. Es findet Treffer in
**sieben** Dateien. Drei davon sind Prosa: `kommandos/operationen.rs:898`, `appkit/mod.rs:77-88` und
`appkit/anwendung.rs:21` nennen die Klasse in einem Kommentar. **Aufrufe** stehen weiterhin in genau
vier Dateien — `volumes.rs`, `terminal.rs`, `zwischenablage.rs` und dem neuen `standardprogramm.rs`.
Das Kriterium ist in der Sache erfüllt und in seinem Wortlaut falsch formuliert: es zählt Textstellen
und meint Aufrufe.

---

## 2. Der Spec: was der Baum trägt und was nur ein Mensch sehen kann

**Kein Kästchen ist abgehakt, und das ist richtig.** Der Kopf des Specs sagt selbst, der Marker bleibe
`_o_`, bis die Abnahmekriterien eingelöst sind. Der Abgleich löst keines ein; er sortiert sie nur
danach, woran sie sich prüfen lassen. Die Tabelle mit den einzelnen Belegen steht im
`## Reconciliation Log` des Specs.

| Gruppe | Zahl | Woran prüfbar |
|---|---|---|
| A | 23 | am Programmtext, an Zahlen in Dateien, oder auf der Verbotsseite — **alle 23 halten heute** |
| B | 32 | nur am laufenden Bündel: Verhalten einer Taste, eines Klicks, der Statuszeile |
| C | 7 | am Bündel, aber mit Prüfaufbau: Verknüpfung, verschwundener Ordner, dreißig markierte Einträge |

**Wer die 23 der Gruppe A bei der Abnahme abhakt, hakt etwas ab, das steht.** Der Abgleich tut es
nicht an seiner Stelle: das Abhaken bezeugt die Abnahme und nicht die Prüfung, und diese Runde soll
nicht als abgenommen gelten, weil sie es nicht ist.

**Ein Kriterium sagt mehr zu, als der Mechanismus beantworten kann.** C5 verlangt, die
Konflikterkennung melde für die drei neuen Kombinationen nichts, „weder gegen eine andere Funktion
noch gegen ein Menükürzel". `Belegung::konflikte` (`crates/krk-core/src/tasten/belegung.rs:1014`)
vergleicht nur innerhalb desselben `gehalten_von`; ein Menükürzel und eine Belegung ohne Zusteller
sind danach nie ein Konflikt. So ist es entschieden
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`).
Der zweite Halbsatz ist damit nicht prüfbar, sondern durch die Bauart ausgeschlossen. Kein Defekt,
aber ein Kriterium, das eine Prüfung verspricht, die es nicht gibt.

---

## 3. Die zwölf geschlossenen Defektdatensätze

**Jede Behauptung ist gegen den Baum gelesen. Alle zwölf tragen.** Jeder Datensatz hat einen
`Abgleichsvermerk 260811-2157` mit Datei und Zeile bekommen.

| Datensatz | Behauptung | Nachgesehen |
|---|---|---|
| `shared/…/260811-1648` Untergrenzen | 31 von 33 Dateien unter `appkit/` tragen den Abschnitt | 33 Dateien, 21 von 23 plus alle 10 unter `blaetter/` = 31; ohne: `koordinaten.rs`, `mod.rs` |
| `shared/…/260811-1245` Vorschaubreite | `bildschirmbreiten_uebernehmen` am Kopf von `kommando_ausfuehren` | `anwendung.rs:2048`, Funktion bei `:2577`, zweiter Rufer `sitzung_bauen` bei `:4287` |
| `shared/…/260810-1906` `must_use` | Attribut an `Auswahlversuch` **und** an `Einzug` | `tabs.rs:270` und `:297`, beide mit Begründungstext |
| `shared/…/260811-1500` Iconset | `CFBundleIconFile`, `.icns` beim Bündeln erzeugt | `Info.plist:114`, `xtask/src/bundle.rs:364`; eigener Bündelbau gefahren, Datei liegt mit 298.129 Bytes |
| `shared/…/260811-1730` Ziffern | festbreite Ziffern in Liste und Leiste, zwei Formatierer | `tabelle.rs:2117-2118`, `leiste.rs:489` |
| `<circle>/…/260811-1648` Rumpf-Stände | alle Stände auf „answered" gezogen | trägt — **aber nur zur Hälfte, siehe unten** |
| `<circle>/…/260811-1916` ×6 (Durchsicht) | sechs Befunde behoben | alle sechs am Baum belegt, Zeilen im Vermerk je Datensatz |

### Der eine Fund: eine Behebung ist halb gelaufen

`<circle>/issues/260811-1648_c_fuenf-entscheidungsdatensaetze-tragen-im-rumpf-noch-den-stand-offen.md`
beschreibt seine Behebung selbst als **zwei** Handgriffe: „`**Status:** open` wird zu
`**Status:** answered`, **und der leere Vorlagenblock entfällt**". Gelaufen ist der erste. Sechs der
sieben Datensätze trugen am 260811-2157 weiterhin den leeren Block
`Answered:` / `Implemented:` / `Deferred:` / `Superseded by:` **und** darunter den ausgefüllten;
allein `260811-1612` war sauber.

**Die Resolved-Notiz ist damit nicht falsch, sondern unvollständig** — sie behauptet den zweiten
Handgriff nicht. Der Abgleich hat die sechs leeren Blöcke entfernt.

Nebenbei: die Notiz berichtigt die Zahl im Titel („es waren sechs und nicht fünf") und lässt den
Titel bewusst stehen. Das ist richtig begründet und bleibt so.

---

## 4. Die Entscheidungsdatensätze

**Sieben Fragen dieses Circles standen auf beantwortet, und ihre Antworten stehen sämtlich im Code.**
Der Abgleich hat jede einzeln am Baum belegt und alle sieben von `_a_` auf `_i_` gezogen, mit einer
`Implemented:`-Zeile samt Commit und Fundstelle.

| Datensatz | Antwort | Umgesetzt in |
|---|---|---|
| `260811-1257` wie weit soll Cmd+W reichen | nur die Fokuslücke | `3d48f34`, `belegung.rs:710`, `anwendung.rs:2234` |
| `260811-1258` was kopiert der Pfadkopierer | `betroffene()` erben | `d23bfdb`, `tabelle.rs:904` |
| `260811-1259` was tut ein Doppelklick auf einen Ordner | die Maus verzweigt, die Taste nicht | `5487695`, `tabelle.rs:1005` und `:847` |
| `260811-1300` welche Kombinationen ab Werk | drei neue, Cmd+W bleibt | `a358d86`, `default-keymap.toml:501,512,526` |
| `260811-1552` welche Sorten in die Zwischenablage | nur Text | `d23bfdb`, `zwischenablage.rs:178-181` |
| `260811-1612` öffnet `return` alle betroffenen | alle betroffenen | `cee5276`, `tabelle.rs:847` |
| `260811-1648` fragt KRK vor dem Öffnen nach | keine Nachfrage | `cee5276`, auf der Verbotsseite belegt: keine Schwelle in `tabelle.rs:940` |

**Ein achter Datensatz ist von der Lage überholt worden**, und zwar in einem anderen Circle.
`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_wird-der-vorschaubreiten-defekt-in-dieser-runde-behoben.md`
fragt, ob der Vorschaubreiten-Defekt in jener Runde behoben wird oder in einer eigenen davor. Er ist
am 260811 in **dieser** Runde behoben worden (`1ea5a3d`), was im Ergebnis Möglichkeit 2 ist. Der
Datensatz ist auf beantwortet gezogen, mit der ausdrücklichen Notiz, dass die Lage geantwortet hat
und nicht der Nutzer. **Auf umgesetzt gezogen ist er nicht**: ob er als eingelöst oder als überholt
gilt, entscheidet, wer jenen Circle aktiviert. Die harte Vorbedingung jenes Circles ist damit weg,
und seine siebte Frage ist gegenstandslos.

---

## 5. Die drei offenen Defektdatensätze

**Alle drei stehen zu Recht offen.** Keiner ist durch diese Runde erledigt worden.

**`shared/issues/260810-1945` — keine Aufgabenereignisse.** Der Befund gilt unverändert und hat sich
in dieser Sitzung wiederholt. Ab dem letzten `session_start` (2026-08-11T12:55:10) trägt
`orchestrator-events.jsonl` neun Ereignisse und **kein einziges** `task_start`, `task_done` oder
`commit`, während git für dieselbe Spanne 16 Commits zählt. Daneben fehlen `scope_resolved` und
`queue_built`, obwohl das Sitzungsprotokoll beides ausschreibt — was der Diagnose jenes Datensatzes
widerspricht, die Grenzereignisse hingen an Schritten, an denen die Sitzung ohnehin anhält.

**`shared/issues/260810-1907` — kein Durchsichtsdokument für Turn 2.** Der Bestand ist unverändert:
`shared/reviews/` trägt weiterhin genau eine Datei. **Die Lehre daraus hat in dieser Runde
gegriffen**: die Durchsicht von Turn 1 liegt als eigenes Dokument vor
(`reviews/260811-1916-coderev-vier-tastenbefehle-turn-1.md`), mit einem ausgeschriebenen Abschnitt
über das, was ohne Befund geblieben ist. Offen bleibt der Datensatz trotzdem, denn er hält eine Regel
fest und keine einzelne Datei. Wer ihn schließen will, schließt ihn als Lage angenommen und nicht als
behoben.

**`circles/260811-1304-…/issues/260811-1732` — die Spaltenschalter.** Eine Zuschnitt-Erweiterung für
einen noch nicht gefahrenen Circle, abgelegt, damit sie bei der Aktivierung gefunden wird. Genau
richtig offen. Der Beifund aus der Vorschaubreiten-Behebung gehört dazu und ist in jenem Circle
vermerkt: `MINDESTGROESSE` steht auf 780 Punkten, der Vierersatz mit dem Editor summiert sich auf 920,
und zwischen beiden Zahlen wird der Editor unter sein Mindestmaß gedrückt.

---

## 6. Ein neuer Datensatz

`shared/issues/260811-2157_o_fuenf-commits-stehen-hinter-dem-letzten-turn-ende-ohne-eigene-turn-grenze.md`

Das Ereignisprotokoll führt für diese Sitzung genau ein Turn-Paar: `turn_start` um 15:26:46,
`turn_end` um 17:35:39. **Fünf Commits sind danach entstanden** — `8695b77`, `3d2c613`, `9b17ff1`,
`1ea5a3d`, `b2a6c2e` — ohne dass ein zweiter `turn_start` emittiert worden wäre. Damit fehlt nicht
nur die Maschinenlesbarkeit, die der Schwesterdatensatz `260810-1945` einräumt, sondern der Nachweis
über die Gliederung der Sitzung: wer später fragt, in welchem Arbeitszyklus der Icon-Bau oder die
`must_use`-Entscheidung gelaufen ist, findet einen Turn, der um 17:35 endet, und fünf Commits danach.

Er liegt im gemeinsamen Speicher, bei seinen zwei Schwesterdatensätzen, die der Orchestrator am
260811-1950 aus demselben Grund ausdrücklich dort belassen hat: er betrifft die Durchführung der
Sitzung und nicht den Gegenstand der Runde.

---

## 7. `CLAUDE.md`: fünf Abweichungen, ausdrücklich geprüft

Die Datei ist in dieser Sitzung nicht nachgezogen worden. **Der Abgleich ändert sie nicht** — das war
die Auflage. Er zählt nach und berichtet.

| Stelle | Was dort steht | Was der Baum sagt |
|---|---|---|
| `:98` | „`Auswahlversuch` trägt kein `#[must_use]`, ein sechster Aufrufer bricht sie also unbemerkt" | **Falsch seit `b2a6c2e`.** Das Attribut steht bei `tabs.rs:270`, ein zweites bei `:297` an `Einzug`. Der Übersetzer erzwingt die Behandlung, und `clippy -D warnings` macht sie zum Fehler. Der ganze Absatz beschreibt die Lage von gestern, bis hin zum Satz über die eigentliche Frage — die ist entschieden. |
| `:56` | „`Kommando` … 65 Varianten statt 53" | **68.** Nachgezählt in der Aufzählung und in der Länge im Typ von `KENNUNGEN` (`belegung.rs:488`). Die drei anderen Zahlen desselben Satzes stimmen weiterhin: `Wirkungsbereich` sieben, `Bereich` fünf, `Fokus` fünf. |
| `:122` | „jedes AppKit-Modul dieses Projekts nennt in seinem Modulkopf die Untergrenze jeder Klasse, die es anspricht" | **31 von 33.** `9b17ff1` hat 26 nachgetragen; ohne den Abschnitt sind `koordinaten.rs` (rechnet auf Versätzen, spricht keinen Typ an) und `mod.rs` (Modulwurzel). Der Satz war zum Zeitpunkt des Schreibens weit falscher als heute, und im Wortlaut ist er es weiterhin. |
| `:146` | „Zwei Circles sind vorgesehen und nicht gefahren, der Web-Betrachter im Vorschaufenster und **die Tastenbelegung als Markdown**" | **Falsch.** Die Tastenbelegung als Markdown ist die Runde 3 und liegt seit `1055500` als beschränkter Abschluss vor (`circles/260809-2040-…/_b_circle.md`). Vorgesehen sind heute der Web-Betrachter und die Statusleiste mit Bereichsschaltern (`circles/260811-1304-…/_a_circle.md`), von der die Datei nichts weiß. |
| durchgehend | die Runden 1 und 2 sind ausgeschrieben, „Geprüft am 260810-1930" | **Zwei Runden fehlen ganz.** `grep -c 'Runde 3\|Runde 4'` liefert 0. Der Abschnitt `## Projektstand` beschreibt den Stand vor der Belegungsausgabe und vor den vier Tastenbefehlen. Zeile 11 erklärt außerdem unqualifizierte Pfade weiterhin relativ zum Circle der Runde 2. |

**Zwei Erwartungen aus dem Auftrag treffen nicht zu, und das gehört dazu.** `CLAUDE.md` nennt
**keine** Zahl der Funktionen in `resources/default-keymap.toml` und **keine** Zahl der `KENNUNGEN`;
`grep -n 'Funktionen'` findet nur die Zeile im Verzeichnisbaum, die die Datei beschreibt. Was
veraltet ist, ist die Zahl der `Kommando`-Varianten, und die steht in der Zeile darüber.

**Zwei Kleinigkeiten daneben**, keine davon eine falsche Aussage: der Verzeichnisbaum bei `:25-44`
führt `iconset/` nicht, das seit `873b768` im Baum liegt; und die Aufstellung der Entscheidungsspeicher
bei `:130-132` nennt drei Circles, während sechs existieren — das `find` darunter fängt es ab, wie die
Datei selbst sagt.

---

## 8. Was der Abgleich nicht geprüft hat

**Nichts am laufenden Bündel.** Kein Tastendruck ist gefahren, keine Zwischenablage gelesen, kein
Doppelklick zugestellt. Was hier steht, steht am Programmtext, an Testläufen und an git.

**Die Messungen der Behebungsnotizen sind nicht nachgemessen.** Die Punktwerte im Datensatz zu den
festbreiten Ziffern und die Aussagen über das laufende System im Datensatz zur Vorschaubreite stehen
als Zahlen nur dort. Nachgesehen ist, dass der Programmtext das tut, was die Notizen beschreiben.

**Der Abnahmelauf der zehn Zeitzusagen ist nicht gefahren.** Er verlangt KRK im Vordergrund und ist
Nutzerarbeit; die Frage dazu ist unverändert offen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Die zehn Schwellen selbst sind unangetastet: `git diff 55a4afa..HEAD -- crates/krk-bench/` ist leer.

---

## 9. Zahlen des Durchgangs

| Gegenstand | Gelesen | Geändert |
|---|---|---|
| Pläne und Specs | 2 | 2 (Plan auf `_c_` mit Reconciliation Log; Spec bleibt `_o_`, mit Reconciliation Log) |
| Defektdatensätze | 27 (Circle + gemeinsam) | 15 (12 Vermerke an geschlossenen, 2 an offenen, 1 neu) |
| Entscheidungsdatensätze | 23 (alle Speicher) | 8 (7 auf `_i_`, 1 auf `_a_`) |
| Durchsichten | 3 | 3 (Vermerke) |
| Commits gelesen | 16 | — |
| Testläufe | `make check`, `make tasten` | — |

**Was der Abgleich nicht angefasst hat:** kein Code, keine Datei unter `crates/`, `resources/`,
`xtask/` oder `iconset/`, kein Circle-Datensatz. Der Übergang `_t_` → `_b_` ist Nutzerarbeit.

---

## Eine Randnotiz zum Werkzeug

`bin/fusion-rules reconciler` hat sechs Regeldateien ausgegeben und **keinen Pfad auf ein
Stilprofil** — weder `stilwerk/chat-voice-de.yaml` noch `stilwerk/default-voice-de.yaml`, obwohl
beide unter `fusion-workbench/stilwerk/` liegen. Der Abgleich hat die Sprachregel deshalb aus
`CLAUDE.md` aufgelöst (`**Language:** de`, keine Zeile `**Artifact language:**`, also Deutsch für
beide Flächen) und ohne Profil geschrieben. Es ist dieselbe Form wie
`shared/issues/260808-0017_c_fusion-rules-gibt-conceptrev-die-stilprofile-nicht-aus.md`, dort für
`conceptrev` erhoben und geschlossen. Ob es für `reconciler` ein eigener Befund ist, gehört in den
Arbeitsbereich des fusion-Plugins und nicht hierher; vermerkt ist es, damit es nicht ein zweites Mal
gesucht wird.
