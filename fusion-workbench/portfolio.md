# Portfolio

**Generated:** 260813-2203 (by playmaker session 260813-2203-playmaker-direct-dispatch)
**Domain bias:** code

---

**Was ansteht.** Zwei Ideen liegen frisch im Speicher, und eine davon ist die billigste
Arbeit, die dieses Projekt gerade anzubieten hat: der Notizzettel. Der vorgesehene
Web-Betrachter bleibt Rang 1 unter den Circles, aber er ist es mangels Mitbewerbern und
verlangt vor der Aktivierung eine Untersuchung. Neu seit dem letzten Lauf ist, dass der
Auslieferungsweg vollständig funktioniert: drei Versionstags stehen, der Arbeitsbaum ist
sauber, und das letzte offene Abnahmekriterium der Runde 8 ist damit erfüllt.

---

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt den
Marker für aktiv (`_t_`). Das ist der reguläre Zustand nach einem Abschluss und kein Befund.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
vorgesehene Circle; vor der Aktivierung stehen unverändert eine Untersuchung des
Darstellungsmittels und eine Klärungsrunde über sechs Fragen.

### Rang 1 — `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters und
wird über die Tastatur bedient, mit Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten:** eine Kante, auf die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`), die den beschränkten Abschluss trägt.

Der Rang ist keine Auswahl. Seit dem Abschluss der Runde 8 am 260813-1415 ist dieser Circle der
einzige vorgesehene, und eine Rangfolge mit einem Element sagt nichts über relative Reife. Wir
stützen die Empfehlung deshalb wie in den fünf Läufen davor auf absolute Signale, und die haben
sich seit dem 260813-1510 an genau einer Stelle bewegt — nicht am Circle, sondern am Projekt.
Der vorige Vermerk stellte dem Aktivierungszeitpunkt zwei Nutzerschritte voran, die den
Auslieferungsweg anhielten: ein fehlender Versionstag und geänderte verfolgte Dateien. Beide sind
erledigt. `git tag --points-at HEAD` liefert `v0.2.1`, `Cargo.toml:13` führt dieselbe Zahl, und
`git status --porcelain --untracked-files=no` ist leer. Station 1 von `cargo xtask release`
vergleicht genau diese beiden Werte (`xtask/src/release.rs`, `stand_pruefen` ab Zeile 208) und
kommt jetzt durch. Was bleibt, hängt am Circle selbst und ist unverändert: das Mittel der
Darstellung von Web-Inhalt ist offen und gehört in eine eigene Untersuchung vor den Plan, und die
Klärungsrunde trägt weiterhin sechs Fragen. Dazu binden zwei ungemessene Punkte, die
Verfügbarkeitsprüfung für Schnittstellen ab macOS 26
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
und die Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`). Die Prüfung
„alle Abhängigkeiten kohärent abgeschlossen" ist für diesen Circle nicht eingerechnet: seine
einzige Kante führt auf einen beschränkten Abschluss, und der ist ein Endzustand, den keine
künftige Arbeit zurücknimmt (`rules/circle-records.md`, `### Worked transitions`). Ein Kriterium,
dessen Wert sich nie ändern kann, trägt kein Rangsignal.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
— eine Idee, kein Split nötig, und alle Bauteile liegen auf der Platte; ihre offenen Punkte sind
Klärungsstoff und keine Vorab-Untersuchung.

```
/fusion:direct shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md
```

### Rang 1 — der Notizzettel

`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`

Eine Fläche, die auf Tastendruck mittig über den anderen Bereichen erscheint, sich selbst sichert
und mit `Esc` schließt. Der Eintrag trägt genau eine Idee; seine sechs offenen Punkte sind Fragen
an eine Klärungsrunde und keine getrennten Vorhaben.

Er steht auf Rang 1, weil er der einzige Kandidat des Portfolios ist, der ohne eine
vorgeschaltete Untersuchung geshaped werden kann. Alles, was er braucht, liegt im Baum: der
Editorkern in `crates/krk-core/src/text/`, die eine Zulässigkeitsregel in
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, die Ablage unter
`~/Library/Application Support/KRK/`. Die Zahlen des Eintrags haben wir nachgezählt und sie
stimmen: `resources/default-keymap.toml` führt 82 Funktionen mit 88 Kombinationen, drei Funktionen
davon ohne Taste.

