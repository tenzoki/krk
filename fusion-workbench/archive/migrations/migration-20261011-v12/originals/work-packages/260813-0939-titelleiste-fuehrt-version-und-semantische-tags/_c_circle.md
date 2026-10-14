# Die Titelleiste führt links Namen und Version, und semantische Versionstags decken die Zahl

---
**Domain:** code
**Status:** closed
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_*_plan-titelleiste-fuehrt-version-und-semantische-tags.md
**Active session history:** circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md

---

## Directive

Die Titelleiste von KRK trägt links einen eigenen Bereich mit Namen und Version, geschrieben als `KRK 0.1.0`. Der absolute Pfad bleibt mittig und ungekürzt, wie C11 der Runde 2 es zusagt; der neue Bereich steht neben ihm und nicht vor ihm. Die angezeigte Zahl ist die aus `[workspace.package]` der `Cargo.toml`, ohne Zusatz für den Arbeitsstand: ein Bau aus einem geänderten Baum zeigt dieselbe Zahl wie das ausgelieferte Bündel. Verbindlich wird die Zahl durch semantische Versionstags. Jede Auslieferung bekommt einen Git-Tag `v<version>`, ein eigener Abschnitt in `README.md` sagt, wann Major, Minor oder Patch steigt, und `cargo xtask release` bricht ab, wenn HEAD keinen Tag trägt, der zur Version in der `Cargo.toml` passt. Den Tag setzt der Nutzer und nicht das Werkzeug. `cargo xtask bundle` und `make check` bleiben unangetastet, damit die tägliche Arbeit nicht an der neuen Prüfung hängt.

## Grounding snapshot

Vorläufig. Ein anticipated Circle trägt noch keine erhobene Grounding; dieser Abschnitt hält fest, was beim Lesen des Baums am 260813-0939 sichtbar war, und wird bei der Aktivierung ersetzt.

### Woher das Vorhaben kommt

Der Nutzer hat es am 260813-0822 als Backlog-Eintrag `shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` abgelegt. Der Eintrag koppelt zwei Vorhaben ausdrücklich in einen Zug, die Anzeige und die Tags, und begründet die Kopplung so: "eine Version anzeigen, die nirgends verbindlich festgelegt ist, wäre eine Zahl ohne Deckung". Der Eintrag ist mit der Anlage dieses Circles geschlossen.

Vier Fragen einer Klärungsrunde hat der Nutzer beantwortet:

1. **Platzierung.** Ein eigener linker Bereich in der Titelleiste. Ein neues AppKit-Modul dafür ist eingeplant und ausdrücklich akzeptiert. Die Alternative, Namen und Version als Text vor den Pfad in einen Titelstring zu setzen, ist verworfen: der Name fräße dann Breite, und macOS kürzte bei schmalem Fenster den Pfad, den KRK absichtlich nicht kürzt.
2. **Tags.** Regel plus Prüfung bei der Auslieferung. Git-Tag `v<version>` bei jeder Auslieferung, ein Abschnitt in `README.md` über die Stufen, und ein Abbruch in `cargo xtask release` bei fehlendem oder unpassendem Tag auf HEAD. Dass das Werkzeug den Tag selbst erzeugt, ist verworfen.
3. **Arbeitsstand im Titel.** Nein. Immer die Zahl aus der `Cargo.toml`, kein `-dev`-Zusatz, kein neuer Bauschritt, der den Git-Stand zur Bauzeit kennt.
4. **Schreibweise und Zahl.** `KRK 0.1.0`, ohne spitze Klammern und mit der tatsächlichen Zahl des Baums. Die spitzen Klammern im Entwurf waren Platzhalter-Notation. Diese Runde hebt die Version nicht auf 1.0.0.

### Was Antwort 3 an der Kopplungsbegründung ändert

