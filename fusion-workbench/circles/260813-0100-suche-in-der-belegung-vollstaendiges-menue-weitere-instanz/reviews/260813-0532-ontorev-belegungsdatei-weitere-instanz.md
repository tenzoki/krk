# Durchsicht der Belegungsdatei: `weitere_instanz` auf `opt+cmd+n`

**Date:** 2026-08-13
**Sender:** ontorev
**Reviewed-range:** `ca66c39..40b5fb0`
**Not-opened:** `crates/krk-core/src/ablage/einstellungen.rs`, `crates/krk-core/src/ablage/lesezeichen.rs`, `crates/krk-core/src/ablage/mod.rs`, `crates/krk-core/src/ablage/sitzung.rs`, `crates/krk-core/src/ablage/sperre.rs`, `crates/krk-core/src/lib.rs`, `crates/krk-core/src/text/suche.rs`, `crates/krk-core/src/verzeichnis/mod.rs`, `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/baum.rs`, `crates/krk-core/tests/gemeinsam/mod.rs`, `crates/krk-ui/src/appkit/belegungsansicht.rs`, `crates/krk-ui/src/appkit/leiste.rs`, `crates/krk-ui/src/appkit/menue.rs`, `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/appkit/teilen.rs`, `crates/krk-ui/src/appkit/weitereinstanz.rs`, `crates/krk-ui/src/kommandos/mod.rs`, `crates/krk-ui/src/leistenmodell.rs`, `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/messmodus.rs`, `crates/krk-ui/src/quellbaum.rs`, `fusion-workbench/circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/decisions/260813-0320_o_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`, `.../decisions/260813-0430_o_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`, `.../history/260813-0325-coder-s1-bis-s3-zulaessigkeit.md`, `.../history/260813-0426-coder-s7-bis-s10-suche-in-der-belegung.md`, `.../history/260813-0445-coder-s4-bis-s6-vollstaendiges-menue.md`, `.../history/260813-0620-coder-s11-bis-s14-weitere-instanz.md`, `.../issues/260813-0311_o_ein-klick-in-die-bereichsleiste-wirkt-seit-s2-waehrend-einer-umbenennung-nicht-mehr.md`

**Gegenstand:** `resources/default-keymap.toml`, Commit `40b5fb0` (Schritt S15). Der
Rust-Anteil des Bereichs liegt beim `coderev`. Sechs Quelldateien sind **teilweise** gelesen,
nämlich in den Abschnitten, die die neue Kennung oder die Prosa der Belegungsdatei berühren,
und stehen deshalb nicht unter „Not-opened": `crates/krk-core/src/tasten/belegung.rs`,
`crates/krk-core/tests/belegung.rs`, `crates/krk-ui/src/belegungsmodell.rs`,
`crates/krk-ui/src/menuemodell.rs`, `crates/krk-ui/src/appkit/ereignisse.rs`,
`crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/kommandos/zulaessigkeit.rs`.

---

## Zusammenfassung

**Der neue Block ist mechanisch einwandfrei.** `opt+cmd+n` war frei, die Schreibweise folgt
der festgelegten Reihenfolge, die zwei Zahlen im Dateikopf sind selbst nachgezählt und
stimmen, `reserviert_fuer` steht nicht dabei, die Kennung ist in allen drei vollständigen
Fallunterscheidungen des Codes angekommen, und keine Kennung steht ohne Gegenstück da. Der
Ausführende hat den Plan nicht abgeschrieben, sondern nachgezählt, und seine drei Abweichungen
vom Plantext sind einzeln belegt und richtig.

**Was nicht hält, ist wieder die Prosa, und diesmal nicht die neue.** Der Kopf der Datei
beschreibt den Fokusvorbehalt als frühen Ausstieg vor dem Nachschlag. Genau diesen Ausstieg
hat dieselbe Prüfspanne in `9da33bc` entfernt, und der Code vermerkt es an der Stelle, an der
er stand. Dazu ist der Datei seit S6 eine Zusicherung zugewachsen, die sie nicht nennt: die
Reihenfolge ihrer Blöcke bestimmt jetzt die Reihenfolge der Menüleiste.

**Zwei neue Befunde, beide Prosa, keiner am Laufzeitverhalten.** Zwei weitere Punkte gehören in
schon offene Datensätze und sind hier nur eingetragen, nicht ein zweites Mal gemeldet.

## Zahlen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 0 |

Dazu zwei Zuträge zu offenen Datensätzen und eine Beobachtung ohne Befundcharakter.

---

## Die sieben Prüfpunkte des Auftrags, nachgeprüft

