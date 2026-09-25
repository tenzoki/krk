Zwei Proben in `leseprofil.rs` schreiben die Gestalt der mitgelieferten Profile von vor fusion 11 fest

---

`resources/default-readers.toml` ist auf fusion 11 nachgezogen. Zwei Proben in
`crates/krk-core/tests/leseprofil.rs` halten die alte Gestalt als Erwartung fest und sind
seitdem rot; `make check` bricht an `cargo test` ab. Die Datei selbst ist in Ordnung: beide
Proben, die sie halten, sind grün.

---

**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** beim Nachziehen der fusion-Leseprofile auf Fassung 11; der Auftrag war auf
`resources/default-readers.toml` beschränkt und schloss Rust und Proben ausdrücklich aus.
**Betroffen:** `crates/krk-core/tests/leseprofil.rs`

## Was grün ist

`cargo test -p krk-core --lib ablage::leseprofile` — drei Proben, alle grün. Darunter die
zwei, die die Auslieferungsfassung halten: `die_auslieferungsfassung_nennt_jeden_bausteinnamen`
und `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`. Die geänderte Datei ist also
gültiges TOML, jeder Ausdruck darin übersetzt, jede Ortsangabe löst auf, und es sind weiter
dreizehn Profile.

## Was rot ist

`cargo test -p krk-core --test leseprofil` — 61 grün, 2 rot.

### 1. `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`

Sie bricht an der ersten ihrer vier Messungen ab, und dahinter stehen fünf Stellen:

**a) Der Prüfordner `runde` (Zeile 2969) liegt am falschen Ort.** Das Profil
„fusion-Werkbank: eine Runde" erkennt seit der Änderung über
`pfad = 'fusion-workbench/circles/[^/]+$'` und nicht mehr über das Kennzeichen
`^_._circle\.md$`. Das ist der Angelpunkt der ganzen Umstellung: fusion 11 benennt den
Datensatz einer Arbeitseinheit nach seinem Verzeichnis, ein Kennzeichen auf den alten Namen
sähe eine heutige Runde nicht mehr. `Pruefordner::neu` legt seinen Ordner aber unter
`~/Library/Caches/krk-messplatz` an, und kein Pfad dorthin trägt `fusion-workbench/circles/`.
Der Prüfordner gehört eine Ebene tiefer, unter `<wurzel>/fusion-workbench/circles/<runde>/`.

**b) Der Datensatz im Prüfordner trägt keine Kopfzeile `**Status:**`.** Er heißt heute
`_t_circle.md` und trägt seinen Zustand im Namen. Das neue `datei`-Muster nimmt beide
Namensformen an, die Zeile „Zustand" stünde an diesem Prüfordner aber auf ihrem Platzhalter.
Wer die Zeile messen will, gibt dem Datensatz die Kopfzeile — oder legt einen zweiten
Prüfordner in der heutigen Form daneben und misst beide.

**c) Die erwartete Beschriftungsliste des Rundenprofils (Zeile 3937 ff.) stimmt nicht mehr.**
Vier Ja/Nein-Zeilen sind zu einer Feldzeile geworden:

```
alt   Vorgesehen, Aktiv, Geschlossen, Abgelegt, Directive, Spec, Plan, Entscheidungen, Die jüngsten zehn Verläufe
neu   Zustand, Directive, Spec, Plan, Entscheidungen, Die jüngsten zehn Verläufe
```

Damit verschieben sich zwei Zugriffe über die Stellung: `rundenwerte[4]` (Directive) wird
`rundenwerte[1]`, `rundenwerte[8]` (die zehn Verläufe) wird `rundenwerte[5]`.

**d) Der Haushalt des Rundenprofils ist `(4, 12)` und nicht mehr `(4, 11)`.** Die Leseläufe
bleiben vier: die Erkennung über `pfad` kostet nichts, gelesen werden der erkannte Ordner und
`planning`, `decisions`, `history`. Die Öffnungen steigen um eine, weil „Zustand" und
„Directive" dieselbe Datei nennen und sie trotzdem zweimal öffnen. Die Schranke `<= 11` in der
Zusicherung darunter ist mitzuziehen.

**e) Die Werkbankwurzel hat zwei Zeilen weniger.** Die erwartete Beschriftungsliste
(Zeile 3988 ff.) verliert „Aktive Runde" und „Sitzung", die ausgeschriebene Werteliste
(Zeile 4014 ff.) verliert `Wert::Text("circles/260823-2208-vorschau")` und
`Wert::Text("Schritt 12, die Zaehlproben")`, und der Haushalt fällt von `(4, 5)` auf `(4, 3)`:
die zwei gestrichenen Zeilen waren Feldzeilen und öffneten je eine Datei. Dieselben zwei
Zeilen fallen im Profil „Projektwurzel mit fusion-Werkbank", also noch einmal an der vierten
Messung derselben Probe (Beschriftungsliste, Werteliste, Haushalt).