Die dritte Antwort schwächt die Begründung ab, mit der der Backlog-Eintrag beide Hälften aneinanderbindet. Der Eintrag argumentiert, eine angezeigte Version ohne verbindliche Festlegung sei eine Zahl ohne Deckung. Ohne Kennzeichnung des Arbeitsstands zeigt aber jeder Bau aus einem geänderten Baum dieselbe Zahl wie das ausgelieferte Bündel, und für diese Bauten deckt kein Tag die Zahl. Die Anzeige sagt damit "0.1.0" auch dort, wo der Baum nicht der getaggte ist.

Die Kopplung bleibt trotzdem gewollt, und der Nutzer hat beide Antworten in derselben Runde gegeben. Der Restpunkt gehört hier hingeschrieben, statt still zu bleiben: die Tags decken die Zahl an der Auslieferung, nicht an jedem Bau. Wer den Deckungsanspruch später auf jeden Bau ausdehnen will, braucht genau den Zusatz, den Antwort 3 verworfen hat.

### Was der Baum heute trägt

**Der Name steht heute nur bis zum ersten Pfad in der Titelleiste.** `crates/krk-ui/src/appkit/fenster.rs:436` setzt den Titel beim Aufbau einmal auf "KRK"; `Anwendungsdelegierter::titel_nachziehen` (`appkit/anwendung.rs:3673`) überschreibt ihn beim ersten Fokus-, Ordner-, Tab- oder Dateiwechsel mit dem Pfad aus `krk-ui/src/fenstertitel.rs`. Danach steht der Name nirgends mehr auf dem Schirm. Ein Stellvertretersymbol oder ein Titelleisten-Zusatz besteht nicht: `NSTitlebarAccessoryViewController` kommt im ganzen Baum nicht vor, und `appkit/fenster.rs` baut das Fenster mit den vier gewöhnlichen Stilmarken.

**Die Version wohnt an einer Stelle und ist zur Übersetzzeit schon da.** `[workspace.package] version = "0.1.0"` in der Wurzel-`Cargo.toml`, geerbt über `version.workspace = true`, unter anderem von `krk-ui`. `resources/Info.plist` trägt bei `CFBundleShortVersionString` allein den Platzhalter `__KRK_VERSION__`, den `cargo xtask bundle` beim Kopieren ersetzt und ohne den es abbricht. `krk-ui` liest `env!("CARGO_PKG_VERSION")` heute nicht, `krk-bench` und `xtask` tun es an fünf Stellen. Antwort 3 ist damit ohne neuen Bauschritt einzuhalten: die Zahl liegt über dieselbe Vererbung an, die die `Info.plist` füllt.

**Tags gibt es keine.** `git tag -l` liefert in diesem Baum nichts, bei sieben geschlossenen Runden. Der erste Tag entsteht also in dieser Runde oder danach, und wer ihn setzt, ist offen (siehe unten).

**`xtask` ruft heute kein `git`.** Weder `bundle.rs` noch `release.rs` noch `sign.rs` nennt das Programm. Die Tag-Prüfung ist die erste Stelle, an der das Bauwerkzeug den Zustand des Arbeitsbaums befragt.

**Der Auslieferungsweg steht und ist sechs Stationen lang.** `cargo xtask release` (`xtask/src/release.rs`) prüft die AppKit-Grenze, übersetzt beide Mac-Ziele, fügt sie mit `lipo` zusammen, montiert dasselbe Bündel wie `bundle`, signiert mit einer Developer-ID-Identität und beglaubigt über `notarytool` und `stapler`. Die Prüfung aus Antwort 2 kommt als weitere Station hinzu.

### Was die Anzeige berührt

**C11 der Runde 2 ist die einzige bestehende Zusage über die Titelleiste**, und zwei ihrer elf Abnahmekriterien berührt der neue Bereich unmittelbar. Das erste lautet: "Der Fenstertitel trägt einen absoluten Pfad und nicht mehr allein den Namen der Anwendung." Es bleibt erfüllt, sobald Name und Version neben dem Titel stehen und nicht darin. Das neunte lautet: "Der Pfad steht ungekürzt. KRK kürzt den Benutzerordner nicht auf eine Tilde und lässt keine Zwischenordner aus; was der Titelbalken nicht fasst, kürzt macOS selbst." Ein linker Bereich nimmt Breite aus der Titelleiste, also kürzt macOS bei schmalem Fenster früher. Die Zusage ist damit gehalten, weil sie das Kürzen durch KRK ausschließt und nicht das durch macOS. Der Aktivierungs-Spec schreibt beides aus, statt es abzuleiten. Quelle: `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md`, Abschnitt `### C11: Der volle Pfad im Fenstertitel`.

