# Coder — Schritt 4: Der Befehl wirkt: einblenden, vergleichen, stellen

**Datum:** 260818-2103
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 4
**Baumstand beim Beginn:** `18af77f`, Arbeitsbaum sauber bis auf das Ereignisprotokoll

## Was der Auftrag war

Den Ausführungszweig bauen, den kein Übersetzer einfordert. Das Kommando
`OrdnerAngleichen` stand seit `18af77f` im Hauptmenü und in der
Belegungsansicht und tat nichts, weil `kommando_ausfuehren` auf einem
Auffangzweig endet und ein fehlender Zweig dort stillschweigend hindurchfällt.

## Was geändert wurde

**`crates/krk-ui/src/appkit/anwendung.rs`**

- Zweig `Kommando::OrdnerAngleichen => self.ordner_angleichen(),` in
  `kommando_ausfuehren`, unmittelbar hinter `Kommando::OrdnerDerDatei` und
  damit vor dem Auffangzweig. Der Kommentar an der Zeile sagt aus, warum der
  Zweig nötig ist, obwohl der Befehl `Wirkungsbereich::Dateifenster` trägt und
  scheinbar dem Auffangzweig zustünde: das Ziel ist das **andere**
  Dateifenster, und an das kommt ein einzelnes nicht heran.
- Neuer Rumpf `ordner_angleichen(&self) -> bool`, in der vom Plan gesetzten
  Reihenfolge: `aktiv` lesen, `ziel = aktiv.andere()`, beide angezeigten
  Ordner holen; bei Gleichheit die Meldung „das andere Dateifenster zeigt
  diesen Ordner bereits" an das auslösende Dateifenster und `true` zurück,
  ohne zu lesen und ohne einzublenden; sonst `Bereich::von_seite(ziel)`, die
  Sichtbarkeit am Fenstermodell fragen, nur bei Verneinung
  `bereich_einblenden` rufen und bei `false` mit „das Fenster ist zu schmal;
  es wurde nichts eingeblendet und nichts gestellt" abbrechen; danach
  `ordner_lesen`. Der Fokus wird nicht angefasst, `aktiv_setzen` nicht
  gerufen, kein Bereich ausgeblendet.
- Der Doc-Kommentar trägt die vier vom Plan verlangten Aussagen: warum die
  Sichtbarkeit **vor** dem Einblenden gefragt wird, warum ohne `canonicalize`
  verglichen wird, warum die Meldung an das auslösende Dateifenster geht, und
  die Folge der Spec-Reihenfolge („steht schon dort" vor „ist sichtbar").
- `diese_datei` und `rumpf` in `mod zettelproben` sind `pub(super)` geworden,
  damit das neue Prüfmodul denselben Quelltext mit derselben Rumpfregel liest
  statt eine zweite Fassung daneben zu setzen. Beide tragen den Grund an ihrem
  Doc-Kommentar.
- Neues Prüfmodul `angleichproben` mit drei Proben am Quellbaum.

**`crates/krk-ui/src/appkit/tabelle.rs`**

- Der Doc-Kommentar von `ordner_lesen` nennt den neuen Rufer und hält fest,
  dass die Zusage der Methode unverändert trägt: sie wechselt den Ordner des
  sichtbaren Tabs und legt keinen neuen an, gleich von welcher Seite aus
  gerufen.

## Eine Ausleihe, die den Absturz gekostet hätte

`bereich_einblenden` nimmt sich über `sichtbarkeit_aendern` denselben
`RefCell` veränderlich. Die naheliegende Kurzform

```rust
if !self.ivars().modell.borrow().sichtbar(bereich) && !self.bereich_einblenden(bereich)
```

lebt in der Ausleihe der linken Hälfte, während die rechte läuft: Temporäre
einer `if`-Bedingung fallen erst am Ende der ganzen Bedingung. Der Griff nach
`borrow_mut()` wäre der Absturz gewesen. Die Sichtbarkeit steht deshalb in
einem eigenen `let`, wie an `anwendung.rs:2063` schon; der Grund steht als
Kommentar an der Zeile.

## Was von der Vorgabe des Plans abweicht

**Der Plan sagt, der Doc-Kommentar von `ordner_lesen` zähle seine Rufer auf und
bekomme „seinen vierten" dazu. Beides trifft am Baum nicht zu.** Der
Doc-Kommentar zählte keine Rufer, sondern nannte die zwei Verwendungen von
`auswahl`, und `ordner_lesen` hat zehn Aufrufstellen:
`tabelle.rs:752`, `:1558`, `:1590`, `:1655` und `anwendung.rs:1600`, `:2597`,
`:3237`, `:6677`, `:6756`, `:6761`.

**Auch „der erste mit einer anderen Seite als `aktiv`" trifft nicht wörtlich
zu.** `anwendung.rs:2597` und `:6677` nennen `Fensterseite::Links` fest,
`:6761` nennt `Fensterseite::Rechts`; alle drei gehören dem Messmodus
(`messhandlung`, `messen_weiter`) und sind keine Befehle. Der geschriebene Satz
sagt deshalb, was stimmt: unter den **Befehlen** ist das Angleichen der erste,
der eine Seite nennt, und der Messmodus kannte das schon, ohne ein Befehl zu
sein. Eine Zahl steht nicht darin, weil sie in diesem Baum die Sorte Zahl ist,
die veraltet.

## Die drei Proben, und was sie nicht sehen

`make check` hält jetzt drei Aussagen über den Baum, die kein Übersetzer hält.
Keine von ihnen baut ein Fenster; die Kriterien aus C1 und C2, die eines
brauchen, stehen unverändert als Nutzerarbeit im Plan und werden hier nicht
behauptet.

- `der_befehl_steht_vor_dem_auffangzweig` — der Ausführungszweig steht im Rumpf
  von `kommando_ausfuehren` und vor `andere => self.bereichskommando`. **Am
  Baum gegengeprüft:** die Zeile testweise entfernt, die Probe wurde rot, die
  Zeile wiederhergestellt.
- `die_sichtbarkeit_wird_vor_dem_einblenden_gefragt` — `sichtbar(bereich)`
  steht vor `bereich_einblenden(bereich)`.
- `das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an` — der Rumpf enthält
  weder `aktiv_setzen(` noch `bereich_umschalten(` noch `ausblenden(`.

Alle drei lesen den Rumpf und nicht den Aufrufbaum darunter; eine Wirkung, die
in eine später gerufene Hilfsfunktion wandert, sehen sie nicht. Der
Modulkommentar sagt es.

## Abnahme

`make check` — Beendigungsstatus `0`. Alle vier Kommandos grün: Bau, Proben,
Formatierung, Clippy unter `-D warnings`. Die drei neuen Proben laufen mit.

Vor dem Lauf geprüft, dass kein Messlauf steht: weder `/tmp` noch `$TMPDIR`
führt eine `krk-messplan-*.toml`.

## Was offen bleibt

- Nicht festgeschrieben. Der Nutzer schreibt fest.
- Die Schritte 1 und 3 sind im Baum (`18af77f`), tragen im Plan aber keine
  `[DONE]`-Marke. Nur Schritt 4 ist von hier aus gesetzt.
- Schritt 5, die acht Prosazahlen, ist nicht Gegenstand dieses Auftrags und
  offen.
