Die Dateiliste von Schritt 12 nennt zwei nötige Dateien nicht

---

Die Umsetzung von Schritt 12 hat zwei Dateien anfassen müssen, die seine
Dateiliste nicht führt:

- `crates/krk-core/src/tasten/belegung.rs` — die Aufzählung `Kommando` wächst um
  die elf Funktionen aus C1 und C7, die Schritt 12 ausführbar macht.
- `crates/krk-ui/src/appkit/ereignisse.rs` — der Ereignisabgriff nahm bis
  Schritt 11 die eine Datenquelle des einen Dateifensters entgegen. Mit zwei
  Dateifenstern und Kommandos, die keinem von beiden gehören, muss er eine
  Senke entgegennehmen statt eines Ziels.

---

Beide sind kein Versäumnis der Durchsicht vom 260803-2007, sondern der Rest, den
der Plan im Kopf von `## Implementierungsschritte` selbst erwartet: "ein Nachtrag
aus der Umsetzung ist kein Versäumnis der Durchsicht, sondern ihr erwarteter
Rest". Sie sind trotzdem festzuhalten, weil dieselbe Form schon dreimal
aufgetreten ist und der Plan sie deshalb als Muster führt.

**Die erste war vorhersagbar, und zwar aus dem Plan selbst.** Der Absatz zu
Schritt 11 sagt wörtlich: "S12 verlangt mit den Tabbefehlen aus C1 bereits
Kommandos, die die verdrahtete Tabelle nicht kennt." Die Datei, in der diese
Kommandos stehen, ist seit Schritt 11 `belegung.rs`; die Dateiliste von S13 nennt
sie aus demselben Grund ausdrücklich, die von S12 nicht.

**Die zweite ist die Form, die die Durchsicht als schwer zu sehen benennt:** die
Datei, in der ein Schritt einen vorhandenen Mechanismus **ablöst** statt einen
neuen danebenzustellen. `ereignisse.rs` behält seine Aufgabe und wechselt seinen
Gegenüber.

Die elf Kennungen, die `Kommando` dazubekommen hat, stehen alle seit S9
beziehungsweise S9b in `resources/default-keymap.toml`: `tab_neu`,
`tab_schliessen`, `tab_naechster`, `tab_voriger`, `fenster_wechseln`,
`leiste_umschalten`, `zweites_fenster_umschalten`, `vorschau_umschalten`,
`fenster_einblenden`, `bereich_verbreitern`, `bereich_verschmaelern`. Die
Belegungsdatei ist unverändert; die Prüfung
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` deckt die
Zuordnung maschinell ab.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 11, Schritt 12, Schritt 13, Kopf von `## Implementierungsschritte`),
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`

---
Resolved: Die Dateiliste von S12 im Plan nennt jetzt `crates/krk-core/src/tasten/belegung.rs` (erweitert, mit allen elf Kennungen) und `crates/krk-ui/src/appkit/ereignisse.rs` (erweitert: Senke statt Ziel). Gegen die Wiederkehr steht der neue Abschnitt "Was eine Dateiliste zusagt, und was nicht" samt der Kommando-Regel im Kopf von `## Implementierungsschritte`: nennt ein Abnahmekriterium einen Tastendruck am laufenden Bündel, führt die Dateiliste `belegung.rs`. Die Regel hätte genau diesen Fall gefangen. Nachgezogen am 260804-2318 vom `planner`.