**L4 wird berührt und bekommt keine neue Zahl.** Die vierte Zeitzusage misst den Kaltstart bis zur bedienbaren Oberfläche gegen 1000 ms, und der neue Bereich entsteht beim Aufbau des Fensters. Dieselbe Lage hatten C9 und C11 der Runde 2, und dieselbe Antwort gilt: keine elfte Zahl, weil der Abnahmelauf KRK im Vordergrund verlangt und damit Nutzerarbeit ist. Die zehn Zahlen bleiben unverändert.

**Die Belegung und die vier vollständigen Aufzählungen bleiben unberührt.** Der Bereich wird geschrieben und nicht bedient, bringt also keinen Tastenbefehl mit. `Kommando`, `Wirkungsbereich`, `Bereich` und `Fokus` wachsen nicht, und `resources/default-keymap.toml` bekommt keinen Eintrag. Genau so lagen C9, C10 und C11 der Runde 2.

**Ein neues Modul unter `crates/krk-ui/src/appkit/` bringt zwei Pflichten mit.** Es trägt den Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` im Modulkopf, weil `objc2` keine Verfügbarkeitsangaben führt und der Übersetzer die Untergrenze macOS 15 nicht hält; und es wird in der Modulliste von `appkit/mod.rs` angemeldet, die heute 27 Namen führt.

### Offene Fragen

Drei Punkte sind beim Lesen des Baums aufgekommen, die die vier Antworten nicht abdecken. Keiner davon ändert die Directive, jeder ist Eingabe für die Klärungsrunde bei der Aktivierung. Jeder trägt einen eigenen Datensatz in `decisions/` dieses Circles.

1. **Bekommt KRK zugleich einen Eintrag "Über KRK" im Anwendungsmenü?** Das Menü der Runde 7 führt keinen, und auf dem Mac ist er der übliche Ort für Namen und Version. Wer ihn später nachträgt, hat zwei Anzeigen derselben Zahl. `decisions/260813-0939_*_bekommt-krk-einen-eintrag-ueber-krk-im-anwendungsmenue.md`
2. **Wer setzt den ersten Tag `v0.1.0`, und wann?** Nach dieser Runde bricht `cargo xtask release` ab, solange HEAD keinen passenden Tag trägt, und das Werkzeug darf ihn nach Antwort 2 nicht selbst erzeugen. Ohne eine Festlegung ist der Auslieferungsweg ab dem Abschluss der Runde abweisend. `decisions/260813-0939_*_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md`
3. **Reicht ein passender Tag auf HEAD, oder muss der Arbeitsbaum sauber sein?** Ein Tag zeigt auf einen Commit, nicht auf den Baum. Ein geänderter Baum liefert ein Bündel, das der Tag nicht benennt, und die Prüfung ließe es durch. `decisions/260813-0939_*_reicht-ein-tag-auf-head-oder-muss-der-arbeitsbaum-sauber-sein.md`

### Was dieser Circle nicht festlegt

Womit der linke Bereich gebaut wird, ist offen und gehört in den Plan. Antwort 1 sagt, dass ein neues AppKit-Modul dafür recht ist, und nennt keine Klasse.

Ebenso offen: an welcher Stelle der sechs Stationen von `cargo xtask release` die Tag-Prüfung sitzt, wie die Meldung bei einem Abbruch lautet, und wie der Abschnitt in `README.md` überschrieben ist. Der Aktivierungs-Spec entscheidet, was der Nutzer davon sieht; die Bauart entscheidet der Planner.

## Dependencies

Kein anderer Circle muss vor diesem laufen. Vier Stellen binden ihn inhaltlich.

- `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md`, Fähigkeit **C11**. Die einzige bestehende Zusage über die Titelleiste. Ihr erstes und ihr neuntes Abnahmekriterium sind oben ausgeschrieben; dieser Circle stellt einen Bereich daneben und ändert den Titel selbst nicht.
- `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`, die Runde 7. Sie hat das vollständige Hauptmenü gebaut, in dem ein Eintrag "Über KRK" fehlt. Die erste offene Frage oben hängt daran.
- `shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`. Offen, betrifft denselben Auslieferungsweg, den die Tag-Prüfung erweitert.
- `shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`. Offen, aus demselben Anlass: der Weg von einem Bau zu einer Auslieferung ist im Werkzeug heute nicht ausgeschildert. Die Tag-Prüfung ist das erste Schild.

Die Technologiewahl vom 260802-1150 bindet wie überall: Rust mit AppKit über `objc2`, außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26.

## Turn log


- Turn 1 (Sitzung 260813-1006): abgeschlossen. Vier Straenge gebaut, Commits 5df3909..21dbc59 (sechs, davon vier am Baum). 15 von 16 Planschritten auf [DONE]; offen bleibt allein E2, die Abnahme am Buendel, die Nutzerarbeit ist. make check exit 0. Die Durchsicht hat einen hohen Befund gefunden: fenster_einblenden ist nach dem Schliessen des Fensters nicht mehr erreichbar. Coherence-Urteil: review-needed, weil dieser Befund eine Randbedingung des Spec bricht. Sitzungsprotokoll: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md
- Turn 2 (Sitzung 260813-1006): abgeschlossen. Ein Schritt, F1: Kommando::FensterEinblenden steht auf der Ausnahmeliste, Commit ed0388e. Proben in kommandos::zulaessigkeit 11 auf 12. make check exit 0. Coherence-Urteil: die Regression ist weg, die sieben uebrigen Befunde sind niedrig oder Prosa. Sitzungsprotokoll: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md

## Activation proposal

**Vorgeschlagen am:** 260813-0958
**Playmaker-Lauf:** 260813-0958-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Klärungsrunde über die drei offenen Fragen
dieses Datensatzes, ohne vorgelagerte Untersuchung

Dieser Circle ist der empfohlene nächste Kandidat, und zum ersten Mal seit dem Abschluss der
Runde 1 steht die Empfehlung gegen einen Vergleichswert statt allein. Neben ihm liegt
`260804-0933-eingebauter-web-betrachter-im-vorschaufenster`, der seit dem 260804 vorgesehen ist
und in vier Läufen der einzige Kandidat war. Die Rangfolge kehrt sich mit diesem Lauf um, und
zwar an einem einzigen Unterschied: der Preis der Vorarbeit vor dem Plan.

**Die Vorarbeit ist eine Klärungsrunde und keine Untersuchung.** Der Betrachter hält in seinem
Abschnitt `## Was dieser Circle nicht festlegt` fest, dass das Mittel der Darstellung von
Web-Inhalt offen ist und in eine eigene Untersuchung vor dem Plan gehört. Für diesen Circle ist
das Mittel der Anzeige mit Antwort 1 der Klärungsrunde vom 260813 bereits eingegrenzt: ein neues
Modul unter `crates/krk-ui/src/appkit/`, vom Nutzer ausdrücklich akzeptiert. Welche Klasse es
verwendet, entscheidet der Planner am Baum und nicht eine vorgelagerte Erhebung.

