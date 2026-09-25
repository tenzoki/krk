Vierzehn geschlossene Datensaetze zeigen auf Zeilen, die ihre eigene Sitzung verschoben hat

---

Die 52 Defektdatensaetze, die die Sitzung 260810-0845 geschlossen hat, belegen ihre
Behebung mit `Pfad:Zeile`. Vierzehn von ihnen zeigen inzwischen auf die falsche
Zeile, und in dreizehn Faellen hat ein **spaeterer Commit derselben Sitzung** sie
verschoben. Die Sache ist an jeder dieser Stellen belegt; nur der Fingerzeig geht
ins Leere. Dazu kommen sechs Angaben, die nicht bloss gewandert, sondern falsch
sind.

---

**Schwere:** Niedrig
**Gefunden:** `reconciler`, Abschluss-Abgleich der Sitzung 260810-0845
**Domain:** code
**Betroffen:** vierzehn Dateien unter `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/`, dazu neun Datensaetze in drei Entscheidungsspeichern
**Zusammenhang:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` (die verwandte, aber andere Sorte: dort wandert der **Marker** im Dateinamen, hier die **Zeilennummer** im Zielcode)

## Was kein Befund ist

**Keine der 52 Behebungen ist erfunden.** Das ist einzeln geprueft: 45 Datensaetze
sind vollstaendig gedeckt, 7 tragen eine Abweichung, 0 sind ohne Deckung. Dieser
Datensatz handelt allein von der Buchhaltung der Belege, nicht von den
Behebungen.

## Die vierzehn Datensaetze und ihre Ursache

Nach Ursache gruppiert, weil die Ursache den Umfang der Behebung bestimmt.

**Verschoben durch das Stapelbudget (`0140df7`), den letzten Commit der Sitzung:**

```text
260810-1314 (Verweis)  editor.rs:3060  -> :3324    let _ = unsafe { text.layoutManager() };
260810-1314 (Verweis)  editor.rs:4629  -> :5039    die Probe
260810-1314 (Verweis)  editor.rs:58-76 -> :51-77   der Modulkopf
260810-1017            sys.rs, acht Angaben, alle um weitere fuenf Zeilen tiefer
```

**Verschoben durch die Oeffnungsherkunft (`8807844`):**

```text
260810-1028  anwendung.rs:2035 -> :1971   die Blattpruefung in kommando_ausfuehren
260810-1102  anwendung.rs:2035 -> :1971   dieselbe Stelle
260810-1102  anwendung.rs:1983 -> :1919   die Blattpruefung in eingabe_ausfuehren
```

Beide Datensaetze sind gegen den Stand `f28497b` geschrieben, also gegen einen
Stand, den ihre eigene Behebung schon verschoben hatte.

**Verschoben durch den Dateikopf der Belegung (`c0b96a6`, `bf0fe18`):**

```text
260810-1217  default-keymap.toml:504,513,521,530 -> :507,516,524,533
260810-1217  der Absatz selbst              :479 -> :482
260810-0011  default-keymap.toml:484-499 -> :487-502  und :625-640 -> :628-644
```

**Verschoben durch S16 und spaetere Schritte, aelter als diese Sitzung:**

```text
260808-1413 (vier Platzhalter)  aufteilung.rs:404-408 -> :406-409
                                aufteilung.rs:434-437 -> :435-438
                                aufteilung.rs:478-486 -> :473-487
```

**Eine erhobene Zahl reproduziert nicht mehr:**

```text
260809-1655  "24 Zeilen in 9 der 22 Dateien"  -> heute 27 Zeilen in 10 von 22
```

Die Zahl ist im Datensatz datiert und steht ausdruecklich **nicht** im Modulkopf
von `crates/krk-ui/src/appkit/mod.rs`; die Abwanderung belegt gerade den Grund,
aus dem sie dort weggefallen ist.

## Sechs Angaben, die nicht gewandert, sondern falsch sind

Diese sechs sind schaerfer als eine verrutschte Zeile, weil kein Leser sie am
Nachbarort findet.

1. **`260810-0303`: „alle sieben Aufrufstellen nennen ihn" — es sind acht**, und
   sie waren es schon am behebenden Commit `bb43315`. Ein Zaehlfehler, keine
   Abwanderung. Berichtigt im Datensatz selbst am 260810-1404.
2. **`260810-0303`: `Verlauf` traegt drei Werte, nicht zwei**, und das
   CRLF-Richten geht als `TraegtNurDiese` und nicht als `Faellt`. Ueberholt durch
   die eigene Sitzung (`260810-1044`). Ebenfalls dort berichtigt.
3. **`260810-0303`: der `Umkehrpunkt` traegt nicht den ganzen Stand**, sondern
   den geaenderten Bereich. Ueberholt durch `260810-1314`. Ebenfalls dort
   berichtigt.
4. **`260810-0418`: `AnwendungsIvars::editor_aus_sitzung` gibt es nicht mehr.**
   `grep -rn editor_aus_sitzung crates/` findet nichts; die Herkunft wohnt jetzt
   als `EditorIvars::herkunft` in `crates/krk-ui/src/appkit/editor.rs:1264`.
   Nachwirkung von `8807844`, also des Restbefunds, den der Datensatz selbst
   abgelegt hat.
5. **`260810-0419`: „Beides steht im Doc-Kommentar" — nur der erste Grund steht
   dort** (`crates/krk-ui/src/appkit/editor.rs:3397-3399`); der zweite steht im
   Doc-Kommentar von `umkehrung_anmelden` (`:1989-1991`).
6. **`260810-0748`: drei Proben lesen ihre Namen aus `EINSTELLUNGEN` — es sind
   zwei.** `der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl`
   arbeitet mit den Zeichenketten `"writingToolsBehavior"` und
   `"allowsWritingToolsAffordance"` (`editor.rs:5267`, `:5273`); `EINSTELLUNGEN`
   kommt dort nur in Fehlertexten vor.

Dazu zwei kleinere: `260810-0512` nennt die Probe
`keine_unbekannte_automatik_steht_an_der_textflaeche`, im Baum heisst sie
`keine_unbekannte_einstellung_steht_an_der_textflaeche` (`editor.rs:4732`); und
`260810-1241` verweist auf `260810-1314` als das, was offen bleibt, obwohl der
Datensatz inzwischen `_c_` traegt.

## Derselbe Befund in den Entscheidungsspeichern

Neun Belegzeilen in Entscheidungsdatensaetzen zeigen ebenfalls nicht mehr auf das
Genannte, fuenf davon durch diese Sitzung verschoben. Der Pfad stimmt jeweils,
die Zeile nicht:

```text
260807-2147_i_fuer-welche-sprachen…        hervorhebung.rs:315 -> :360
260807-2147_i_fuer-welche-sprachen…        Cargo.toml:103-161, Block reicht ueber :177
260807-2147_i_welche-dateien-oeffnet…      datei.rs:136,153,167 -> :153,170,184
260807-2147_i_welche-dateien-oeffnet…      editormodell.rs:456, anwendung.rs:3333 — beides
                                           keine Aufrufstelle mehr