Ein Punkt kommt hinzu, den der Eintrag nicht nennt und der in die Klärungsrunde gehört. Der
Eintrag setzt `Esc` als Weg heraus, und `Esc` ist Gegenstand eines offenen Entscheids: der Befehl
`abbrechen` liegt auf `Esc` und wird seit Schritt 3 der Runde 7 geschluckt, statt an AppKit
weiterzulaufen
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/decisions/260813-0320_*_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`).
Wer dem Zettel `Esc` gibt, legt eine dritte Bedeutung auf dieselbe Taste.

### Rang 2 — das zweite Kürzel für den Editor-Einstieg

`shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`

Eine zweite, besser erreichbare Tastenkombination für dieselbe Funktion, die heute allein auf
`F4` liegt. Auch dies ist genau eine Idee.

Rang 2 und nicht Rang 1, weil der Eintrag seine eigene Vorbedingung mitliefert und sie noch nicht
erfüllt ist: er vermutet, `F4` sei nur deshalb hakelig, weil auf Apple-Tastaturen die
Systemeinstellung „F1, F2 usw. als Standard-Funktionstasten verwenden" ausgeschaltet ist. Trifft
das zu, löst sich die Idee auf. Die Prüfung kostet einen Blick in die Systemeinstellungen, und
sie vor dem Shapen zu machen ist billiger, als einen Circle auf ein Problem zu setzen, das keines
ist.

Zwei Feststellungen aus dem Baum, beide für die spätere Klärungsrunde:

- **Die naheliegende Buchstabenfamilie ist voll.** Alle vier Cmd-Ebenen von `e` sind vergeben:
  `cmd+e` auf `editor_aus_vorschau`, `shift+cmd+e` auf `fokus_editor`, `opt+cmd+e` auf
  `editor_schliessen`, `ctrl+cmd+e` auf `editor_ansicht_umschalten`
  (`resources/default-keymap.toml`, Zeilen 691, 700, 708, 733). Eine neue Kombination für
  `bearbeiten` muss deshalb aus einer anderen Familie kommen oder eine bestehende verdrängen.
- **Der Eintrag kippt eine ausgeschriebene Überlegung, und das ist eine Nutzerentscheidung.** Der
  Kommentar an `bearbeiten` (`resources/default-keymap.toml:164-174`) begründet ausdrücklich,
  warum kein Cmd-Kürzel danebensteht: der zweite Einstiegsweg in den Editor sei kein zweiter Weg
  auf dieselbe Handlung, sondern eine eigene Funktion mit eigener Quelle. Der Eintrag benennt das
  selbst. Eine Runde ersetzt diese Überlegung, sie übergeht sie nicht.

Der Eintrag ist keine Dublette zum offenen Defekt
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Defekt betrifft dasselbe `F4` mit einem anderen Symptom, nämlich das stille Laden am schmalen
Fenster, und er bliebe bestehen, gleich welche zweite Kombination hinzukommt.

### Zur Konsolidierung

Zwei Einträge auf offen (`_o_`), keiner auf empfohlen (`_p_`). Kein Eintrag trägt mehr als eine
Idee, also ist kein Split vorzuschlagen. Keine Dublette und keine Fastdublette, weder zwischen
den Einträgen noch innerhalb eines. Kein Eintrag ist defekt- oder entscheidungsförmig; beide sind
Vorhaben. Der dritte Eintrag im Speicher,
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md`, steht auf geschlossen und
nennt in seinem Rumpf den Circle, der aus ihm wurde.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss in einem Satz |
|---|---|---|
| `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | `_c_` | Kohärent am 260813-1415, als erste Runde dieses Projekts: Titelleiste mit Name und Version, Über-Dialog, semantische Versionstags und eine vierte Bedingung in der Zulässigkeitsregel. |
| `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` | `_b_` | Beschränkt am 260813: Suche in der Belegungsansicht, alle 82 Funktionen im Menü, eine weitere Instanz mit zwei Sperren über `flock`; gebaut, nicht abgenommen. |
| `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | `_b_` | Beschränkt am 260812: Teilen, Ordnersprung, geschützte Ablage, gerendertes Markdown in der Vorschau und eine Statuszeile über die volle Breite; gebaut, nicht abgenommen. |
| `260811-1304-statusleiste-mit-bereichsschaltern` | `_b_` | Beschränkt am 260812-0820: Breitenregel über Anteile für alle fünf Bereiche und ausblendbares linkes Dateifenster; 13 Abnahmekriterien nur am laufenden Bündel zu sehen. |
| `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` | `_b_` | Beschränkt am 260811-2210: Pfad des Ordners und des Eintrags kopieren, Öffnen mit dem Standardprogramm, `Cmd+W` aus jedem Fokus; alle 62 Abnahmekriterien offen. |

