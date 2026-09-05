# Das atomare Schreiben erhält die Rechte der Zieldatei

**Datum:** 2026-09-05 04:06
**Status:** Complete
**Auslöser:** Nutzerbefund aus dem laufenden Programm, Fassung v1.7.0, zum Defekt
`260904-1902_*_das-atomare-schreiben-weitet-die-rechte-einer-600-datei-auf-644.md`
**Kein Circle aktiv.**

## Der Auftrag

`ablage::atomar::vorbereiten` legte die Nachbardatei mit `fs::File::create` an, also mit
`0666 & ~umask`, und `Nachbardatei::umbenennen` setzte diesen Modus über `rename(2)` an
die Stelle des Ziels. Zwei Gestalten, eine Ursache: eine `600`-Datei stand danach auf
`644` und war für jeden Nutzer des Geräts lesbar; ein Script auf `755` stand auf `644`
und lief nicht mehr.

## Der alte Stand, gemessen

Die fünf Proben wurden **vor** der Änderung geschrieben und gegen den unveränderten
Schreibweg gefahren (die Konstante `RECHTEMASKE` musste dafür vorab dastehen, sonst
übersetzte die Testdatei nicht; sie ändert kein Verhalten). Vier von fünf rot:

| Probe | alt | soll |
|---|---|---|
| `das_atomare_schreiben_erhaelt_das_ausfuehrungsrecht` | 420 (`644`) | 493 (`755`) |
| `das_atomare_schreiben_weitet_enge_rechte_nicht_auf` | 420 (`644`) | 384 (`600`) |
| `die_nachbardatei_traegt_die_rechte_des_ziels_schon_vor_dem_umbenennen` | 420 | 384 |
| `eine_liegengebliebene_nachbardatei_ohne_schreibrecht_blockiert_nicht` | `PermissionDenied` | grün |
| `ein_noch_nicht_bestehendes_ziel_bekommt_die_vorgaberechte` | grün | grün |

Die fünfte ist absichtlich schon am alten Stand grün: sie hält fest, was **unverändert**
bleibt, und wäre als Beleg für die Behebung wertlos. Die vierte war schon vorher rot, und
das ist ein eigener Befund: eine liegengebliebene Nachbardatei ohne Schreibrecht sperrte
das Sichern dieser Datei bereits vor der Änderung, nur war sie ohne Rechteübernahme kaum
zu erreichen.

## Was gebaut ist

`crates/krk-core/src/ablage/atomar.rs`, drei Schritte in `vorbereiten`, jeder aus einem
Grund:

1. Eine liegengebliebene Nachbardatei wird **abgeräumt** statt überschrieben.
   `File::create` scheitert an einer Datei ohne Schreibrecht mit `EACCES`.
2. `rechte_uebernehmen` setzt die neun Rechtebits eines bestehenden Ziels auf den
   Deskriptor der Nachbardatei.
3. **Dann erst** fließt der Inhalt. Die Rechteprüfung eines offenen Deskriptors steht
   beim Öffnen, ein `chmod` auf `400` dazwischen nimmt dem offenen Handle das Schreiben
   also nicht — und der Inhalt liegt zu keinem Zeitpunkt offener da als das Ziel.

Der Wert `Nachbardatei` entsteht seitdem **vor** den drei Schritten, damit sein `Drop`
auch einen in ihnen gescheiterten Lauf ohne Rest hinterlässt.

Die atomare Zusage ist unangetastet: erst vollständig schreiben, dann `rename`. Die
Kindprobe `ein_abbruch_zwischen_schreiben_und_umbenennen_laesst_die_alte_datei_unveraendert`
läuft weiter grün.

## Die vier Fragen

**1. Ziel existiert noch nicht.** Es bleibt bei `0666 & ~umask`. Eine neu angelegte Datei
hat nichts zu erben, und eine Zahl an dieser Stelle wäre eine Vorgabe, die der Code sich
selbst ausdächte, statt die des Nutzers zu nehmen. Steht im Doc-Kommentar von
`rechte_uebernehmen`; die Probe vergleicht gegen eine frisch geschriebene Datei im selben
Ordner statt gegen eine Zahl, sonst hinge sie an der `umask` des Laufs.

**2. Zeitstempel, Besitzer, erweiterte Attribute.** Das Änderungsdatum steht danach auf
jetzt, und das ist richtig: die Datei ist gerade geändert worden. `Editormodell::sichern`
zieht seinen Stempel unmittelbar danach nach, und `Stempel` trägt Änderungszeit und Größe
und keine Inode-Nummer — der Wechsel des Verzeichniseintrags stört `fremd_geaendert`
also nicht. Alles Übrige, was `rename` mitnimmt, ist **nicht** behoben und gefilt als
`260905-0406_*_das-atomare-schreiben-verliert-besitzer-attribute-und-zugriffslisten-der-ersetzten-datei.md`:
Besitzer und Gruppe, die erweiterten Attribute samt Finder-Marken, Zugriffslisten,
Dateiflags, Anlagedatum, harte Verweise und eine symbolische Verknüpfung als Ziel. Jeder
Punkt außer dem letzten verlangt `libc` in `krk-core`, also eine Änderung der
Bauvoraussetzungen und damit eine Nutzerentscheidung.

**3. Alle Rufer.** Ja, und für jeden richtig. Die Ablagedateien stehen beim ersten
Schreiben noch nicht da und bekommen die Vorgabe; beim zweiten erben sie ihren eigenen
Modus. Wer `bookmarks.toml` bewusst eng stellt, behält es seitdem. Der Weg
`Zugang::beiseite_legen` schreibt auf einen `.beschaedigt`-Pfad, den es in aller Regel
noch nicht gibt, und bekommt damit die Vorgabe — dass eine **Kopie** die Rechte ihrer
Quelle erbt, ist eine andere Frage als die dieses Auftrags und hier nicht gebaut.

**4. Fehlschlag beim Übertragen.** Er hält das Schreiben an. Und zwar auch dann, wenn
`set_permissions` `Ok` meldet: der Modus wird danach am Deskriptor zurückgelesen und
verglichen. Ein Dateisystem, das `chmod` still wegwirft, brächte sonst genau diesen
Defekt zurück, und dieses Projekt hat am 260904 einen stillen Fehlschlag beim Sichern
behoben. Der Preis ist benannt: auf einem Dateisystem ohne Rechteverwaltung scheitert das
Sichern, statt die Datei mit fremden Rechten hinzulegen. Der Nutzer verliert dabei nichts
— die alte Datei steht unverändert, sein Stand steht im Editor — und bekommt seit dem
260904 auch eine Meldung. Eng bleibt der Fall, weil `chmod` nur gerufen wird, wenn der
Modus wirklich abweicht: ein Dateisystem, das für jede Datei denselben Modus meldet,
kommt hier nie vorbei.

## Was `RECHTEMASKE` draußen lässt

Die Typbits, deren Wirkung `chmod(2)` unspezifiziert lässt, und `setuid`, `setgid`,
`sticky`. Die drei bleiben **bewusst** draußen: die Nachbardatei gehört dem schreibenden
Nutzer, das ersetzte Ziel muss ihm nicht gehört haben, und ein mitgetragenes `setuid`
übertrüge das Recht eines fremden Besitzers auf einen Inhalt, den dieser Nutzer
geschrieben hat. Verloren geht dabei nichts, was heute erhalten bliebe: vor dem 260905
fielen alle zwölf Bits.

## Geänderte Dateien

- `crates/krk-core/src/ablage/atomar.rs`
- `crates/krk-core/tests/ablage.rs`

## Abnahme

`make check` — exit 0.