| Punkt | Befund |
|---|---|
| War `opt+cmd+n` frei? | **Ja.** Über alle 82 `tasten`-Listen selbst ausgezählt; `opt+cmd+n` kommt genau einmal vor, im neuen Block. Am Stand `ca66c39` kommt es null Mal vor. |
| Folgt die Schreibweise? | **Ja.** `[ctrl+][opt+][shift+][cmd+]<taste>`; `opt+cmd+n` steht in dieser Ordnung, `n` gehört zu den zugelassenen Tastennamen. |
| Stimmen die zwei Zahlen? | **Ja.** 82 `[[funktion]]`-Blöcke, 88 Einträge über alle `tasten`-Listen. Vorher 81 und 87, am Stand `ca66c39` nachgezählt. `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` läuft grün. |
| Der Ort des Blocks | **Beide Behauptungen des Ausführenden stimmen; der Ort trägt.** Im Einzelnen unten. |
| Ist der Kommentar wahr? | **Ja, in jeder prüfbaren Aussage.** Der zitierte Datensatz besteht und trägt den Satz wörtlich. Im Einzelnen unten. |
| `reserviert_fuer` | **Steht nicht dabei, richtig so.** Die Funktion trägt seit `3caa2b7` ein Kommando und eine Taste. |
| Deckung gegen den Code | **Vollständig, in beide Richtungen.** Im Einzelnen unten. |

### Der Ort des Blocks

Der Ausführende berichtet zweierlei, und beides ist am Baum nachgeprüft.

**„Den von S15 genannten Zielabschnitt gibt es nicht."** Stimmt. S15 verlangt „den Abschnitt zu
C3, in dem `belegung_ansehen` und `beenden` stehen"
(`planning/260813-0205_o_plan-…:397`). Die Datei führt die beiden in **zwei** Abschnitten,
`# ── C3: die Belegungsansicht ──` (`:767`) und `# ── C3: das Beenden der Anwendung ──`
(`:903`), und dazwischen liegen die sechs Textbefehle des Menüs „Bearbeiten". Ein Abschnitt,
der beide führte, besteht nicht. (Genau genommen nennen **drei** Abschnittsköpfe C3; der
dritte ist `# ── C3 und C4: die Norton-Reihe ──` (`:104`) und führt keinen der beiden. Das
ändert am Befund nichts.)

**„S4 hat die Datei nicht angefasst."** Stimmt. `git log -- resources/default-keymap.toml`
nennt vor `40b5fb0` als letzten Commit `95b2dfa` aus der Runde 6.

**Der gewählte Ort trägt, und er trägt aus einem Grund, den der Kommentar nicht nennt.** Der
Kommentar begründet ihn mit der Sachgruppe und beruft sich auf `bereich_des_kommandos`
(`crates/krk-ui/src/belegungsmodell.rs:327`), das `WeitereInstanz` mit `BelegungAnsehen` und
`Beenden` unter `Funktionsbereich::Anwendung` führt. Das ist richtig zitiert, trägt die
Ortswahl aber nur halb: `belegung_ansehen` steht in derselben Gruppe und liegt trotzdem über
hundert Zeilen entfernt. Die Datei ordnet nach Fähigkeit und nicht nach Funktionsbereich, und
aus der Gruppe folgt deshalb keine Nachbarschaft.

Was den Ort trägt, ist etwas anderes: seit S6 bestimmt die Blockreihenfolge die Reihenfolge im
Obermenü (Befund 2 unten). Der gewählte Ort ergibt „Tastaturbelegung anzeigen", „Weitere
Instanz starten", „Belegung als Markdown …", Trenner, „KRK beenden" — eine brauchbare
Menüfolge. Das ist ein gutes Ergebnis und kein geprüftes: weder der Plan noch der Kommentar
noch der Verlaufseintrag erwähnen die Wirkung.

### Ist der Kommentar wahr?

Vier Aussagen sind prüfbar, und alle vier halten.

1. **„keine davon nennt opt+cmd+n"** (`:891`) — nachgezählt, stimmt.
2. **„shift+cmd+n legt einen Ordner an, ctrl+cmd+n eine leere Datei"** — `:128`
   (`ordner_anlegen`, `["f7", "shift+cmd+n"]`) und `:370` (`datei_anlegen`). Stimmt.
3. **„das nackte cmd+n haelt bei `fenster_einblenden` den Platz des Mac-ueblichen ‚Neu'"** —
   stimmt in der Sache. Der Eintrag selbst sagt „Darum ‚Fenster einblenden' und nicht ‚Neues
   Fenster'" (`:492-493`), und der Datensatz sagt, dass die Runde mit mehreren Fenstern ihn
   umbenennt und das Kürzel behält. Kein Widerspruch: heute heißt er nicht „Neu", das Kürzel
   ist dafür vorgemerkt.