**Die Grundlage ist vier Stunden alt und am Baum nachgeprüft.** Vier Tatsachenaussagen des
Abschnitts `## Grounding snapshot` sind bei diesem Lauf gegen den Baum gelesen worden, und alle
vier halten: `git tag -l` liefert null Tags; die Version steht einquellig in `[workspace.package]`
der Wurzel-`Cargo.toml` auf `0.1.0`; `NSTitlebarAccessoryViewController` kommt unter `crates/`
nicht vor; und `crates/krk-ui/src/appkit/mod.rs` führt 27 Modulnamen. Zum Vergleich: die
Grundlage des Betrachters stammt vom 260804 und ist seither viermal durch einen
`## Parent grounding stale`-Vermerk nachgezogen worden.

**Die drei offenen Fragen sind schmal und tragen je einen Datensatz.** Ob KRK zugleich einen
Eintrag „Über KRK" bekommt, wer den ersten Tag `v0.1.0` setzt, und ob ein passender Tag auf HEAD
genügt oder der Arbeitsbaum sauber sein muss. Keine davon verlangt eine Messung, keine hängt an
einer fremden Kiste. Die zweite ist die dringlichste: nach dieser Runde bricht
`cargo xtask release` ab, solange HEAD keinen passenden Tag trägt, und das Werkzeug darf ihn nach
Antwort 2 nicht selbst erzeugen. Ohne eine Festlegung ist der Auslieferungsweg ab dem Abschluss
der Runde abweisend. Zum Vergleich: vor der Aktivierung des Betrachters stehen sechs Fragen, von
denen eine ein offener projektweiter Nutzerentscheid ist.

