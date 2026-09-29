# Durchsicht: Werkseinstellungen, Suchblatt, Tabulatoren, EPERM, Leseprofile fusion 12

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `37e86a8..52be37a`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/shared/checkouts/6c11b1f2.md`, `260912-0441_*_die-spalte-wer-schreibt-der-howto-tabelle-nennt-fuer-drei-dateien-das-gegenteil-dessen-was-krk-tut.md`, `260929-1033-klaerung-vorschau-uebernimmt-profilstand.md`, `260929-1034-klaerung-ortsweg-mit-festem-ziel.md`, `crates/krk-core/tests/werkszustand.rs`
**Review domain:** both

`crates/krk-core/tests/werkszustand.rs` ist nicht gelesen, aber gefahren: `cargo test -p krk-core --test werkszustand`, 14 von 14 grün. `make check` ist nicht gefahren.

## Summary

Der Werkseinstellungen-Befehl hält die Zusagen, die sich am Baum prüfen lassen: Alles-oder-nichts bis Stufe 3, ein Durchgang ohne Verschachtelung, kein Weg zum Notizordner, `notizordner` über dieselbe Ersetzung wie „Ort wählen…“, alle Pflichtstellen des Kommandos. Kein Befund über Low. Drei neue Defekte, zwei Nachträge an offenen Datensätzen.

## Totals

| Schwere | Anzahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 3 |

## Befunde nach Thema

### 1. Werkseinstellungen (ebf94a4, a9a8abc, c44c71f, 8b60668, e516598, e4bf819)

Geprüft und ohne Befund:

- **Alles oder nichts.** `werkszustand::zuruecksetzen` (`crates/krk-core/src/ablage/werkszustand.rs`): Lage und neue Fassungen unter der Sperre, dann `fs::hard_link` je vorhandener Datei, dann `atomar::vorbereiten` je neuer Fassung; Scheitern in Stufe 2 oder 3 ruft `rueckbau`, das allein die in diesem Lauf angelegten Sicherungsnamen entfernt, und `drop(vorbereitet)` räumt die Nachbardateien. Stufe 4 wird nicht zurückgebaut und meldet je Datei (`Dateiausgang::vollzug`), wie Entscheidung 4 des Plans es festlegt. `link(2)` überschreibt nie; `AlreadyExists` zählt bis `HOECHSTE_NUMMER`.
- **Notizordner.** `einstellungen::auslieferung_mit_notizordner` setzt den Quelltext des alten Werts (`&alt[alte_stelle.bereich]`) über `wert_einsetzen` in `AUSLIEFERUNGSTEXT` ein; `pruefen` vergleicht jetzt gegen `wert_aus_quelltext`, also auch für `notizordner = 5`. Fehlende Datei oder fehlender Schlüssel ergeben `AUSLIEFERUNGSTEXT` unverändert. Beschädigt (`ortsstelle` mit `deny_unknown_fields`, `ist_einzelwert`) bricht in `lage` ab, vor der Rückfrage (`Anwendungsdelegierter::werkseinstellungen`) und noch einmal unter der Sperre. `notizordner_schreiben` ruft dieselben drei Funktionen, `die_ersetzung_des_notizordners_steht_an_einer_stelle` hält es.
- **Keine Datei im Notizordner.** `werkszustand.rs` fasst allein `zugang.pfad(Datei)` an. `werkseinstellungen_vollziehen` ruft weder `heimgriff::ersetzen`, `ort_wechseln`, `sitzung_vormerken` noch `heimordner_gewechselt`; das Feld `notizordner` in `AnwendungsIvars::einstellungen` liest im Betrieb niemand (einzige Leser: `anwendung.rs` Zeilen 2265 und 2813, `terminal`). `profile_uebernehmen` reicht `heimgriff::lesen` nur lesend durch.
- **Sperre.** Die Vorabfrage nimmt keine Sperre (`werkszustand::lage` über `Ablage::pfad`). Der Vollzug fährt einen `unter_der_sperre` mit `zuruecksetzen`, `belegung::laden`, `nutzerdateien_lesen`, `neuerungen::erheben`; `belegung::laden` statt `fuer_den_betrieb`, also keine zweite Ablage. Der Rückruf des Blattes ruft zuerst `blatt_geschlossen`, der Vollzug läuft außerhalb jedes anderen Durchgangs.
- **Reihenfolge des Neu-Einlesens.** Einstellungen, `belegung_uebernehmen`, Profile in ivars und `profile_uebernehmen`, Urteile, Neuerungen, zuletzt die Statuszeile; entspricht Schritt 8 des Plans. `neuerungen::erheben` statt `neuerungen_erheben`, `reported.toml` bleibt unberührt.
- **Blattsperre und Pflichtstellen.** `Kommando::KENNUNGEN` (Länge 100), `wirkungsbereich` → `Ueberall`, `bereich_des_kommandos` → `Anwendung`, eigener Zweig `Kommando::Werkseinstellungen => self.werkseinstellungen()` (`anwendung.rs` Zeile 4461), Eintrag in `zweigproben::BEFEHLE`, `default-keymap.toml` mit `tasten = []` und nachgezählter Kopfzeile (107 Funktionen, 107 Kombinationen). Nicht in `immer_erreichbar` oder `waehrend_blatt_erlaubt`; `der_werkseinstellungsbefehl_kommt_bei_stehendem_blatt_nicht_durch` hält es.
- **secrets.txt.** Kein Pfad in diesem Bereich berührt `ist_geheimnisdatei`, `haelt_geheimnisse` oder den Heimordner-Griff.
- **Rückfrage.** `loeschbestaetigung::zeigen` mit `laut = true`; Return und Esc brechen ab, Cmd+Return bestätigt (C1.6). Die Opt+Return-Änderung aus `ab8d7db` erreicht dieses Blatt nicht: es hat kein Textfeld und damit keinen `Eingabewaechter`.

**Low, data: `260929-1141_*_die-auslieferungsfassung-von-settings-toml-sagt-das-zuruecksetzen-nenne-bei-einem-verweis-die-zeile-zum-eintragen.md`.** `resources/default-settings.toml`, Kopf: „Ist die Datei ein symbolischer Verweis, schreibt KRK sie auf keinem der zwei Wege und nennt die Zeile zum Eintragen von Hand.“ `Werkshindernis::meldung`, Zweig `Verweis`, nennt keine Zeile. `README.md` sagt es richtig. Der Kommentar reist in jede zurückgesetzte `settings.toml`.

Kein Defekt, aber vermerkt: `Werkshindernis::Einstellungen` hängt „. Nichts ist zurückgesetzt.“ an `Schreibhindernis::meldung`; für `Schreibhindernis::Intern` ergibt das „… das Ergebnis hätte mehr geändert als den Notizordner …“, ein Satz, der zum Zurücksetzen nicht passt. Erreichbar nur bei einem internen Prüffehler.

### 2. Suchblatt (ab8d7db)

Geprüft und ohne Befund: `suche::schaltflaechen` legt Weitersuchen auf `Taste::Eingabe`, Ersetzen auf Cmd+Return, Alle ersetzen auf Opt+Return; `die_eingabetaste_sucht_und_ersetzt_nicht` hält, dass bloßes Return nie ersetzt. `Editorbereich::suchblatt_beantworten` ruft allein `weitersuchen`, `treffer_ersetzen`, `alle_treffer_ersetzen`, `suche_beginnen`; das Sammelersetzen bleibt ein Rückgängig-Schritt, weil es denselben Weg wie `ctrl+cmd+r` geht. Der neue Wächterzweig greift nur, wo `wahlstelle` eine Stelle liefert: im Suchblatt und im Konfliktblatt, sonst nirgends (`die_tafel_der_waehlenden_stelle`).

**Low, code: `260929-1141_*_der-kopf-des-konfliktblatts-nennt-opt-return-im-namensfeld-ungemessen-seit-ab8d7db-leitet-der-waechter-es-weiter.md`.** Der Modulkopf von `konflikt.rs` nennt Opt+Return im Namensfeld „am laufenden Bündel zu messen“. Seit `ab8d7db` ist es ein Codeweg, der „Umbenennen“ mit dem getippten Namen auslöst. Nachtrag `Also seen` an `260825-1130_*_ein-selbst-getippter-name-im-konfliktblatt-kann-einen-belegten-treffen-und-wird-ohne-rueckfrage-ueberschrieben.md`, weil dieser Defekt dadurch per Tastatur erreichbar ist.

Latent, kein Defekt heute: `Eingabewaechter::waehlen` fragt `bestaetigung_erlaubt` nicht. Heute trägt kein Blatt zugleich `bestaetigung_pruefen` (allein das PIN-Blatt) und eine Schaltfläche auf `EingabeMitWahl`. Ein künftiges Blatt mit beidem ließe Opt+Return an seiner Prüfung vorbei.

### 3. Tabulatoren (3c246a2)

Geprüft: `grund_legen` ersetzt den Absatzstil statt ihn zu entfernen, `einzugsmerkmal` baut auf `grundabsatz` auf, die Zeichen bleiben (Probe `ein_tabulator_laesst_mindestens_eine_spalte_zwischenraum`). Vorschau und Editor gehen beide über `zuruecksetzen`.

**Low, code: `260929-1141_*_quicknote-und-eine-leer-geoeffnete-editordatei-tabben-weiter-auf-die-28-punkt-stopps.md`.** Der Grundabsatz liegt nur auf Text, der beim Aufruf schon im Speicher steht. Die Quicknote ruft `textmerkmale::zuruecksetzen` nie (`quicknote.rs`, Aufbau um Zeile 281, nur `setFont`), und eine beim Öffnen leere Editordatei in der Rohansicht bekommt ihn für getippten Text nicht. **inference:** Getippter Text erbt die `typingAttributes`, die dort den Absatzstil des Systems tragen; am Bündel nicht gemessen.

### 4. EPERM-Meldung (a7a76b4)

Ohne Befund. `verzeichnis::leser::datenschutzsperre` antwortet allein bei `raw_os_error() == Some(1)`. Die zwei Rufer (`tabs.rs`, `lesemeldungen_einziehen`; `kommandos/pfadeingabe.rs`, `pruefen`) behalten für `None` ihren alten Satz wörtlich. `allein_eperm_meldet_die_datenschutzsperre` hält EACCES, ENOENT und einen Fehler ohne Nummer fern. Keine andere Meldung ist geändert.

### 5. Leseprofile fusion 12 (9ea57a3), Domäne data

Gegen `## fusion-workbench Layout` von fusion 12.0.1 gehalten:

- Speicherprofil: `analyses|checkouts|consultations|decisions|discussions|history|investigations|memos|plans|reviews`. Das sind die Speicher unter `shared/` ohne `issues` und `forum` (eigene Profile), und es enthält die sechs Speicher eines Arbeitspakets ohne `issues` (`plans`, `decisions`, `discussions`, `reviews`, `analyses`, `history`). Vollständig.
- Gemeinsamer Speicher: zehn Unterspeicher gelistet, `forum` und `discussions` bewusst nicht, begründet mit dem Lesehaushalt; zehn Leseläufe von zwölf, unverändert gegenüber vorher (`consult` → `consultations`, `planning` → `plans` sind Tausch, kein Zuwachs).
- `ein Arbeitspaket`: `pfad = 'fusion-workbench/work-packages/[^/]+$'`, Datensatz `[0-9]{6}-[0-9]{4}-.+\.md`, Felder `**Status:**`, `**Cross-references:**`, `**Filed by:**`, `## Directive` stehen in der Vorlage von fusion 12 (`## Work packages`). Ein Leselauf, eine Öffnung.
- `alle Arbeitspakete`: zwei Leseläufe, `*/issues` als Platzhalter wie vorher.
- `discussions` mit nur `_o_`/`_c_` entspricht `## Filename Patterns`.