4. **Der zitierte Datensatz besteht und trägt den Satz wörtlich.**
   `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2007_i_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
   liegt auf der Platte; die Nutzerantwort steht unter `## Antwort des Nutzers, 260804-0830`
   (`:87`), und die zitierte Zusage steht in `:124-125`: „einführt, benennt ihn in ‚Neues
   Fenster' um, beantwortet dabei die beiden Folgefragen und behält das Kürzel."

**Der Kommentar behauptet keinen Entscheidungsdatensatz für `opt+cmd+n` selbst**, und das ist
richtig so: einen gibt es nicht. Der Spec leitet die Kombination ab
(`shared/planning/260813-0053_o_spec-…:317`, „Am 260813 als einzige naheliegende freie
Kombination … abgelesen"). Der Kommentar zitiert ausschließlich den Datensatz zu `cmd+n`, und
der trägt, was er tragen soll. **Der Fehler der Runde 6 wiederholt sich hier nicht.**

Eine Formulierung ist enger als die Sache: „`opt+cmd+n` ist die **vierte Form** dieser Reihe"
setzt eine viergliedrige Reihe voraus. `opt+shift+cmd+n` und `ctrl+opt+cmd+n` sind ebenfalls
frei. Das ist Wortwahl und kein Befund.

### Deckung gegen den Code

In beide Richtungen ausgezählt, nicht nur die neue Kennung nachgesehen:

- 82 `id`-Werte in der Datei, keiner doppelt.
- `Kommando::KENNUNGEN` führt 76 Kennungen (`crates/krk-core/src/tasten/belegung.rs:566-671`).
- **Jede der 76 hat einen Block in der Datei.** Keine Kennung steht ohne Gegenstück.
- **Die 6 Blöcke ohne Kennung sind genau die sechs zugestellten Textbefehle** mit
  `gehalten_von = "menue"`. 82 − 6 = 76. `Funktion::kommando` (`crates/krk-core/src/tasten/belegung.rs:987`)
  liefert für sie `None`; keine von ihnen bekommt je ein Kommando.
- Die neue Kennung steht an allen drei Pflichtstellen: `Kommando::WeitereInstanz`
  (`belegung.rs:560`), `KENNUNGEN` (`:663`), `wirkungsbereich` mit `Ueberall` (`:760`),
  `bereich_des_kommandos` mit `Anwendung` (`belegungsmodell.rs:327`).

Nachgefahren, ohne Vordergrundlauf und ohne Bündelbau:

| Kommando | Ergebnis |
|---|---|
| `cargo test -p krk-core --lib tasten::belegung` | 4 Proben, 0 Fehlschläge |
| `cargo test -p krk-core --test belegung` | 45 Proben, 0 Fehlschläge |

---

## Befund 1 (Medium): der Dateikopf beschreibt den Fokusvorbehalt als frühen Ausstieg vor dem Nachschlag

Datensatz: `issues/260813-0532_o_der-dateikopf-der-belegung-beschreibt-den-fokusvorbehalt-als-fruehen-ausstieg-vor-dem-nachschlag.md`

`resources/default-keymap.toml:76-80` sagt, der Ereignisabgriff reiche einen Tastendruck bei
Schreibmarke im Textfeld unverändert an AppKit weiter und schlage nur im Dateifenster in
dieser Datei nach, und der Vorbehalt frage **vor** dem Nachschlag.

Am Stand `ca66c39` stand das so im Code, wörtlich:

```
// Der Fokusvorbehalt, vor dem Nachschlag. […]
if ersthelfer_gehoert_appkit(mtm, ist_editorflaeche) {
    return false;
}
```

Am Stand `40b5fb0` steht an derselben Stelle:

```
// Hier stand bis zur Runde 7 der Fokusvorbehalt als frueher Ausstieg.
// […] Der Abgriff reicht beide Ausgaenge des Nachschlags
// unveraendert weiter und fragt nicht mehr nach dem Ersthelfer.
let nachschlag = belegung.nachschlag(druck);
```

(`crates/krk-ui/src/appkit/ereignisse.rs:517-522`.) Beide Sätze des Dateikopfs sind damit
widerlegt. Die **Folgerung** des zweiten hält weiter, ihre Begründung nicht: eine nackte Taste
wird nach wie vor behandelt wie eine mit Zusatztaste, aber weil zwei Stellen hinter dem
Nachschlag denselben Wert lesen — `zulaessig` im Kommandozweig, die eigene Abfrage im
Sprungmarkenzweig (`crates/krk-ui/src/appkit/anwendung.rs:2491-2494`).

**Warum das an dieser Datei zählt und nicht am Code:** die Belegungsdatei ist nach ihrem
eigenen ersten Satz die alleinige Quelle, und ihr Kopf ist das erste, was eine Runde liest,
bevor sie eine Kombination vergibt. Der Modulkopf von `ereignisse.rs` trägt die neue Lage
vollständig; die Datei, die den Leser zuerst erreicht, trägt die alte. Keine Probe fängt es
ab: geprüft wird von diesem Kopf allein die Zählzeile.

## Befund 2 (Medium): die Blockreihenfolge steuert seit S6 die Menüleiste, und der Dateikopf sagt es nicht

Datensatz: `issues/260813-0534_o_die-blockreihenfolge-der-belegungsdatei-steuert-seit-s6-die-menueleiste-und-der-dateikopf-sagt-es-nicht.md`

`belegungsmodell::nach_bereichen` gibt die Funktionen einer Gruppe in Dateireihenfolge zurück
(`crates/krk-ui/src/belegungsmodell.rs:806-818`). Bis zur Runde 7 hingen daran die
Belegungsansicht und die Markdown-Ausgabe; seit S6 hängt die Menüleiste mit daran
(`crates/krk-ui/src/menuemodell.rs:204-224`). Der Modulkopf dort sagt es und zieht die
Folgerung, die Behebung gehöre in die Belegungsdatei (`:76-82`). **Der Kopf der
Belegungsdatei nennt die Zusicherung nicht.** Er erklärt jedes Feld eines Eintrags und die
Ein-Zeilen-Regel; über die Reihenfolge der Blöcke steht dort nichts.

Drei Folgen: der offene Befund `260813-0420` schlägt genau eine solche Verschiebung vor und
findet in der Datei keinen Satz, der sie erklärt; der neue Block aus S15 ist ohne diesen
Prüfschritt eingeordnet worden und hat die Menüfolge verändert, ohne dass es jemand geprüft
hätte; und keine Probe hält die Reihenfolge fest.

---

## Zuträge zu offenen Datensätzen — hier eingetragen, nicht ein zweites Mal gemeldet

### `cmd+a`: die Beschreibung in der Datei stimmt im Ergebnis und nicht mehr in ihrer Begründung

Gehört zu `issues/260813-0416_o_zwei-menueeintraege-mit-cmd-a-…`, das
`resources/default-keymap.toml` bereits unter „Betroffen" führt. **Kein zweiter Datensatz.**

`resources/default-keymap.toml:843-848` sagt:

> cmd+a steht auch bei „alle_markieren" weiter oben, und das ist kein Konflikt. Dort stellt
> der Ereignisabgriff zu, hier das Menue, und der Fokusvorbehalt laesst die beiden einander
> nie begegnen.

**Das Urteil hält.** Die Zusteller sind weiterhin verschieden (`alle_markieren` ohne
`gehalten_von`, `text_alles_auswaehlen` mit `gehalten_von = "menue"`), und
`die_auslieferungsbelegung_ist_konfliktfrei` sowie
`cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt` laufen grün.

**Die Begründung reicht seit `16c0924` nicht mehr aus.** Der Datensatz `260813-0416` misst es
selbst: mit dem vollständigen Menü standen beide Funktionen in **einer** Leiste, dort
entscheidet nicht der Fokusvorbehalt, sondern AppKit nach der Stellung, und „Alles auswählen"
verlor sein Kürzel. Cmd+A wäre in jedem Textfeld ausgefallen. Was den Fall heute
zusammenhält, ist eine **zweite** Absprache, `menuemodell::zugestellte_kuerzel`, und die steht
in keinem Wort dieser Datei — weder im Kopf (`:88-99`) noch am `cmd+a`-Kommentar (`:843-848`)
noch am Eintrag `alle_markieren` (`:272-275`), dem sie das angezeigte Kürzel nimmt.

Der Datensatz schreibt „In der Belegungsdatei stimmt der Satz weiterhin". **Dem widerspreche
ich zur Hälfte:** die Aussage „kein Konflikt" stimmt, die Erklärung „der Fokusvorbehalt lässt
die beiden einander nie begegnen" ist als vollständige Erklärung nicht mehr richtig. Wer sie
für vollständig hält, könnte `alle_markieren` eine zweite Kombination geben oder den Block
verschieben und dabei nicht bemerken, dass er an einer zweiten Absprache rührt.

**Empfehlung:** die Behebung von `260813-0416` um zwei Sätze in dieser Datei erweitern — einen
am `cmd+a`-Kommentar, der die zweite Absprache nennt, und einen am Eintrag `alle_markieren`,
der sagt, dass sein Menüeintrag das Kürzel nicht anzeigt, obwohl es ihn auslöst.

### `opt+cmd`: der neue Block ist der dritte Gegenbeleg zu einer Reihenordnung, die die Datei behauptet

Gehört zu `circles/260812-1000-…/issues/260812-1527_o_die-zwei-neuen-kommentare-verengen-die-reihenordnung-…`.
**Kein zweiter Datensatz** — aber die dort empfohlene Berichtigung greift zu kurz.

Die Datei behauptet über die `opt+cmd`-Reihe zweierlei: sie trage, „was einen Ordner
herstellt oder liefert" (`:246-249`), und „Die Umschaltfamilie steht auf
opt+cmd+<Buchstabe>" (`:689-690`). `weitere_instanz` ist keins von beidem. Es ist nach
`opt+cmd+delete` (`:133`) und `opt+cmd+e` (`:688`) der **dritte** Gegenbeleg zum Satz in
`:246-249`, und er ist neu in dieser Prüfspanne.

Der neue Kommentar geht der Frage nicht nach, sondern wechselt die Achse: er begründet über
den Grundbuchstaben („Das `n` ist in dieser Datei die Taste des Neuen"), nicht über die
Zusatztastenreihe. Die Begründung ist für sich schlüssig und nachgeprüft. Sie lässt die Datei
aber mit **drei** Lesarten ihrer eigenen Ordnung zurück, wo `260812-1527` schon zwei gezählt
hat.

**Empfehlung:** die Berichtigung zu `260812-1527` auf den neuen Block ausdehnen. Wenn die
zweigliedrige Fassung des Datensatzes gilt („`opt+cmd+X` wirkt auf Ordner und Bereiche"), ist
`weitere_instanz` die zweite benannte Ausnahme neben `opt+cmd+delete`, und sie gehört dort
genannt statt schweigend übergangen.

### Nicht berührt: `260812-1526` und `260812-1528`

Beide Befunde der Runde 6 stehen unverändert. S15 hat weder die Halbzeile an
`ordner_der_datei` (`:250-251`) noch den Abschnittskopf über `teilen` (`:573-580`) angefasst.

Zu `260812-1528` ein Datenpunkt, kein Befund: der neue Abschnittskopf „Eine weitere Instanz
von KRK" ist der **dritte** ohne Fähigkeitskennung (neben `:573` und `:642`), von jetzt 16
Köpfen. Er nennt die Runde nicht im Kopf, sondern in der Prosa darunter, und geht damit den
Weg, den `260812-1528` unter „Zu prüfen bei der Berichtigung" zur Diskussion stellt. Das
spricht für die dort erwogene Richtung.

---

## Beobachtung ohne Befundcharakter

`resources/default-keymap.toml:62-63` sagt über die Eingabetaste: „Ein stehendes Blatt faengt
sie weiterhin ab, **bevor** sie hier nachgeschlagen wird." Nach `CLAUDE.md` und nach
`Anwendungsdelegierter::kommando_ausfuehren` hält ein stehendes Blatt Befehle beim
Anwendungsdelegierten an, also **hinter** dem Nachschlag. `inference:` der Satz dürfte in
seiner Reihenfolgeaussage falsch sein. Ich habe es nicht bis zum Ende verfolgt, weil es anders
als Befund 1 **nicht** von dieser Prüfspanne verursacht ist: die Blattregel saß auch am Stand
`ca66c39` schon hinter dem Nachschlag. Wer Befund 1 behebt, sieht sich diese Zeile mit an.

---

## Empfohlene Reihenfolge

1. **Befund 1** zuerst. Er betrifft den Kopf der Datei, den jeder liest, der eine Kombination
   vergibt, und er ist von dieser Runde verursacht.
2. **Den Zutrag zu `260813-0416`** mit dessen Behebung zusammen. Beides sind zwei Sätze in
   derselben Datei.
3. **Befund 2 zusammen mit `260813-0420`.** Dessen Behebung ist die erste Verschiebung, die den
   fehlenden Absatz braucht; die beiden einzeln zu fahren, kostet zweimal dieselbe Überlegung.
4. **Den Zutrag zu `260812-1527`** zuletzt, mit der Berichtigung der Runde 6. Er ist der
   billigste und der einzige, der ohne die anderen keinen Schaden anrichtet.

Keiner der vier hält einen Schritt der Runde auf.
