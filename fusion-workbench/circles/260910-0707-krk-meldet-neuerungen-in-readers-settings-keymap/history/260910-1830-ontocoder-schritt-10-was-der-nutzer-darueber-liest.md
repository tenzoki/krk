# Ontocoder-Sitzung: Schritt 10, was der Nutzer darüber liest

**Date:** 2026-09-10, 260910-1830
**Filed by:** ontocoder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 10
**HEAD:** `aba36d6` (nicht committet; der Nutzer committet)

## Was getan wurde

Zwei Nutzerdateien, `README.md` und `HowTo.md`. Kein Code, keine `.toml`.

### `README.md`

Der Abschnitt `## Neue Leseprofile übernehmen` heißt jetzt
`## Neuerungen an den eigenen Dateien übernehmen` und gilt für alle drei von Hand
gepflegten Ablagedateien statt allein für `readers.toml`. Er trägt:

- die drei Dateien und den Grund, aus dem eine Neuerung nicht von selbst ankommt,
- die Startzeile: je betroffener Datei die Zahl der neuen Einträge, dahinter der Ordner,
  nur Dateien mit einem Unterschied, **einmal je Fassung**,
- den Befehl „Neuerungen anzeigen" mit Menügruppe „Anwendung" **und** `opt+cmd+i`, dazu
  was das Blatt zeigt (voller Pfad, beide Richtungen, der Preis je Datei),
- den lebenden Fall als Beispiel: wer eine `keymap.toml` von vor dieser Fassung hat, findet
  `neuerungen_zeigen` bei sich ohne Kürzel am Fuß der Gruppe „Anwendung",
- die drei verschiedenen Preise (`readers.toml` alles, `settings.toml` nur den
  Kommentarblock, `keymap.toml` die Kombinationen),
- den vorhandenen Handgriff, auf „die betroffene Datei" verallgemeinert, und den Absatz
  „Beiseitelegen und nicht löschen" unverändert in seiner Aussage.

Der Handgriff steht weiter an **einer** Stelle. `HowTo.md` verweist darauf und schreibt ihn
nicht zum zweiten Mal aus.

### `HowTo.md`

Im Abschnitt `## Wo KRK seine eigenen Dateien ablegt` ersetzt der Absatz über die drei
Dateien den alten „Für die Leseprofile hat es einen Preis". Er trägt dieselbe Auskunft in
der knapperen Form der Anleitung, mit einer Tabelle für die drei Preise, und verweist für
den Handgriff auf den neuen Abschnittsnamen der `README.md`. Der Rückverweis am Ende von
`## Leseprofile` ist auf denselben Namen nachgezogen und nennt zusätzlich die Startmeldung
und den Befehl.

## Abnahme des Schrittes

- **Beide Stellen nennen den Befehl mit seinem Menüeintrag und nicht nur mit der
  Tastenkombination.** „im Hauptmenü unter „Anwendung" und ab Werk auf `opt+cmd+i`", in
  beiden Dateien, und in beiden steht dahinter, dass eine alte `keymap.toml` das Kürzel
  nicht hat.
- **Die Betriebsregel steht unverändert an jeder Stelle, an der sie heute steht.** Die
  Erhebung ``grep -rnE --exclude-dir=fusion-workbench --exclude-dir=target
  '[Dd]ie alte.{0,24}löschen' .`` liefert vor und nach dem Schritt dieselben vier Zeilen:
  `README.md:35`, `CLAUDE.md:133`, `HowTo.md:15`, `xtask/src/veroeffentlichung.rs:760`.
  Keine der vier ist angefasst.
- **Keine neue Zahl im Text, die der Baum schon trägt.** Die Startzeile ist nach ihrer Form
  beschrieben und nicht mit einem erfundenen Zahlenbeispiel; die Zahl der Leseprofile, die
  Zahl der Funktionen und die Zahl der Ablagedateien kommen in keinem neuen Satz vor.

## Was daneben liegt und nicht angefasst ist

- **`CLAUDE.md` verweist auf den alten Abschnittsnamen.** Zeile 135 nennt „im Einzelnen
  steht er in `README.md` unter `## Neue Leseprofile übernehmen`"; der Abschnitt heißt jetzt
  anders. Der Plan hält CLAUDE.md ausdrücklich aus dieser Runde heraus
  (`## Where this Circle stops`), der Abgleich gehört `/fusion:cleanup --only claude-md` am
  Ende der Sitzung. Dort ist der Verweis mitzuziehen.
- **`RELEASETEXT` (`xtask/src/veroeffentlichung.rs`) ist nicht angefasst und muss es nach
  hiesigem Urteil auch nicht.** Er sagt, was beim Installieren zu tun ist, und die Neuerung
  betrifft den ersten Start danach. Eine Aussage über den Befehl dort machte den festen Text
  länger, ohne die Installation zu ändern, und jede seiner Aussagen hängt an einer eigenen
  Behauptung der Probe `der_releasetext_traegt_jede_seiner_aussagen`.
