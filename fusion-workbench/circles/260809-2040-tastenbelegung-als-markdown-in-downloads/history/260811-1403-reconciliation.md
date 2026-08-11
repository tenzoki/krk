# Abschluss-Abgleich: Die Tastenbelegung als Markdown im Downloads-Ordner (Runde 3)

**Datum:** 2026-08-11, 14:03
**Agent:** reconciler
**Domain:** code
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Sitzung:** `history/260811-0107-orchestrator-session.md`
**Umfang:** Commits `e43f21a..caf6375`, elf Stück
**Status:** Complete

---

## Was geprüft wurde und was dabei herauskam

Neun Defektdatensätze, sieben beantwortete Entscheidungen und drei Planschritte sind einzeln
gegen den Baum gelesen. **Keine behauptete Behebung fehlt im Code.** Drei Abweichungen sind
gefunden, alle drei in Beschreibungen und keine im Programmtext; sie stehen unten unter
`## Abweichungen`.

| Gegenstand | geprüft | gehalten | geändert |
|---|---|---|---|
| Planschritte | 3 gebaut, 1 gestrichen | 3 | Plan auf `_c_`, `**Status:** Complete`, Reconciliation Log |
| Defektdatensätze | 9 | 9 | keiner umbenannt, alle Marker stimmten |
| Entscheidungen | 7 beantwortet, 1 offen | 7 | 7 von `_a_` auf `_i_`, dazu sechs Kopffelder berichtigt |
| Durchsichten | 3 | 3 | je ein Abgleichs-Abschnitt angehängt |
| Spec | 1 | — | **nicht angefasst**, auf Weisung des Nutzers |

---

## 1. Die acht geschlossenen und der eine zurückgestellte Defekt

Jede `Resolved:`-Behauptung ist gegen die genannte Stelle im Baum gelesen. Alle acht halten.

| Datensatz | Behauptung | Beleg |
|---|---|---|
| `260811-0838_c_antwort-zeigen-nennt-vier-raenge-…` | `anwendung.rs` nennt jetzt fünf Ränge | `anwendung.rs:3334` "Rang 1, der oberste der fuenf Raenge"; `:3338` "Fenstermeldung auf Rang 3" |
| `260811-0930_c_die-ableitung-textfelder-und-editor-bricht-…` | der Datensatz ist selbst das Ergebnis, im Baum verankert | Probe in `appkit/menue.rs:845`, Zitat des Datensatzes in `belegungsausgabe.rs:296`, Dreiteilung in C3 des Specs |
| `260811-0955_d_der-auffangzweig-in-wirkung-ist-erreichbar-…` | Weg b) gebaut, Ungleichheit bleibt | `belegungsausgabe.rs:237` `NICHT_EINGEORDNET`, `:357` der Zweig, Probe `:822` |
| `260811-0956_c_der-nutzerentscheid-vom-260811-0935-…` | Datensatz nachgetragen, C3 berichtigt | `decisions/260811-1010_i_was-traegt-die-dritte-spalte-…md`, C3 des Specs |
| `260811-0957_c_gemessen-reicht-fuer-textfelder-weiter-…` | `inference:` gesetzt, eine Zählweise | `belegungsausgabe.rs:57`, `:65`, `:275`, `:719`; Probe `:684` |
| `260811-0958_c_elf-module-neben-appkit-sind-zwoelf-…` | nachgezählt und berichtigt | `crates/krk-ui/src/main.rs:17` "Zwoelf Module", `hervorhebung` in der Aufzählung |
| `260811-0959_c_zu-s1-bis-s3-gibt-es-keinen-sitzungsbericht-…` | die Substanz steht in der Sitzungsdatei | Abschnitt `## Turn 1` in `history/260811-0107-orchestrator-session.md` |
| `260811-1000_c_die-begruendung-fuer-den-downloads-ordner-…` | der Text nennt das Schreiben | `resources/Info.plist:179` |
| `260811-1210_c_eine-dritte-stelle-nennt-den-rang-…` | drei Stellen berichtigt, achtzehn geprüft | `anwendung.rs:3620`, `tabelle.rs:322` "der oberste der fuenf Raenge" |

**Die Zurückstellung von `260811-0955` ist eine Wahl und kein Rest.** Der Nutzer hat am
260811-1005 Weg b) gewählt und die beiden anderen abgelehnt. Gebaut ist, was Weg b) leistet: die
Verwechslung zwischen "hier ist nichts entschieden" und "hier hat niemand nachgesehen" gibt es
in der Ausgabedatei nicht mehr. Die Ungleichheit zwischen `belegungsmodell::bereich` und
`belegungsausgabe::wirkung` besteht unverändert fort, und der Datensatz führt sie.

