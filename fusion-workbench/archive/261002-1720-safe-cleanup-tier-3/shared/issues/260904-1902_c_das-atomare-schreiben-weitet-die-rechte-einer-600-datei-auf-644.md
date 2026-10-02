Das atomare Schreiben weitet die Rechte einer 600-Datei auf 644

---
`ablage::atomar::vorbereiten` (`crates/krk-core/src/ablage/atomar.rs`) legt die Nachbardatei mit `fs::File::create` an. Die bekommt damit `0666 & ~umask`, also auf diesem Gerät `644`, und das anschließende `rename(2)` setzt diesen Modus an die Stelle des Ziels. **Die Rechte des Ziels gehen dabei verloren und werden aufgeweitet**, wenn das Ziel enger stand als die umask des Prozesses.

**Gemessen am 260904-1848**, ohne Netzlaufwerk, in einem Wegwerfordner:

```
vorher:  -rw-------  geheim.txt          (chmod 600)
nach `krk_core::text::datei::sichern`:
         -rw-r--r--  geheim.txt
```

Betroffen ist jeder Rufer von `atomar::schreiben`, also das Sichern aus dem Editor **und** die Ablagedateien unter `~/Library/Application Support/KRK/`. Für den Editor wiegt es am schwersten: eine Datei, die der Nutzer bewusst auf `600` gesetzt hat, ist nach dem ersten `cmd+s` für jeden Nutzer des Geräts lesbar, und nichts sagt es ihm. Die Dateien unter Google Drive stehen dort durchweg auf `600`.

**Gefunden bei der Untersuchung von `260904-1827` und nicht behoben**, weil die Behebung eine Entscheidung verlangt, die diesem Defekt nicht gehört: ob die Nachbardatei die Rechte des Ziels erbt (dann braucht `vorbereiten` ein `fstat` am Ziel und ein `fchmod` am Deskriptor, und für ein noch nicht bestehendes Ziel eine Vorgabe), oder ob sie eng anfängt und erst beim Umbenennen aufgeht.

---
**Filed by:** bugfixer, Kai Stalmann <kai@qantr.com>
**Domain:** code

---
Also seen: 260905 by user — dieselbe Ursache in ihrer schwereren Gestalt: ein **ausführbares Script** (`755`) steht nach dem Sichern im Editor auf `644` und läuft nicht mehr. Der gefilte Fall beschreibt die Aufweitung einer engeren Datei; dieser ist der Verlust des Ausführungsrechts, und er kostet den Nutzer mehr als eine Rechteänderung: die Datei ist danach unbrauchbar, bis er sie von Hand wieder ausführbar macht. Beide Male ist es `fs::File::create` in `atomar::vorbereiten`, dessen Rechte `rename` an die Stelle des Ziels setzt.

---
Resolved: Beide Gestalten sind behoben, und zwar an der einen Stelle, die sie gemeinsam
haben. `atomar::vorbereiten` (`crates/krk-core/src/ablage/atomar.rs`) überträgt die neun
Rechtebits eines **bestehenden** Ziels auf die Nachbardatei, und zwar **vor** dem
`io::copy` und nicht erst vor dem `rename`: läge die Übertragung am Ende, stünde der
Inhalt einer `600`-Datei für die Dauer des Schreibens unter `644` neben ihr, und bei einer
großen Datei sind das keine Mikrosekunden. Die atomare Zusage bleibt unangetastet — die
Reihenfolge „erst vollständig schreiben, dann `rename`" ist unverändert, und die
Kindprobe `ein_abbruch_zwischen_schreiben_und_umbenennen_laesst_die_alte_datei_unveraendert`
läuft weiter grün.

Die vier Fragen der Behebung, entschieden:

- **Ein Ziel, das es noch nicht gibt**, hat keine Rechte zu erben; dann bleibt es bei
  `0666 & ~umask` aus `File::create`, also bei dem, was vorher galt. Das steht im
  Doc-Kommentar von `rechte_uebernehmen` und wird von der Probe
  `ein_noch_nicht_bestehendes_ziel_bekommt_die_vorgaberechte` gehalten, die gegen eine
  frisch geschriebene Vergleichsdatei prüft statt gegen eine Zahl.
- **Übertragen werden die neun Rechtebits und nichts darüber hinaus** (`RECHTEMASKE`).
  Draußen bleiben die Typbits, deren Wirkung `chmod(2)` unspezifiziert lässt, und
  `setuid`, `setgid`, `sticky`: die Nachbardatei gehört dem schreibenden Nutzer, das
  ersetzte Ziel muss ihm nicht gehört haben. Alles Weitere, was `rename` mitnimmt —
  Besitzer, erweiterte Attribute samt Finder-Marken, Zugriffslisten, Dateiflags,
  Anlagedatum, harte Verweise, eine Verknüpfung als Ziel —, ist nicht behoben und als
  `260905-0406_*_das-atomare-schreiben-verliert-besitzer-attribute-und-zugriffslisten-der-ersetzten-datei.md`
  gefilt; jeder Punkt außer dem letzten verlangt `libc` in `krk-core`.
- **Es gilt für alle Rufer**, und das ist für jeden richtig: die Ablagedateien stehen beim
  ersten Schreiben noch nicht da und bekommen die Vorgabe, beim zweiten erben sie ihren
  eigenen Modus; wer `bookmarks.toml` bewusst auf `600` stellt, behält es seitdem.
- **Ein Fehlschlag beim Übertragen hält das Schreiben an.** Und zwar auch dann, wenn
  `set_permissions` `Ok` meldet: der Modus wird danach am Deskriptor zurückgelesen. Ein
  Dateisystem, das `chmod` still wegwirft, brächte sonst genau diesen Defekt zurück, und
  dieses Projekt hat am 260904 einen stillen Fehlschlag beim Sichern behoben. Der Preis —
  auf einem Dateisystem ohne Rechteverwaltung scheitert das Sichern — bleibt eng, weil
  nur gefragt wird, wenn der Modus wirklich abweicht; der Nutzer verliert dabei nichts und
  bekommt eine Meldung.

Dazu behoben ist eine Falle, die schon vorher bestand und durch die Rechteübernahme
erreichbarer wird: eine liegengebliebene Nachbardatei ohne Schreibrecht ließ
`File::create` mit `EACCES` scheitern und sperrte damit jedes weitere Sichern dieser
Datei. `vorbereiten` räumt sie jetzt ab, statt sie zu überschreiben. Gemessen am alten
Stand: die Probe `eine_liegengebliebene_nachbardatei_ohne_schreibrecht_blockiert_nicht`
scheiterte dort mit `PermissionDenied`.

Fünf Proben in `crates/krk-core/tests/ablage.rs`, Abschnitt „Die Rechte der Zieldatei".
Am Stand vor der Änderung gemessen: vier von fünf rot — `755` stand auf `644` (493 gegen
420), `600` stand auf `644` (384 gegen 420), die Nachbardatei ebenso, und die
liegengebliebene Nachbardatei brach mit `PermissionDenied` ab; grün war allein die Probe
über das noch nicht bestehende Ziel, die das unveränderte Verhalten festhält.
Verification: `make check` — exit 0.