260807-2147_i_wie-weit-reicht-die-suche…   editor.rs:414
shared/260802-0842_i_editor-formatansicht… hervorhebung.rs:200-208, editormodell.rs:239,
                                           editor.rs:1646/1681/1709 — alle fuenf
260810-0822_i_wie-die-formatansicht…       editor.rs:1099  (am 260810-1404 nachgetragen)
```

Vier weitere sind aelter als diese Sitzung, gegen `38a02b2` geprueft:
`260803-1755_i_` (`messen.rs:377`), `260803-2007_i_` (`menue.rs:217`),
`260802-0842_i_f-tasten…` (`default-keymap.toml:101-123`),
`260804-1122_i_wandern-die-bereichsbreiten…` (`default-keymap.toml:383-391`).

## Fehlszenario

Kein Fehlverhalten zur Laufzeit. Was es kostet, ist die naechste Nachprüfung. Wer
`260810-1102` liest und `anwendung.rs:2035` aufschlaegt, findet dort heute nicht
die Blattpruefung, sondern etwas anderes, und muss die Stelle suchen — oder
schliesst, die Behebung sei nicht gelandet, und baut sie ein zweites Mal. Genau
diesen Weg hat der Befund `260810-1102` selbst schon einmal genommen: er ist als
„der Befund haelt nicht" geschlossen worden, weil sein Verfasser nur eine von
zwei Sperren gefunden hatte.

## Vorgeschlagene Behebung

**Die Zeilennummer weglassen und das Stueck benennen.** Das ist im Projekt keine
neue Regel, sondern die, die diese Sitzung an drei anderen Stellen schon
angewandt hat: `CLAUDE.md` hat seine Zahl der Fallunterscheidungen gestrichen
(„der Uebersetzer nennt die Stellen ohnehin genauer als jede Aufzaehlung"), der
Modulkopf von `appkit/mod.rs` seine Zaehlung der Pfeile, und der Kopf des Plans
seine Zahl der Abnahmekriterien. Ein Beleg der Form
`crates/krk-ui/src/appkit/anwendung.rs` (`kommando_ausfuehren`, Blattpruefung)
ueberlebt jeden Commit, der die Datei nur verschiebt; eine Zeilennummer nicht.

Zwei Schritte, in dieser Reihenfolge:

1. Die sechs sachlich falschen Angaben berichtigen. Drei davon sind am
   260810-1404 bereits im Datensatz `260810-0303` erledigt; bleiben `260810-0418`,
   `260810-0419`, `260810-0748` und die zwei kleineren.
2. Die verrutschten Zeilenangaben in den vierzehn Defekt- und neun
   Entscheidungsdatensaetzen auf die Form „Pfad plus Name des Stueckes" ziehen.
   Das ist mechanisch und beruehrt keinen Programmteil.

**Nicht vorgeschlagen: eine Probe, die Belegzeilen gegen den Baum prueft.** Sie
waere ein Mechanismus fuer ein Problem, das die Schreibform loest, und sie muesste
Prosa lesen.

## Zustaendigkeit

`ontocoder`. Es sind Zeilen in Datensaetzen, kein Programmteil.

---
Resolved: teils berichtigt, im Rest bewusst nicht weiterverfolgt — der Orchestrator am 260810-1520.

Die drei sachlich falschen Angaben, die der Abgleich selbst berichtigen konnte,
sind berichtigt. Die vierzehn abgewanderten Zeilenverweise bleiben stehen, und
das ist eine Entscheidung und kein Versäumnis: sie stehen in Datensätzen über
**geschlossene** Arbeit, die niemand mehr aufruft, um an eine Zeile zu springen.
Sie einzeln nachzuziehen kostet Agentenzeit für Text, dessen Zweck erfüllt ist.

Der Befund hat trotzdem etwas hinterlassen, und das ist sein Wert: die Form
„das Stück benennen statt der Zahl" ist in dieser Sitzung an mehreren Stellen
angewandt worden, unter anderem in `CLAUDE.md`, das bewusst keine Zeilennummern
mehr führt. Damit greift der Befund für künftige Datensätze vorbeugend, statt
rückwirkend Text zu putzen.
