# Schlussdurchsicht: Menü „Navigation“, `quicknote.txt`, Ende von Notizen und Terminen, `minutes/` und „Protokoll anlegen“

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `02c2f859..2cae597a`
**Not-opened:** `crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/heimordner.rs`, `crates/krk-core/tests/sprache.rs`, `crates/krk-core/tests/text.rs`, `crates/krk-ui/src/appkit/eintragsansicht.rs`
**Domain:** code
**Work-item:** `261111-0809-menue-navigation-krkhome-drei-dateien-und-minutes`
**Gemessen an:** Spec `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`, Plan `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`

## Summary

Kein Befund spricht gegen die Auslieferung dieses Stands. Die Wege, die Bestand des Nutzers berühren (Übernahme von `scratch.txt`, Bereitstellung des Notizordners, `secrets.txt`, Namensregel in `minutes/`, eigene `keymap.toml`), sind am Quelltext gelesen und halten, was Spec und Plan zusagen. Gefunden sind zwei Stellen niedriger Schwere, beide Text und kein Verhalten.

`make check` ist am Stand `2cae597a` grün durchgelaufen (alle fünf Kommandos, Ausgang 0).

## Totals

- Critical: keiner
- High: keiner
- Medium: keiner
- Low: zwei

## Wie tief gelesen wurde

Nicht jede geöffnete Datei ist ganz gelesen. Die Tiefe je Datei:

- **Ganz gelesen (Betriebscode):** `heimordner/bereitstellen.rs`, `heimordner/protokoll.rs`, `quicknotesicherung.rs` (Prüfmodul nur die Probennamen).
- **Diff ganz gelesen:** `heimordner/mod.rs`, `heimordner/ort.rs`, `lib.rs`, `tasten/belegung.rs`, `eintraege/geheimnisse.rs`, `text/datei.rs`, `ablage/sitzung.rs`, `ablage/pfade.rs`, `ablage/mod.rs`, `tresorweg.rs`, `README.md`, `resources/default-keymap.toml`.
- **Diff teilweise gelesen:** `appkit/anwendung.rs` (Zeilen 1 bis 931 von 1164 des Diffs und der Anfang der Tafel dahinter; der Rest der Tafel `f2_entscheidet_je_stand_und_hindernis` nicht), `kommandos/operationen.rs` (der Betriebscode ganz, das Prüfmodul bis zur Tafel von `im_protokollordner`), `blaetter/namenseingabe.rs` (bis in den Rumpf von `blatt_bauen`; die Proben nicht), `ablage/merker.rs` (Kopf und der gefallene Schreiber).
- **Allein die geänderten Zeilen ohne Kommentare:** `appkit/editor.rs` (dazu ganz die Rümpfe der Quicknote-Sicherung, `oeffnungsweg`, `zelle_abbrechen`, `esc_regel`), `heimordner/eintraege.rs`, die drei Sprachtabellen `de.rs`, `en.rs`, `fr.rs`, `sprache/tabelle/mod.rs`, `kommandos/zulaessigkeit.rs`, `kommandos/fokus.rs`, `belegungsmodell.rs`, `menuemodell.rs`, `belegungsausgabe.rs`, `vorschaumodell.rs`, `markdown.rs`, `hervorhebung.rs`, `editormodell.rs`, `fenstermodell.rs`, `appkit/ereignisse.rs`, `appkit/hauptfaden.rs`, `appkit/quicknote.rs`, `appkit/tabelle.rs`, `appkit/textautomatik.rs`, `appkit/vorschau.rs`, `quicknote.rs`.
- **Abschnittsweise gelesen:** `HowTo.md` (Tastaturbelegung, Quicknote, Notizordner, Protokolle, Geheimnisse bis zur Tabelle), `CLAUDE.md` (der geänderte Absatz), `.claude/rules/coding-was-man-nicht-sieht.md` (bis zum Absatz über das Prüfziel `hauptfaden`, der Rest über Suchläufe), `resources/default-settings.toml` (Suchlauf).
- **Nicht geöffnet:** die Dateien der Zeile `**Not-opened:**`. Die fünf Probendateien des Kerns sind über ihren grünen Lauf und über Suchläufe nach Probennamen belegt, nicht gelesen; von `eintragsansicht.rs` sind allein der Modulkopf zur mehrzeiligen Zelle und zwei Suchläufe gelesen, der Diff nicht. `sprache/schluessel.rs` ist über die Probe `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` und einen Lauf über tote Schlüssel belegt.

## Findings by theme

### Anleitung und Kommentare hinter dem Code

**L1 (Low): `HowTo.md` sagt, jede unbekannte Kennung verwerfe die eigene `keymap.toml`.** Der Absatz unter `## Die Tastaturbelegung` („Eine fehlerhafte `keymap.toml` wird als Ganzes verworfen … Unbekannte Kennung …“) kennt die Duldung aus E6 nicht. `Belegung::bauen` (`crates/krk-core/src/tasten/belegung.rs`) übergeht seit `871f1062` einen Eintrag mit einer Kennung aus `ZURUECKGEZOGENE_KENNUNGEN`. Betroffen: die Anleitung im Releasepaket. Abgelegt als `261111-1104-howto-sagt-jede-unbekannte-kennung-verwerfe-die-eigene-keymap-und-eine-zurueckgezogene-wird-seit-heute-uebergangen.md`.