**Eine Gegenprobe:** `grep -rn "vier Raenge" crates/` liefert null Treffer. Der Rangbefund ist
im ganzen Baum abgeräumt und nicht bloß an den drei gemeldeten Stellen.

---

## 2. Die drei Planschritte

S1 bis S3 sind gebaut; der Beleg je Schritt steht im `## Reconciliation Log` des Plans und wird
hier nicht verdoppelt. Kurz:

- **S1** misst über `AnyClass::responds_to` in `appkit/menue.rs` und hält drei Proben.
- **S2** trägt `Wirkungsbereich::beschriftung` in `krk-core/src/tasten/belegung.rs:269`, ohne
  Auffangzweig.
- **S3** steht vollständig: `belegungsausgabe.rs` mit 1065 Zeilen, eingehängt in `main.rs:45`;
  `nach_bereichen` und `tastenliste` herausgezogen; `gekuerzt_fuer_anzeige` im Kern; Menüeintrag
  und Selektor am Delegierten.

**Die Verbotsseite hält.** `resources/default-keymap.toml` und `crates/krk-ui/src/fenstertitel.rs`
sind im ganzen Commit-Bereich unberührt; die Belegung führt weiter 71 Funktionen.

**S4 ist gestrichen, und der Abgleich hakt deshalb kein Kriterium ab.** Die 41 Abnahmekriterien
des Specs stehen sämtlich auf `- [ ]`, geprüft mit `grep -c '^- \[ \]'` gegen `grep -c '^- \[x\]'`
(41 zu 0).

---

## 3. Die sieben beantworteten Entscheidungen sind umgesetzt

Alle sieben `_a_`-Datensätze dieses Circles sind auf `_i_` gegangen, jeder mit einer
`Implemented:`-Zeile, die die Stelle im Baum nennt. Der Reihe nach: die Auslösung über den
Menüeintrag, der Dateiname mit Überschreiben, Umfang und Gliederung, die dritte Spalte, der
gesicherte Stand bei offener Belegungsansicht, die Kürzung des Pfades mit Tilde, und "Editor"
für Rückgängig und Wiederholen.

**Eine Einschränkung gehört an den fünften.** Dass die Ausgabe bei offener Belegungsansicht den
gesicherten Stand schreibt, ist am Aufbau belegt: der Delegierte leiht die Belegung aus
`ivars().belegung` (`anwendung.rs:2255`), und die Ansicht arbeitet auf einer Kopie
(`anwendung.rs:2174`). Am laufenden Bündel beobachtet hat es niemand; das war Sache des
gestrichenen Schrittes S4. Die `Implemented:`-Zeile sagt es dazu.

**Sechs Kopffelder waren stehengeblieben.** Die sechs älteren Datensätze trugen im Kopf
`**Status:** open`, während der Dateiname `_a_` sagte und eine `Answered:`-Zeile im Rumpf stand.
Marker und Feld widersprachen sich. Beide stehen jetzt auf `implemented`. Es ist dieselbe
Fehlerform, die `shared/issues/260811-0932_*_die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md`
für den Circle-Datensatz führt: niemandes Prompt beauftragt das Nachziehen.

---

## Abweichungen

Drei, alle in Beschreibungen. Keine davon ist im Programmtext.

**1. Der Kopf des Plans nennt 40 Abnahmekriterien, der Spec führt 41.** Die Zeile `**Spec:**`
sagt "38 Abnahmekriterien, dazu zwei". Die Berichtigung von C3 am 260811-1038 hat aus der
einheitlichen Aussage über die sechs Textbefehle eine Dreiteilung gemacht und dabei ein Kriterium
hinzugefügt. Der gestrichene Schritt S4 nennt weiter unten im selben Plan bereits die 41; die
Kopfzeile ist die einzige Stelle, die die alte Zahl trägt. Dieselbe Zahl steht im
Maßstab-Absatz der Durchsicht `reviews/260811-1000-coderev-…`, dort mit einem Vermerk versehen.
Beide Stellen sind Beschreibungen und bleiben unverändert; ein Abgleich berichtigt Zustandsmarker
und schreibt keine Beschreibungen um.

**2. Zwei Zeilenangaben sind verrutscht.** Befund 2 des Plans nennt die Arbeitskopie der
Belegungsansicht bei `anwendung.rs:2159`; sie steht heute in Zeile 2174. Dieselbe Angabe steht in
`history/260811-0826-shaper-diagrammnachzug-…`. Beide sind durch den Zuwachs in `anwendung.rs`
gewandert, nicht durch eine Änderung an der Sache.

