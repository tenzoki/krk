# C3.15: die zweite Hälfte gemessen

**Datum:** 260815-0430
**Agent:** coder
**Status:** Complete
**Circle:** 260814-1551-tippen-filtert-dateiliste-flach-und-tief

## Auftrag

Die vom Shaper ausgesagte Messlücke von C3.15 schließen. Gemessen war bis dahin
allein die Rückrichtung, nämlich dass der Durchlauf keinen eigenen
Deskriptormangel erzeugt. Ungemessen war die Vorwärtsrichtung: dass ein von
außen herbeigeführter Mangel zu **keinem** Befund führt.

## Was gemacht wurde

**1. Eine Kindprobe für die Vorwärtsrichtung.**
`ein_deskriptormangel_von_aussen_laesst_die_ordner_unentschieden` in
`crates/krk-core/tests/verzeichnis.rs`, gebaut nach derselben Form wie die
vorhandene Probe daneben: der Elternteil legt den Baum an, das Kind läuft über
`/bin/sh` mit `ulimit -n 64`, und der Elternteil räumt ab.

Der Unterschied zur vorhandenen Probe ist der Aufbau. Dort ist der Baum tief
und der Durchlauf soll trotzdem durchkommen. Hier ist er flach, und das Kind
**hält** die Deskriptoren, während der Durchlauf läuft: es nimmt sie, bis keiner
mehr kommt, und gibt sie erst nach dem Einsammeln der Befunde zurück. Das erste
`File::open` des Durchlaufs kann damit nur noch `EMFILE` liefern, und der Zweig
`Err(fehler) if ist_deskriptormangel(&fehler) => return None` wird sicher
getroffen.

**Der zweite Halbsatz von C3.15 ist mitgemessen**, dass nämlich die noch
offenen Aufträge ebenfalls unentschieden bleiben. Der Auftrag steht auf drei
Einträgen, und der erste ist eine symbolische Verknüpfung. Die ist ohne ein
einziges Öffnen entschieden (C3.9), also kommt für sie ein Befund; für die
beiden Ordner danach kommt keiner, obwohl der Kanal danach schließt und
niemand abgebrochen hat.

Drei Vorkehrungen halten die Probe davon ab, mehr zuzusagen als sie hält:

- **Eine Gegenprobe mit freiem Vorrat läuft zuerst.** Sie entscheidet dieselben
  drei Aufträge vollständig. Ohne sie sähe der zweite Durchgang auch dann so
  aus, wenn der Baum gar nicht stünde oder der Filtertext nirgends träfe.
- **Der Mangel wird als Mangel nachgewiesen.** Der Fehler, an dem das Nehmen
  endet, geht durch `ist_deskriptormangel`; ein Scheitern aus einem anderen
  Grund lässt die Probe rot werden statt grün.
- **Die abgesenkte Grenze wird gemessen**, nicht angenommen: weniger als
  `DESKRIPTORSCHRANKE` gleichzeitig freie Deskriptoren, sonst Abbruch mit der
  Meldung, dass `ulimit` nicht gegriffen hat.

Ein leerer Kanal hat in `unterbaum_entscheiden` genau zwei mögliche Ursachen,
den Abbruch und den Deskriptormangel. Die erste ist ausgeschlossen, weil der
`Durchlauf` bis zum Ende des Einsammelns lebt und niemand `abbrechen` ruft.

**Mutationsprobe gefahren.** Mit entferntem Zweig kommen alle drei Befunde
(`index: 5, 7, 8`, jeder mit `treffer: false`) statt des einen, und die Probe
wird rot. Der Baum ist danach unverändert gegen HEAD wiederhergestellt.

**2. Eine eigene Probe für `ist_deskriptormangel`.**
`nur_emfile_und_enfile_gelten_als_deskriptormangel` im `#[cfg(test)]`-Modul von
`crates/krk-core/src/verzeichnis/sys.rs`, ohne Kindprozess. Sie trennt `EMFILE`
und `ENFILE` von `ENOENT`, `EINTR`, `EACCES`, `ENOTDIR`, `ELOOP` und
`ENAMETOOLONG` — und dazu von einem selbst gebauten Fehler ohne
Betriebssystemnummer. Der letzte ist der Fall, den ein Blick auf die Zahlen
übersieht: `Schwungleser::oeffnen` baut den Fehler „kein Verzeichnis" selbst,
`raw_os_error()` liefert dafür `None`, und das darf nicht als Mangel durchgehen.

**3. Der Doc-Kommentar nachgezogen.** Die Überschrift von
`die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden` nannte
„C3.8 und C3.10". Nach der Spec-Berichtigung vom 260815-0246 gilt C3.10 für
Gründe am Pfad; die genauere zweite Nennung ist C3.15.

## Geänderte Dateien

- `/Users/k1/Projects/productive/krk/crates/krk-core/tests/verzeichnis.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-core/src/verzeichnis/sys.rs`

Kein Produktivcode geändert. `crates/krk-core/src/verzeichnis/durchlauf.rs` ist
unverändert gegen HEAD.

## Verifikation

`make check` — exit 0. Die drei einschlägigen Proben laufen grün:

- `verzeichnis::sys::tests::nur_emfile_und_enfile_gelten_als_deskriptormangel`
- `ein_deskriptormangel_von_aussen_laesst_die_ordner_unentschieden`
- `die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden`

## Stand von C3.15

**Beide Richtungen sind gemessen, und beide Halbsätze des Kriteriums auch.** Der
Spec darf die Einschränkung im Klammerzusatz von C3.15 fallen lassen und C3.8
als Nachweis der Rückrichtung, `ein_deskriptormangel_von_aussen_laesst_die_
ordner_unentschieden` als Nachweis der Vorwärtsrichtung nennen.

Ungemessen bleibt eine Feinheit, die C3.15 nicht zusagt: ob der Durchlauf nach
dem Mangel **anhält** oder die restlichen Aufträge nur **übergeht**. Beide
Lesarten führen zu demselben beobachtbaren Ergebnis, nämlich keinem Befund, und
eine Probe kann sie am Kanal nicht trennen. Am Code ist es ohne
Fallunterscheidung abzulesen: `durchlauffaden` ruft bei `None` schlicht
`return`.
