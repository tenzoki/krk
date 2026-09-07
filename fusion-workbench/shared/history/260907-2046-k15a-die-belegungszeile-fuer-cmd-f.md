# K15a: die Belegungszeile für cmd+f, und vier nachgezogene Prosastellen derselben Datei

**Status:** Complete
**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Die neue Funktion `filter_einfuegen` in `resources/default-keymap.toml` eintragen,
mit `gehalten_von = "menue"`, und alles nachziehen, was in derselben Datei daran
hängt. Der Codeanteil kommt in einem zweiten Durchgang; beide Hälften gehen in
einen Commit. Grundlage: der Nutzerentscheid vom 260907-2009
(`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`)
und der Befund
`260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`.

## Was geändert ist

Alles in `resources/default-keymap.toml`, keine Datei unter `crates/` angefasst.

1. **Die neue Funktion**, hinter `inhaltssuche_umschalten` im Block des
   Dateilistings, mit `gehalten_von = "menue"` und einem Kommentar darunter nach
   dem Muster der cmd+a-Stelle. Der Kommentar sagt: dass cmd+f auch bei
   `editor_suchen` steht und das kein Konflikt ist, weil dort der Ereignisabgriff
   zustellt und hier das Menü; dass der **Zusteller** das Paar trägt und nicht der
   Wirkungsbereich, weil zwei vom Abgriff zugestellte Funktionen mit verschiedenem
   Wirkungsbereich ein Konflikt blieben und die eingebettete Belegung beim ersten
   Zugriff abbräche; dass cmd+a und cmd+f den Anschlag an **verschiedenen**
   Bestandteilen der Zulässigkeitsregel trennen, cmd+a am Fokusvorbehalt und cmd+f
   am Wirkungsbereich, weshalb cmd+a kein Beleg für cmd+f ist; und woher die
   Funktion kommt, samt Verweis auf den Entscheid vom 260805, den vom 260907-2009
   und die offene Frage von heute.

   Die erste Fassung des Kommentars schrieb die Trennung dem Fokusvorbehalt zu,
   also genau das, was der abgelegte Datensatz als falsch benennt. Sie ist vor der
   Abnahme berichtigt worden.
2. **Die zwei Zahlen im Dateikopf**, 92 auf 93 Funktionen und 95 auf 96
   Kombinationen. Die Probe
   `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` liest beide
   zurück und bleibt grün.
3. **Der Absatz zur Zustellerregel im Kopf.** Er sagte, ausgeliefert gebe es „genau
   einen Fall der Doppelung". Jetzt nennt er beide Fälle beim Namen, cmd+a und
   cmd+f, und trägt keine Zahl mehr; die Begründung steht dabei.
4. **Der C10-Absatz im Kopf zu Cmd+C und Cmd+V.** Er sagte, die Reservierung aus C3
   sei „ganz eingeloest". Jetzt trennt er die beiden: Cmd+C ist am Dateifenster
   besetzt, Cmd+V war es von der Runde 21 bis zum 260907 und ist seither für die
   Dateizwischenablage wieder offen.
5. **Der Blockkommentar bei C10** und **der Blockkommentar der Textbefehle**. Beide
   nannten `paste:` als das Einfügen in den Filtertext seit der Runde 21; beide
   sagen jetzt, dass der Delegierte es seit dem 260907 nicht mehr beantwortet, dass
   der Einhängepunkt wieder frei ist und dass die Funktion auf cmd+f mit eigener
   Zeile steht.

Unangetastet: der Eintrag `text_einfuegen` mit `cmd+v` und `gehalten_von = "menue"`.
Der Befund vom 260907-2020 ist darin richtig — dass `cmd+v` aufhört, den Filter zu
füllen, entscheidet der Code über den Wegfall der `paste:`-Antwort und nicht diese
Datei; die Kombination bleibt für das Einfügen im Textfeld zuständig.

## Was geprüft ist