**3. Eine Datei außerhalb der Planschritte ist mitgeändert worden.**
`crates/krk-bench/src/messen.rs` hat in `ffb702c` 80 Zeilen bekommen; kein Schritt dieses Plans
nennt sie. Es ist keine unbemerkte Änderung: sie behebt
`shared/issues/260810-1925_*_eine-probe-schreibt-ins-echte-temporaerverzeichnis-…`, der jetzt
geschlossen ist, und die Commit-Nachricht schreibt sie aus. `Messplanwaechter::neu` ist dabei
entfernt worden, und `plan_schreiben` ist die einzige Stelle im Baum geblieben, die
`std::env::temp_dir` für den Messplan nennt. **Das macht einen Absatz in `CLAUDE.md` falsch**,
siehe unten.

---

## Der Zustand von Plan und Spec

**Der Plan gehört auf `_c_`, und er steht jetzt dort.** Drei Schritte tragen `[DONE]`, der
vierte trägt `[GESTRICHEN]`. Kein Schritt steht offen, also ist der Plan abgearbeitet, und die
Regel zur Zustandsführung verlangt `**Status:** Complete` und den Marker `_c_`. Ein gestrichener
Schritt ist kein offener: er ist eine Nutzerentscheidung mit einem benannten Preis, und der Preis
steht im Schritt.

**Der Spec gehört auf `_o_`, und der Marker ist unverändert geblieben.** Der Nutzer hat es
angewiesen, und die Anweisung deckt sich mit der Sache. Ein Spec ist abgearbeitet, wenn seine
Abnahmekriterien eingelöst sind; hier ist keines der 41 eingelöst, weil der einzige Weg dorthin
gestrichen ist. Ein `_c_` behauptete, die Runde sei abgenommen, und das ist genau die Aussage,
die diese Runde nicht treffen kann. Die Empfehlung des Abgleichs lautet deshalb: **`_o_` lassen,
bis der Abnahmelauf gefahren ist.** Solange steht "gebaut" über dieser Runde und "abgenommen"
nicht.

**Die Abnahmeanleitung `planning/260811-1130_o_…` bleibt ebenfalls auf `_o_`.** Sie ist kein
Rest, sondern die Grundlage für die Frage, wie sich ein solcher Lauf automatisieren lässt. Block 1
darin ist schon heute ohne Oberfläche prüfbar (`make menue`).

---

## Die Offene-Fragen-Fläche über alle Speicher

Neunzehn offene Entscheidungen, sechs offene Defekte, null beantwortete Entscheidungen.
Der Dateibestand ist verbindlich; die beiden `find`-Kommandos aus `CLAUDE.md` sammeln ihn ein.

**Von dieser Runde berührt sind drei.**

- `circles/260809-2040-…/decisions/260811-1230_o_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`
  ist in dieser Runde entstanden und bleibt offen. Sie ist die Wurzelfrage hinter drei
  geschlossenen Rangbefunden: kein Prüflauf liest eine Zahl in einem Kommentar, also fängt nichts
  den nächsten Fehler dieser Art. Die Empfehlung im Datensatz lautet, die Zahl wegzulassen und die
  Quelle zu verlinken, sobald jemand die Dateien ohnehin anfasst.
- `circles/260802-0842-…/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`
  ist durch diese Runde **gewichtiger** geworden, nicht beantwortet. Sie ist der Grund, aus dem
  S4 gestrichen ist, und sie hält jetzt zwei Abnahmeläufe auf: die 110 Kriterien der Runde 2 und
  die 41 dieser Runde. Der Plan nennt sie im gestrichenen Schritt ausdrücklich als die Frage, an
  der eine Automatisierung hängt.
- `shared/issues/260811-1245_o_die-breite-des-vorschaufensters-faellt-beim-navigieren-…` ist am
  260811-1240 vom Nutzer gemeldet worden und liegt außerhalb dieser Runde. Der vorgesehene Circle
  `260811-1304-statusleiste-mit-bereichsschaltern` führt eine eigene Frage dazu, ob er ihn mit
  behebt.

**Gegenstandslos geworden ist keine.** Geprüft: keine der neunzehn offenen Fragen wird durch den
Bau dieser Runde beantwortet oder hinfällig.

**Zwei vorgesehene Circles sind am 260811 dazugekommen** (`caf6375`):
`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` mit vier offenen Fragen und
`260811-1304-statusleiste-mit-bereichsschaltern` mit sieben. Sie sind Aktivierungsfragen ihrer
Circles und binden diese Runde nicht.