Nicht neu, aber vom Commit nicht behoben: die Zeile „Projekt“ im Wurzelprofil liest `setup_pwd`, das `.fusion-setup` unter 12.0.1 nicht führt (`{"setup_at":…,"plugin_version":"12.0.1"}`). Offener Datensatz `260825-2044_*_die-zeile-projekt-der-werkbankprofile-haengt-an-einem-feld-das-fusion-nicht-mehr-schreibt.md`, Nachtrag `Also seen` gesetzt.

### 6. Doku-Commits (e71ccd9, e8b1e82, 52be37a, 79cbb95)

Gegen den Code gelesen: `README.md` `## Neuerungen an den eigenen Dateien übernehmen`, `HowTo.md` (Tabelle Zeilen 27 bis 31, Abschnitt ab Zeile 89, Zeilen 912 ff.), `CLAUDE.md` (zwei Absätze zu Leseprofilen und `settings.toml`), `resources/default-keymap.toml`, `resources/default-settings.toml`. Einziger Widerspruch zum Code ist der Befund unter Thema 1 in `resources/default-settings.toml`. Die in `CLAUDE.md` genannten Proben `die_leseprofile_haben_einen_leseweg_und_zwei_zeitpunkte`, `die_profile_haben_einen_schreiber_und_zwei_rufer` und `der_werkseinstellungsbefehl_fasst_den_notizordner_nicht_an` stehen im Baum.

## Cross-cutting observations

- **Zwei Kommentarstellen haben den neuen Codeweg nicht mitgezogen**, beide mit „gilt für beide“ oder „ungemessen“: `default-settings.toml` schreibt eine Eigenschaft von „Ort wählen…“ beiden Wegen zu, `konflikt.rs` hält einen jetzt gebauten Weg für offen. Beide Commits haben die Hauptdokumentation (`README.md`, `blaetter/mod.rs`) richtig nachgezogen und die Nebenstelle übersehen.
- **Ein Fix wirkt dort, wo der Aufrufer schon ist, nicht an jeder Fläche mit derselben Eigenschaft**: der Tabschritt erreicht die Quicknote nicht, weil sie `zuruecksetzen` nicht ruft. Dasselbe Muster wie die `ALLE`-Listen in `CLAUDE.md`: eine Regel an der Stelle statt an der Fläche.

## Recommended sequencing

Nichts davon hält eine Auslieferung auf. Der Kommentar in `default-settings.toml` sollte vor der nächsten Auslieferung berichtigt werden, weil er mit jedem Zurücksetzen in die Nutzerdatei geschrieben wird. Die übrigen zwei sind Aufräumarbeit. Offen bleibt der Abnahmelauf am Bündel (C1 bis C3), den kein Agent fahren kann.