**L2 (Low): zwei Kommentare in `belegung.rs` begründen das Abhaken mit der gefallenen Notiztabelle.** `Wirkungsbereich::Aufgaben` und der Kommentar über dem Zweig der Eintragsbefehle in `Kommando::wirkungsbereich` sagen „eine Notiz hat kein Kaestchen“. Abgelegt als `261111-1104-zwei-kommentare-in-belegung-rs-begruenden-das-abhaken-noch-mit-der-notiz-die-kein-kaestchen-hat.md`.

### Geprüft und ohne Befund

**Bestand des Nutzers (Punkt 1 des Auftrags).**

- `heimordner::bereitstellen` gibt allein den Namen aus `Sonderdatei::ALLE` (`tasks.txt`, `secrets.txt`) einen Pfad, legt mit `create_new` an und `minutes/` mit `fs::create_dir`. Kein Aufruf öffnet, liest oder entfernt etwas Vorhandenes; ein vorhandenes `minutes/` wird allein über `metadata` gefragt. `notes.txt`, `appointments.md`, `note-1.txt`, `note-2.txt` nennt der Betriebscode nicht mehr.
- `ohne_ersetzen_umbenennen` stellt vor `fs::hard_link` keine Frage und ruft kein `fs::rename`. Ein vorhandenes Ziel lässt `link(2)` scheitern.
- Übernahme von `scratch.txt` (`quicknotesicherung.rs`, `uebernehmen`, `lesen`, `sichern`), jeder Ausgang der Tafel aus E3 am Rumpf nachvollzogen:
  - *Umbenannt:* gelesen wird `quicknote.txt`, dieselbe Inode.
  - *BeideStehen:* gelesen wird `quicknote.txt`; `scratch.txt` wird nicht geöffnet.
  - *AlterNameBleibt:* gelesen wird `quicknote.txt`; das spätere Sichern geht über `ablage::atomar` und lässt den alten Namen auf der alten Inode.
  - *Gescheitert, `quicknote.txt` fehlt:* gelesen wird `scratch.txt`, der Stempel ist `None`. Das erste Sichern legt `quicknote.txt` exklusiv an; steht sie inzwischen, antwortet `sichern` mit `FremdGeaendert` und schreibt nicht. `scratch.txt` schreibt der Faden nie.
  - *Gescheitert, `quicknote.txt` steht:* gelesen wird `quicknote.txt`.
- Vor der Antwort auf das Lesen nimmt `Sicherung::schreiben` keinen Auftrag an, und die Fläche ist nicht bearbeitbar (`quicknote_sicherung_einziehen`). Ein leerer Puffer kann die Datei nicht überschreiben.

**`secrets.txt` (Punkt 2).** `heimordner/tresor.rs`, `schluesselmerker.rs` und `verbindung.rs` sind im Bereich unverändert. `geheimnisse.rs` und `tresorweg.rs` ändern allein Kommentare. `oeffnungsweg` in `editor.rs` führt `Sonderdatei::Geheimnisse` weiter über `PinErfragen` oder `Unterlassen`. `kein_weg_schreibt_klartext_nach_secrets_txt` und `die_sperre_fragt_die_sonderdatei_vor_dem_lesen` sind grün.

**Namensregel (Punkt 3).** `protokollname` ist rein. `im_protokollordner` antwortet für `Ort::Fern` und `Ort::Archiv` und ohne geltenden Notizort nein. `anlegename` ruft `protokollname` allein für `Anlegeart::Datei` im Protokollordner; `anlegen_ausfuehren` ruft es allein im lokalen Zweig, nach der Abzweigung zu `fern_anlegen`. Angelegt wird über `operation::datei_anlegen` wie in jedem Ordner. Blatt und Name hängen an derselben Erhebung (`im_protokollordner` reist aus `anlegen_in` mit).

**`cmd+m` (Punkt 4).** `notizordner_oeffnen` nimmt das Ziel als Wert durch dieselben drei Funktionen. `notizordnerausgang` gibt für das Ziel `Protokolle` mit `protokollordner: Err(_)` `NurSaetze`, bei abweichender Nummer `Zurueckstellen`, bei abweichendem Stand `NurSaetze`. Das Blatt hängt allein an `TabFokusUndBlatt`. `tab_oeffnen` und `tab_waehlen` (`appkit/tabelle.rs`) setzen den Tab synchron, also liest `anlegen_in` den Ordner `minutes/`. `notizordner_erheben` berührt die Ablage nicht mehr.

**Eigene `keymap.toml` (Punkt 5).** Der Zweig der Duldung steht vor der Abweisung und allein im Bau mit Wortschatz; `endgueltig_loeschen` steht nicht in der Liste. Die Proben dazu (`crates/krk-core/tests/belegung.rs`, nach Namen gesucht, nicht gelesen) sind grün.