---

## `CLAUDE.md` gegen den heutigen Stand

Sechs Abweichungen, gelesen am 260811-1403. **Nichts davon ist geändert**; die Revision ist ein
eigener Schritt.

**1. Zeile 96 beschreibt einen behobenen Zustand als bestehend.** Der Absatz "Ein `make check`
löscht den Messplan eines gleichzeitig laufenden Messlaufs" nennt `Messplanwaechter::neu`
(`krk-bench/src/messen.rs`). Die Funktion gibt es nicht mehr: `ffb702c` hat sie entfernt, jede
Probe geht seither über `in_verzeichnis` mit einem `Wegwerfordner`, und `plan_schreiben` ist die
einzige Stelle im Baum, die `std::env::temp_dir` für den Messplan nennt. Der zitierte Defekt
`issues/260810-1925_*_…` ist geschlossen. Der erste Satz des Absatzes bleibt richtig für zwei
gleichzeitige **Messläufe**; der zweite Greifer, `cargo test`, ist weg. **Das ist die
folgenreichste der sechs**, weil sie einen Leser vor etwas warnt, was nicht mehr eintritt.

**2. Zeile 146 nennt die Tastenbelegungs-Ausgabe als vorgesehenen, nicht gefahrenen Circle.**
Sie ist gefahren und schließt mit dieser Sitzung. Und es sind nicht mehr zwei vorgesehene
Circles, sondern drei: der Web-Betrachter, die vier Tastenbefehle und die Statusleiste. Derselbe
Satz sagt "Außerhalb der beiden gefahrenen Runden" — es sind drei.

**3. Zeile 128 bis 130 zählt die Entscheidungsspeicher auf und lässt diesen Circle aus.**
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/` führt acht Datensätze,
davon sieben umgesetzte und einen offenen, und keiner steht in der Aufstellung. Zeile 132 sagt
"die Runde 2 schließt mit dieser Sitzung" und meint eine Sitzung, die seit dem 260810 vorbei ist.

**4. Zeile 11 setzt die Runde 2 als den Circle, der ohne Nennung gilt.** Für Pfade der Form
`decisions/…` und `issues/…` in `CLAUDE.md` ist das seit dieser Runde mehrdeutig, weil ein dritter
Circle mit denselben Unterordnern danebensteht.

**5. Zeile 23 datiert den Projektstand auf 260810-1930.** Die Ausgabe der Tastenbelegung kommt
darin nicht vor, und der Absatz zählt auf, was die Anwendung trägt.

**6. Zeile 100 nennt zwei Stellen, die ein neues Kommando braucht, und eine dritte fehlt jetzt
daneben.** Der Absatz ist ausdrücklich unvollständig und sagt es selbst. Neu seit dieser Runde:
ein achter `Wirkungsbereich` braucht eine Zeile in `Wirkungsbereich::beschriftung`
(`krk-core/src/tasten/belegung.rs:269`), und der Übersetzer hält sie an.

**Zwei Gegenstände wären eine Ergänzung wert, nicht eine Berichtigung.**

- **Eine Zusage nach dem Mechanismus für Transparenz, Zustimmung und Kontrolle gilt je Paar aus
  Programm und Dienst, nicht je Vorgang.** KRK zeigt den Downloads-Ordner seit Runde 1 an und
  löst die Rückfrage damit beim Anzeigen aus, lange vor dem ersten Schreiben. Ein Text in
  `Info.plist`, der eine spätere Handlung nicht nennt, beschafft Zustimmung für etwas, was er
  verschweigt. Am Gerät belegt in `issues/260811-1000_c_…`.
- **`belegungsmodell::bereich` und `belegungsausgabe::wirkung` stellen zwei verschiedene Fragen.**
  Die erste fragt über `Kommando::aus_kennung` und sieht `gehalten_von` nicht, die zweite über
  `Funktion::kommando` und sieht es. Eine von Hand geschriebene `keymap.toml` lässt sie
  auseinanderlaufen. Der Datensatz `issues/260811-0955_d_…` führt beide Behebungswege mit ihrem
  Preis.

---

## Neu abgelegte Datensätze

Keine. Was der Abgleich gefunden hat, ist entweder in einem bestehenden Datensatz erfasst oder
eine Beschreibungsdrift, die in diesen Bericht und in das Reconciliation Log des Plans gehört.

---

## Der Kohärenz-Spruch

`bounded-closure-proposed`. Die Begründung steht im Abschnitt `## Coherence` der Sitzungsdatei
`history/260811-0107-orchestrator-session.md` und wird hier nicht verdoppelt.
