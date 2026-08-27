Ein gescheitertes Kopieren ueber die Datentraegergrenze loescht die Quelle trotzdem

---

`ueber_datentraeger` loescht die Quelle, sobald das Kopieren nicht **abgebrochen** wurde. Ein
Kopieren, das **gescheitert** ist, liefert denselben Rueckgabewert `Ablauf::Weiter` wie ein
gegluecktes, und der Baumloescher laeuft danach unbedingt. Die Datei oder der Ordner des Nutzers
ist damit endgueltig weg, ohne dass am Ziel etwas entstanden waere. Der Kommentar an der Stelle
behauptet das Gegenteil.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/operation/verschieben.rs:111-129`:

```rust
fn ueber_datentraeger(…) -> Ablauf {
    if kopieren::kopieren_nach(quelle, ziel, art, steuerung) == Ablauf::Abgebrochen {
        return Ablauf::Abgebrochen;
    }
    // Geloescht wird nur, was auch angekommen ist. Ist beim Kopieren etwas
    // uebersprungen worden, steht es noch in der Quelle, und `baum_entfernen`
    // scheitert daran; der Grund kommt in die Abschlussliste.
    if let Err(fehler) = loeschen::baum_entfernen(quelle.pfad) { … }
    Ablauf::Weiter
}
```

## Die Kette, Zeile fuer Zeile

1. `eintrag_verschieben` (`verschieben.rs:49-61`) bekommt von
   `im_datentraeger_verschieben` den Fehler `EXDEV` und gibt an `ueber_datentraeger` ab.
2. `kopieren_nach` (`kopieren.rs:55-66`) verzweigt ueber den Typ.
   - `Typ::Datei` → `datei` (`kopieren.rs:69-120`). Scheitert `sys_datei_kopieren`, laeuft der
     Zweig `Err(fehler)` in `kopieren.rs:115-118`: `steuerung.ueberspringen(pfad, …)` und
     **`Ablauf::Weiter`**.
   - `Typ::Ordner` → `ordner` (`kopieren.rs:123-171`). Scheitert `fs::create_dir(ziel)` mit
     etwas anderem als `AlreadyExists`, laeuft `kopieren.rs:132-133`: `ueberspringen` und
     **`Ablauf::Weiter`**. Dasselbe, wenn `lesen(quelle.pfad)` scheitert (`kopieren.rs:138-140`).
3. Zurueck in `ueber_datentraeger` ist der Wert nicht `Abgebrochen`, also faellt der Lauf in
   `baum_entfernen(quelle.pfad)`.
4. `baum_entfernen` (`loeschen.rs:101-110`) loescht die Quelle: eine Datei mit
   `fs::remove_file`, einen Ordner **rekursiv** — es steigt selbst ab und raeumt jedes Kind weg,
   bevor es `fs::remove_dir` ruft.

## Warum der Kommentar an beiden Haelften falsch ist

- Fuer eine **Datei** gibt es kein "steht noch in der Quelle, und `baum_entfernen` scheitert
  daran": die Quelle steht immer noch da, sie ist ja nie angefasst worden, und `remove_file`
  glueckt genau deshalb.
- Fuer einen **Ordner** ebenso wenig: `baum_entfernen` scheitert nicht an einem nicht leeren
  Ordner, sondern leert ihn erst. Die Vorstellung "ein uebersprungenes Kind haelt den
  Ordner fest" waere nur richtig, wenn `baum_entfernen` ein einzelnes `rmdir` waere.

## Warum das kein Randfall ist

Der Weg ist die einzige Art, wie ein Verschieben zwischen zwei Datentraegern ueberhaupt geht
(Modulkopf `verschieben.rs:16-19`), und er wird von jedem Abwurf auf ein anderes Volume
erreicht, von jedem `F6` in ein Dateifenster auf einem anderen Datentraeger und von jedem Abwurf
aus einer fremden Anwendung. Die gewoehnlichsten Ausloeser des Fehlschlags stehen alle im
Uebersetzer von `grund` (`mod.rs:476-484`): keine Rechte am Ziel, kein Platz mehr auf dem
Datentraeger, Ziel verschwunden.

Damit steht das Verschieben ueber die Datentraegergrenze als einzige Stelle der Maschine gegen
die Zusage aus C4, die der Modulkopf von `operation/mod.rs:50-54` ausschreibt: eine gescheiterte
Einzelposition sammelt Eintrag und Grund. Hier sammelt sie den Grund **und** loescht die Quelle.

## Was zu tun waere

`kopieren_nach` sagt heute nicht, ob es etwas abgelegt hat: `Ablauf` kennt nur `Weiter` und
`Abgebrochen`, und der Fehlschlag verschwindet in `Weiter`. Der kleinste tragfaehige Schnitt
ist, den Fehlschlag sichtbar zu machen — entweder ueber einen dritten Wert am Rueckgabetyp, so
wie `zippen::Packschritt` (`zippen.rs:175-184`) den seinen drei Werte gibt, oder indem
`ueber_datentraeger` vor dem Loeschen den Zaehlstand der uebersprungenen Eintraege vergleicht.
Ein Blick auf das Ziel im Dateisystem waere die schlechtere Antwort: er beantwortet fuer einen
Ordner nur, ob **etwas** ankam, und nicht, ob alles ankam.

## Umfang

`krk-core`, `operation/verschieben.rs`, mit einem Eingriff in den Rueckgabetyp von
`operation/kopieren.rs`. Keine Probe im Baum beruehrt den Weg: `crates/krk-core/tests/operation.rs`
kennt kein `EXDEV` und keinen zweiten Datentraeger.

## Was gepruefet ist und was nicht

Gelesen und Zeile fuer Zeile nachvollzogen (siehe Kette oben). **Nicht** am laufenden Geraet
nachgestellt: dafuer braeuchte es zwei Datentraeger und einen Lauf ausserhalb des Quellbaums.

---
Resolved: 260826-1900 — ueber_datentraeger merkt den Zaehlstand der uebersprungenen Eintraege vor kopieren_nach und loescht die Quelle nur, wenn er unveraendert ist; drei Proben in verschieben.rs, zwei davon rot vor der Behebung (Plan 260826-1811 Schritt 1).

Reconciled: 260826-2205 — gegen den Baum `bc5991d` geprueft und zutreffend: der Commit ist `36e54b4`, die Behebung steht in `crates/krk-core/src/operation/verschieben.rs:127-146` und `crates/krk-core/src/operation/fortschritt.rs:355-366`, die drei Proben an `verschieben.rs:198`, `:233` und `:277`; `make check` ueber `bc5991d` gruen. Der Hash steht hier als Abgleichsbeleg und nicht als Berichtigung der `Resolved:`-Zeile: welche Form sie tragen soll, bleibt offen (`shared/issues/260826-1933_*_die-zwei-resolved-zeilen-der-schritte-1-und-2-tragen-den-sitzungsstempel-statt-des-commits.md`).