**Die Abhängigkeitslage ist hier zum ersten Mal eine Auskunft.** Der Abschnitt
`## Dependencies` dieses Datensatzes lautet „Kein anderer Circle muss vor diesem laufen"; die vier
genannten Stellen sind inhaltliche Bindungen an einen Spec und an zwei offene Defekte, keine
Vorbedingungen. Damit ist die Prüfung „alle Abhängigkeiten kohärent abgeschlossen" für diesen
Circle leer erfüllt statt unentscheidbar. Der Betrachter hängt an der Runde 1, die beschränkt
abgeschlossen ist. Die Standardheuristik der Gewichtung `code` bleibt trotzdem ausgesetzt, weil
alle sieben gefahrenen Runden `_b_` tragen und der Marker damit nichts unterscheidet
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
offen seit dem 260806). Was hier zählt, ist die leere Menge an Vorbedingungen, nicht der Marker
an einer erfüllten.

**Das stärkste Gegenargument liegt an der Tag-Hälfte.** Sie sitzt am selben Auslieferungsweg wie
ein offener Defekt: `cargo xtask bundle` und `cargo xtask release` schreiben beide nach
`target/KRK.app`, und ein gewöhnlicher Entwicklungsbau löscht die Beglaubigung
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`).
Der Datensatz führt ihn selbst unter `## Dependencies`. Die Runde muss den Defekt nicht mitnehmen,
sie fasst den Weg aber ohnehin an, und wer ihn stehen lässt, baut die Tag-Prüfung an eine Stelle,
deren Ausgabeort gerade strittig ist. Der Aktivierungs-Spec sollte sagen, ob der Defekt in die
Runde gezogen wird oder ausdrücklich draußen bleibt.

**Ein zweites Gegenargument, kleiner.** Die dritte Antwort der Klärungsrunde schwächt die
Begründung ab, mit der der Ideeneintrag beide Hälften aneinanderbindet; der Datensatz schreibt
das in seinem Abschnitt `## Was Antwort 3 an der Kopplungsbegründung ändert` selbst aus. Die
Kopplung bleibt eine Nutzerfestlegung und ist damit nicht die Sache des Playmakers. Sie steht
hier, weil ein Aktivierungs-Spec den Restpunkt aufnehmen sollte, statt ihn ein zweites Mal zu
entdecken.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Closure note

**Geschlossen als kohärenter Abschluss (`_c_`) am 260813-1415.** Die erste Runde dieses Projekts, die nicht beschränkt endet.