### 2. `ohne_orchestrator_live_zeigt_allein_die_sitzungszeile_ihren_platzhalter`

Diese Probe hat ihren Gegenstand verloren. Sie benennt `orchestrator-live.md` um und misst,
dass allein die Zeile „Sitzung" ihren Platzhalter zeigt. Die Zeile gibt es nicht mehr: fusion
hat die Datei mit Fassung 11 abgeschafft, nichts schreibt sie noch, und eine Zeile, die von nun
an immer ihren Platzhalter zeigte, ist keine Auskunft. Dasselbe gilt für die Zeile „Aktive
Runde" und ihren Zeiger `.active-circle`, den das Umstellen der Werkbank löscht.

Die Mechanik, um die es dem Abnahmekriterium C5.8 der Runde 16 geht — eine Feldzeile, deren
Datei fehlt, zeigt ihren Platzhalter, und die übrigen Zeilen stimmen weiter —, ist unverändert
richtig und an einem von Hand gebauten Profilsatz auch weiter belegt
(`das_feld_zieht_die_erste_fanggruppe_des_ersten_treffers`). Was fehlt, ist ein Träger in der
**Auslieferungsfassung**. Welcher es sein soll, ist eine Frage und keine Ableitung: der
Wortlaut des Kriteriums nennt `orchestrator-live.md` namentlich, und es steht dem Coder nicht
zu, ein Abnahmekriterium umzuschreiben.

## Was der Haushalt heute sagt

Nachgerechnet an der geänderten Datei, je Profil:

| Profil | Leseläufe | Öffnungen |
|---|---|---|
| fusion-Werkbank: die Wurzel | 4 von 12 | 3 von 24 |
| fusion-Werkbank: alle Runden | 2 von 12 | 0 |
| fusion-Werkbank: der gemeinsame Speicher | 10 von 12 | 0 |
| fusion-Werkbank: eine Runde | 4 von 12 | 12 von 24 |
| Projektwurzel mit fusion-Werkbank | 5 von 12 | 3 von 24 |

## Eine stille Schwächung nebenbei

Die Zusicherung mit dem `step_by(2)` (Zeile 4088) läuft weiter grün, prüft aber einen Speicher
weniger. Sie setzt voraus, dass in jedem der zehn Unterspeicher die Zählungszeile vor der
Datumszeile steht. `history` und `investigations` tragen seit der Änderung keine Datumszeile
mehr, und dadurch rutscht die Zählungszeile von `investigations` auf eine ungerade Stellung und
wird übersprungen. Die Probe bleibt grün und deckt neun der zehn Zählungen statt zehn. Der
Doc-Kommentar darüber (Zeile 3912) behauptet die Voraussetzung weiter und ist damit falsch.

---
Resolved: 4815f0a — beide Proben sind auf die Gestalt der Fassung 11 nachgezogen. Der
Pruefordner liegt unter `<wurzel>/fusion-workbench/circles/<runde>/`, sein Datensatz traegt
zusaetzlich die Kopfzeile `**Status:**`, und drei Beschriftungslisten samt der Zugriffe ueber
ihre Stellung sind mitgezogen. `ohne_orchestrator_live_…` heisst jetzt
`ohne_den_rundendatensatz_zeigen_allein_seine_zwei_zeilen_ihren_platzhalter` und haengt am
Rundendatensatz; der Nutzer hat den Umzug am 260913 entschieden, und der Doc-Kopf schreibt
aus, worum es C5.8 geht und warum der Traeger erst seit dieser Umstellung moeglich ist. Die
stille Schwaechung ist behoben, indem die Zusicherung ueber den Baustein greift statt ueber
die Stellung: die Menge der Prueflinge kann damit nicht mehr lautlos schrumpfen.

**Eine Angabe dieses Datensatzes war falsch.** Punkt (a) nennt `~/Library/Caches/krk-messplatz`
als Ort von `Pruefordner::neu`; er nimmt `std::env::temp_dir()`, und `krk-messplatz` ist der
Messplatz von `krk-bench`. Der Schluss des Punktes haelt trotzdem, denn auch kein Pfad unter
dem Temporaerverzeichnis traegt `fusion-workbench/circles/`.

**Der Bruch von C6.7 bleibt offen und ist nicht Teil dieser Behebung.** Die Schranke `<= 11`
ist absichtlich **nicht** mitgezogen worden; die Probe haelt die exakte Zwoelf, damit jede
weitere Oeffnung rot wird. Behoben wird die Ursache:
`260913-0851_*_merkt-sich-der-feldbaustein-seine-dateioeffnungen-wie-der-leselauf-seine-lesungen.md`.
`make check` mit Exit 0.