Ältere Abschlüsse: `260809-2040-tastenbelegung-als-markdown-in-downloads` (beschränkt am
260811-1415), `260807-2116-eingebauter-editor-mit-textmarken` (beschränkt am 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (beschränkt am 260807-1035).

## Archived (_s_ / _d_)

(keiner) — kein Circle-Datensatz trägt überholt (`_s_`) oder zurückgestellt (`_d_`).

## Warnings

**1. `CLAUDE.md` beschreibt ein Projekt mit vier Runden, und es sind acht.** Das ist die
folgenreichste Warnung dieses Laufs, weil jeder Agent diese Datei zuerst liest. Falsch sind unter
anderem: die Tabelle „Vier Runden sind gefahren" (der Baum trägt neun Circles, acht davon
gefahren), der Satz „Alle vier Runden sind als beschränkter Abschluss geschlossen" samt der daran
hängenden Bemerkung zur Rangheuristik (die Runde 8 ist kohärent geschlossen), der Absatz „Zwei
Circles sind vorgesehen und nicht gefahren" (es ist einer; die Statusleiste ist als Runde 5
gefahren und beschränkt geschlossen) und die Zeile „Die Statusleiste steht auf Rang 1". Der
Projektstand trägt „Geprüft am 260811-2230" und ist zwei Tage alt. Zwei der falschen Zahlen haben
bereits Defektdatensätze:
`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`
(der Baum trägt inzwischen 76, am 260813-2203 nachgezählt in
`crates/krk-core/src/tasten/belegung.rs:566`) und
`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`.
Für die Rundenzahl selbst gibt es keinen Datensatz. `/fusion:revise-claude-md` ist der Weg; der
Playmaker legt keine Defekte an.

**2. Das letzte offene Abnahmekriterium der Runde 8 ist erfüllt, und kein Datensatz sagt es.**
Die `## Closure note` von `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_*_circle.md`
benennt C3.15 als den einen verbleibenden Nutzerschritt: den Tag `v0.1.0` auf den Abschlusscommit.
`git rev-list -n1 v0.1.0` liefert `3a0a4bf`, und das ist genau dieser Commit. Die Runde steht
damit bei 59 von 59, aber Spec und Abschlussnotiz führen sie weiter bei 58. Das ist Arbeit für den
`reconciler`, nicht für den Playmaker.

**3. Der Defekt am doppelt belegten Ausgabeort besteht unverändert.** `cargo xtask bundle` und
`cargo xtask release` legen beide `target/KRK.app` an, am 260813-2203 in
`xtask/src/bundle.rs:50` (`BUENDELNAME`) und in `xtask/src/release.rs:121` nachgelesen: der
Auslieferungsweg ruft dieselbe `bundle::vorbereiten()`. Ein gewöhnliches `make run` überschreibt
damit ein beglaubigtes Bündel, und das kostet Minuten für einen Tastendruck, der wie ein
gewöhnlicher aussieht
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`,
drei Zuschnitte mit Kosten). Der Weg hat seit dem 260813-1556 eine Station mehr, der Defekt ist
dadurch weder besser noch schlechter geworden.

**4. 19 Entscheidungsdatensätze sind offen, einer ist beantwortet und nicht umgesetzt.** Vier der
offenen stammen aus der Runde 7 und sind gebaut, ohne dass der Nutzer geantwortet hätte. Keiner
hält einen Planschritt auf; alle binden künftige Arbeit. Die Liste liefert:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**5. 70 Defekte sind offen**, 10 davon im gemeinsamen Speicher. Der Schwerpunkt liegt bei den
drei jüngsten Runden: 25 in der Runde 6, 16 in der Runde 8, 14 in der Runde 7. Der Anteil an
Prosa, die dem Code hinterherläuft, ist in allen dreien hoch.

**6. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge. Die einzige Kante des Portfolios führt vom
Web-Betrachter auf die Runde 1, und die ist terminal. An keinen Circle-Datensatz ist eine
`## Dependency warning` angehängt worden.

**7. Kein neuer Vermerk zu gealterter Grundlage, weil kein Circle den Zustand gewechselt hat.**
Seit dem Lauf vom 260813-1510 trägt kein Datensatz einen anderen Marker. Die Auslösebedingung für
`parent-grounding-stale` ist an keiner Stelle erfüllt, und wir haben deshalb keinen Vermerk
angehängt.

**8. Der Datensatz des Web-Betrachters trägt 820 Zeilen und elf Playmaker-Abschnitte aus sechs
Läufen.** Die Länge wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne dass an ihm
gearbeitet würde. Der Vermerk dieses Laufs ist deshalb kurz gehalten und trägt allein die eine
Änderung nach; er wiederholt den Vorschlag vom 260813-1510 nicht. Wer den Stand lesen will, liest
die letzten beiden Abschnitte, nicht alle elf.