**Was den Unterschied macht.** Die sieben Runden davor endeten alle aus demselben Grund beschränkt: der Abnahmelauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit, die kein Agent fahren kann. In dieser Runde hat der Nutzer sie gefahren. Alle elf Beobachtungen mit Bündelanteil sind am laufenden `target/KRK.app` bestanden (`history/260813-1405-abnahmeliste-e2.md`, gefahren am 260813-1410). Damit sind alle 59 Abnahmekriterien des Spec abgenommen bis auf eines: C3.15, der Tag `v0.1.0`, den der Nutzer auf den Abschlusscommit setzt.

**Was die Runde gebaut hat.** Die Titelleiste trägt links einen eigenen Bereich mit `KRK 0.1.0`, gebaut über `NSTitlebarAccessoryViewController` im neuen Modul `crates/krk-ui/src/appkit/titelzusatz.rs`; der absolute Pfad steht weiter mittig und ungekürzt, wie C11 der Runde 2 es zusagt. Dieselbe Zahl steht im Standard-Über-Dialog von macOS, den ein Menüeintrag ohne Kürzel ganz oben im Anwendungsmenü öffnet. Verbindlich wird sie durch semantische Versionstags: `cargo xtask release` bricht als erste Station ab, solange HEAD keinen Tag `v<version>` trägt oder eine verfolgte Datei geändert ist, und ein Abschnitt in `README.md` sagt, wann welche Stufe steigt. Den Tag setzt der Nutzer, nie das Werkzeug. Dazu hat die Runde die Zulässigkeitsregel um eine vierte Bedingung erweitert: steht ein fremdes Fenster vorn, wirkt kein Befehl außer denen auf der Ausnahmeliste.

**Ein Nebenertrag, der nicht in der Directive stand.** Der Defekt der Runde 6 zur Blattregel am Freigabedialog (`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_*_die-blattregel-sieht-den-freigabedialog-nicht.md`) ist mit derselben Beobachtung geschlossen worden — und die Vermutung des Plans dazu ist widerlegt statt bestätigt. Der Wähler entsteht über `showRelativeToRect:` als Verfolgungsschleife, und der Plan schloss daraus, die neue Bedingung erreiche ihn nicht. Am laufenden Bündel kommt der Befehl trotzdem nicht durch.

**Eine Regression, die die Runde selbst erzeugt und in Turn 2 behoben hat.** Die neue Bedingung faltete „ein fremdes Fenster steht vorn" und „gar kein Fenster steht vorn" zu demselben Wert; nach `Shift+Cmd+W` war `Cmd+N` tot und nur noch das Dock-Symbol führte zurück. Die Durchsicht hat das gefunden, `ed0388e` hat es behoben, und Beobachtung 5 der Abnahme bestätigt die Behebung am laufenden Bündel. Die Faltung selbst besteht fort; sie fällt an keinem Befehl mehr auf, den ein Nutzer als Verlust bemerkt.

**Was offen bleibt.** 16 Defekte im Circle. Keiner betrifft das Verhalten der Anwendung; der Schwerpunkt liegt bei Prosa, die dem Code hinterherläuft — Zahlen in Modulköpfen, Aufzählungen, die eine Zahl nennen, die eine andere Datei geändert hat. Zwei wiegen mehr als die übrigen: die Aufruferzahl an `fokus` steht in Plan, Baum und Durchsicht auf fünf und ist sechs, und die Diagrammbefunde an Spec und Plan sind nie behoben worden, obwohl das Sitzungsprotokoll sie als erledigt mitführte. Neun Abnahmekriterien tragen die Kennzeichnung `(Probe)` und haben keine; sie sind am Text nachgelesen, nicht maschinell abgenommen.

**Bilanz.** Zwei Turns, elf Commits, 16 von 16 Planschritten auf `[DONE]`. `make check` exit 0 mit 1025 Proben. Fünf Entscheidungsdatensätze, vier davon umgesetzt; der fünfte wartet auf den Tag. Sitzungsprotokoll: `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md`. Abgleich: `history/260813-1345-reconciliation.md`.

**Der eine offene Schritt für den Nutzer.** `git tag v0.1.0 <abschlusscommit>` — danach ist auch C3.15 erfüllt und der grüne Fall der neuen Prüfung an einem echten Lauf zu sehen.