- `cargo test -p krk-core` — exit 0, 0 Fehlschläge. Darin
  `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` (die zwei
  Kopfzahlen gegen die Datei), `die_auslieferungsbelegung_ist_konfliktfrei` und
  `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren`. Damit ist
  zugleich belegt, dass die Datei gültiges TOML ist: alle drei deserialisieren den
  eingebetteten `AUSLIEFERUNGSTEXT`. Ein TOML-Werkzeug steht auf diesem Gerät nicht
  zur Verfügung, weder `tomllib` (Python 3.9), `toml`, `yq` noch `taplo`.
- `cargo fmt --all --check` — exit 0.
- `cargo test -p krk-ui` — exit 101, 52 Fehlschläge, **erwartet und nicht
  behoben**. Jeder einzelne Fehlschlag nennt `filter_einfuegen`, an drei Stellen:
  `belegungsmodell.rs:900` („die Funktion filter_einfuegen hat keinen
  Funktionsbereich", 50 Fehlschläge), `belegungsmodell.rs:1032`
  (`jede_kennung_hat_einen_funktionsbereich`) und `belegungsausgabe.rs:845`
  („filter_einfuegen ist zugestellt, aber wirkung() kennt sie nicht"). Keine andere
  Ursache im Lauf. Das ist genau der rote Stand, den der Befund vom 260907-2020
  vorhergesagt hat, und der Codeanteil zieht ihn nach.
- `make tasten` und `make menue` sind **nicht** gefahren: sie verlangen ein
  gebautes, signiertes Bündel und einen Mitschnitt und sind Nutzerarbeit.

## Was der Codeanteil aus dieser Aufgabe mitnehmen muss

Zwei Prosastellen in `crates/` sagen weiter, ausgeliefert gebe es genau einen Fall
der Doppelung: `crates/krk-ui/src/menuemodell.rs:317` und der Kommentar in
`crates/krk-core/tests/belegung.rs:472`. Sie liegen außerhalb des Auftragsrahmens
dieses Durchgangs.

## Abgelegte Datensätze

- `260907-2046_*_traegt-das-cmd-f-paar-auf-demselben-grund-wie-cmd-a-oder-auf-einem-dritten.md`
  (Nutzerfrage). Die Datei lehnt ein drittes Paar dieser Bauart bei
  `eintragspfad_kopieren` ab, „solange nicht gemessen ist, ob der Fokusvorbehalt
  dort so trennt wie bei Cmd+A". Für cmd+f ist das nicht gemessen, und es trennt
  auch nicht am Fokusvorbehalt: gelesen in `kommandos/zulaessigkeit.rs`, Modulkopf
  `# Die vier Bestandteile`, weist mit dem Fokus im Dateifenster Bestandteil (3),
  `fokus::wirkt`, das `editor_suchen` ab und nicht Bestandteil (2). Bei cmd+a
  trennt (2). Zwei Paare, zwei Bestandteile.
- `260907-2046_*_claude-md-und-howto-md-nennen-cmd-v-als-den-weg-in-den-filtertext-seit-dem-260907-traegt-ihn-cmd-f.md`
  (Defekt). Der `coder` hatte ihn am 260907-2020 bewusst offengelassen, weil beide
  Stellen bis zur Änderung zutrafen.
- `Also seen:`-Zeilen an
  `260907-2026_*_die-antwortzeile-zu-cmd-f-begruendet-das-paar-mit-dem-wirkungsbereich-die-regel-vom-260805-nennt-den-zusteller.md`
  und an
  `260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`.
  Die zweite trifft `filter_einfuegen` in verschärfter Form: kommt die Funktion bei
  einem Nutzer mit eigener `keymap.toml` unbelegt an, tut cmd+f dort nichts,
  während cmd+v nach demselben Durchgang schon nichts mehr tut.

Nicht committet; der Orchestrator legt beide Hälften in einen Commit.