**Listen und Zählproben (Punkt 6).** `jede_variante_von_kommando_steht_genau_einmal_in_kennungen`, `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`, `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` und `jeder_dieser_befehle_hat_einen_eigenen_ausfuehrungszweig` sind grün; der Zweig `Kommando::ProtokollAnlegen` steht in `kommando_ausfuehren_bei`.

**Sprachtabelle (Punkt 7).** Jeder hinzugefügte Eintrag steht in `de.rs`, `en.rs` und `fr.rs` und sagt dasselbe. Ein Lauf über jede Variante von `Text` findet keinen Schlüssel ohne Leser außerhalb der Tabellen (er zählt Proben mit). `grep -rnE 'Notiztabelle|Termintabelle|Zettel|appointment|notes\.txt|scratch'` über die Tabellen liefert nichts.

**Anleitungen und Regeln (Punkt 8).** Die vier übernommenen Aussagen zur Geheimnistabelle in `HowTo.md`:

| Aussage | Beleg | Urteil |
|---|---|---|
| Eine Zeile wächst beim Tippen mit | Modulkopf von `eintragsansicht.rs`, „Die mehrzeilige Notizzelle“ (`Notizfeld`) | hält für die Notizenspalte; das Schrumpfen ist laut Modulkopf nicht gemessen |
| `cmd+return` öffnet den Namen | `eintrag_bearbeiten` über `Handlung::Bearbeiten`; die Spalte am Diff von `eintragsansicht.rs` nicht nachgelesen | inference: hält |
| Suchen, Ersetzen, Zeilensprung ausgegraut | `form_passt` in `zulaessigkeit.rs`, Tafel `IN_DER_GEHEIMNISTABELLE` | hält |
| `esc` übernimmt in den Notizen und verwirft in den fünf anderen Spalten | `esc_regel` und `zelle_abbrechen` in `editor.rs` | hält |

`CLAUDE.md` und die Regeldatei nennen keine Probe und keine Funktion, die im Baum fehlt (neun Probennamen aus den geänderten Absätzen im Lauf gefunden).

**Stehen gelassenes ohne Rufer (Punkt 9).** Keines ist ein Defekt.

- `Notizen::notiz`: Rufer allein in `crates/krk-core/tests/heimordner.rs`. Öffentliche Lesefunktion der Form, auf der `secrets.txt` ruht.
- `Heimordner::ist_vorgabeort`: Rufer allein in denselben Proben. Toter Betriebscode, harmlos.
- `fehlt` an `Textstand::KeinGueltigesZiel`: der Doc-Kommentar sagt, dass kein Aufrufer es liest. Also seen: `260926-0257_*_textstand-unlesbar-traegt-einen-offenen-deskriptor-den-seit-dem-wegfall-des-zettelwegs-kein-aufrufer-liest.md` behandelt denselben Typ.
- `Sitzung::terminrichtung`: bleibt lesbar, wird als `None` geschrieben. Eine `session.toml` mit dem Feld lädt weiter.

## Cross-cutting observations

- **Die Kommentare sind dem Code gefolgt, bis auf Einzelstellen.** L2 und der schon abgelegte Defekt `261111-1056-der-modulkopf-der-eintragsansicht-…` sind dieselbe Art: ein Satz, der beim Entfernen in derselben Datei neben einem berichtigten stehen blieb.
- **Zwei Fassungen an einem geteilten Notizordner.** Liegt der Notizordner in einem synchronisierten Speicher und läuft auf einem zweiten Gerät noch eine Fassung vor 4.0.0, legt jene nach der Übernahme wieder eine `scratch.txt` an; danach stehen beide, es gilt `quicknote.txt`, und die Meldung kommt bei jedem ersten Öffnen. Verloren geht nichts. Die Anleitung sagt dazu nichts; ein Rückweg auf ältere Fassungen ist nirgends zugesagt. Kein Datensatz.
- **Ein Ordner namens `scratch.txt` ohne `quicknote.txt`** führt über „Gescheitert“ zum Lesen des Ordners und sperrt die Sicherung bis zum Neustart. Die Fassung davor sperrte in derselben Lage ebenso. Kein Datensatz.
- **`HowTo.md` beschreibt das Menü „Navigation“ nicht**, obwohl die Betreffzeile von `96363ac4` es nennt. C1.9 verlangt allein, dass die alten Titel fehlen, und das ist erfüllt. Kein Datensatz.

## Was diese Durchsicht nicht belegt

- Das laufende Bündel ist nicht gestartet. Offen bleiben die Nutzerpunkte aus `## Where this work stops` des Plans, zuerst ob `cmd+m` das Fenster minimiert.
- Die zwei offenen Entscheidungen des Pakets und die zwei schon abgelegten Defekte sind nicht neu beurteilt.

## Recommended sequencing

1. Vor der Auslieferung: die Nutzerpunkte am Bündel (`cmd+m`, Übernahme einer wirklichen `scratch.txt`, eine vorher gesicherte `secrets.txt`).
2. L1 lohnt vor der Auslieferung, weil `HowTo.md` im Releasepaket mitreist; hält sie nicht an.
3. L2 mit dem nächsten Aufräumen.
