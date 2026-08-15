# Eine Verknüpfung auf einen Ordner lässt sich betreten

**Status:** Complete
**Agent:** coder
**Anlass:** Vom Nutzer gemeldeter Defekt
`shared/issues/260814-1612_p_eine-verknuepfung-auf-einen-ordner-laesst-sich-nicht-betreten.md`,
mit der vom Nutzer am 260815 entschiedenen Entwurfsrichtung: aufgelöst wird
allein im Einstiegsweg, am Deskriptor, nicht beim Lesen des Ordners. Kein
Commit.

---

## Wo der Schnitt liegt

```
Doppelklick / Rechts-Pfeil
        │
        v
tabelle::in_zeile_einsteigen
        │
        ├─ Typ::Ordner ──────────────────────────> ordner_lesen
        ├─ Typ::Datei ───────────────────────────> an das Standardprogramm
        └─ Typ::Verknuepfung
                 │
                 v
        verzeichnis::verweisziel::bestimmen        (ein Systemaufrufpaar,
                 │                                  nur hier, nur jetzt)
                 │
                 └─ sys::ohne_warten_oeffnen ──> metadata am Deskriptor
                          │
             ┌────────────┼────────────────┐
             v            v                v
          Ordner      KeinOrdner      Unerreichbar
             │            │                │
        ordner_lesen  Standard-      Statuszeile,
                      programm       kein zweiter Versuch
```

**Zwei Stellen tragen die Änderung, und keine dritte.** Im Kern das neue Modul
`crates/krk-core/src/verzeichnis/verweisziel.rs`, in der Oberfläche der
Einstiegsweg in `crates/krk-ui/src/appkit/tabelle.rs`.

Der Schnitt liegt im Kern, weil `krk-ui` kein Bibliotheksziel hat: eine Datei
unter `crates/krk-ui/tests/` erreicht nichts aus jener Kiste, und die drei
geforderten Fälle sind ohne Fenster prüfbar. Die Oberfläche behält allein die
Verzweigung und den Satz für die Statuszeile.

## Was ausdrücklich nicht geändert ist

- **`Typ`, `Eintrag::ist_ordner` und `Eintrag::ist_verknuepfung`** stehen
  unverändert. Der Defekt ist keine falsche Antwort dieser drei, sondern eine
  falsche Frage an der Einstiegsstelle.
- **Der Verzeichnisleser bekommt keinen zusätzlichen Systemaufruf.** Der
  Sortierschlüssel entsteht weiter einmal beim Lesen, und die Rechnung, an der
  L3 und L10 hängen, bleibt unangetastet. Der eine Aufruf fällt beim Doppelklick
  an und nicht bei der Anzeige.
- **Keine neue Hülle um `open` oder `stat`.** `verweisziel::bestimmen` ist der
  dritte Rufer von `sys::ohne_warten_oeffnen`, neben `text::datei::lesen` und
  dem Leseweg der Vorschau. Wie bei jenen beiden bleibt die Frage „was ist ein
  gültiges Ziel" beim Rufer: der Editor sucht eine gewöhnliche Datei, dieses
  Modul ein Verzeichnis.
- **Die anderen Rufer von `ist_ordner`** bleiben stehen; welche gelesen und
  warum sie eine andere Frage stellen, steht unten.

## Die drei Fälle

| Verweisziel | Was geschieht |
|---|---|
| Ordner | Einstieg, mit dem **Pfad der Verknüpfung** und nicht dem aufgelösten Ziel: der Aufstieg führt damit zurück in den Ordner, in dem die Verknüpfung liegt. |
| Datei (und alles, was kein Verzeichnis ist) | Was heute bei einer Datei geschieht: der Doppelklick reicht sie an das System, das seinerseits auflöst; der Rechts-Pfeil tut nichts. |
| ins Leere, Ring, ohne Recht | Ein Satz in der Statuszeile aus C1 über `befehlsantwort_zeigen`, und **kein** zweiter Versuch über das Standardprogramm. |

Der dritte Fall ist der Grund, aus dem `in_zeile_einsteigen` jetzt eine
Aufzählung `Einstieg` liefert statt eines Wahrheitswerts: „gemeldet" ist weder
„eingestiegen" noch „gib es an das System". Ginge die unerreichbare
Verknüpfung zusätzlich an das Standardprogramm, überschriebe dessen eigene
Antwort die eben geschriebene Zeile.

Der Grund im Satz ist die Meldung des Systems und keine eigene Formulierung —
dieselbe Wahl wie in `Abweisung::KeinGueltigesZiel` und in
`pfadeingabe::pruefen`. Damit trennen sich fehlendes Ziel, Ring und fehlendes
Recht am Text, ohne dass drei Zweige dafür nötig wären.

## Was an den anderen Rufern von `ist_ordner` gelesen und in Ruhe gelassen ist

- `kommandos/operationen.rs:178` und `:190` — zählen Ordner einer Auswahl für
  eine Dateioperation. Dort ist die Verknüpfung selbst das Ziel und nicht ihr
  Verweisziel; so will es der Datensatz.
- `verzeichnis/modell.rs:474` — der Markierungsstand summiert Größen und zählt
  Ordner getrennt, weil ein Ordner keine eigene Größe hat. Eine Verknüpfung hat
  eine, nämlich ihre eigene.
- `verzeichnis/modell.rs:571` und `tabs.rs:955` — die Redewendung
  `ist_ordner() || ist_verknuepfung()` für die Sichtbarkeit und die
  Auftragsliste der tiefen Suche. Für den Einstieg zu weit, weil sie auch
  Verknüpfungen auf Dateien aufnimmt; bewusst nicht übernommen.
- `appkit/tabelle.rs:2547` — die Größenspalte, aus demselben Grund wie
  `modell.rs:474`.

`sortierung.rs`, `fn gruppe` liest `eintrag.typ == Typ::Ordner` unmittelbar und
nicht über `ist_ordner`; die Sortierung ist von der Änderung nicht berührt.

## Proben

Sechs neue Proben in `crates/krk-core/tests/verzeichnis.rs`, alle über die eine
Prüfordner-Fassung des Kerns aus `tests/gemeinsam/`:

1. Verknüpfung auf ein Verzeichnis → `Ordner` (der gemeldete Fall).
2. Verknüpfung auf eine Datei → `KeinOrdner`.
3. Verknüpfung ins Leere → `Unerreichbar`, mit nicht leerem Grund.
4. Ring aus zwei Verknüpfungen → `Unerreichbar`.
5. Ordner und Datei ohne Verknüpfung → `Ordner` beziehungsweise `KeinOrdner`;
   die Einschränkung auf Verknüpfungen liegt beim Rufer, nicht in der Funktion.
6. Benannte Röhre ohne Schreiber unter Zeitschranke → kommt zurück statt zu
   hängen. Das ist die eine Zusage, um derentwillen das Modul
   `sys::ohne_warten_oeffnen` nimmt und kein `File::open`.

## Verifikation

```
cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo test --workspace
```

Exit 0.

## Offen

- **Der Klicktest am laufenden Bündel steht aus.** Die Auflösung selbst ist im
  Kern geprüft, der Weg vom Doppelklick bis `ordner_lesen` ist es nicht: die
  Proben der Oberfläche stehen in `#[cfg(test)]`-Modulen, und der Einstiegsweg
  hängt an `NSTableView`. Der Datensatz bleibt deshalb auf `_p_`.
- **Ein Symlink auf einen Socket** kommt als `Unerreichbar` zurück und nicht als
  `KeinOrdner`, weil `open(2)` auf einem Socket mit `ENXIO` scheitert. Der
  Nutzer bekommt eine Meldung statt einer stillen Wirkungslosigkeit; anzumerken
  ist es trotzdem.
- **Am Rand gelesen, nicht Auftrag:** `pfadeingabe::pruefen` fragt weiter
  `std::fs::metadata` am **Pfad** und danach `read_dir` auf denselben Pfad. Das
  ist die Bauart, die `text::datei::lesen` am 260810 verlassen hat: zwischen
  beiden Aufrufen liegt ein Zeitfenster. `speculation:` Anhalten kann es dort
  nicht — `stat(2)` blockiert an einer Röhre nicht, und `read_dir` scheitert an
  ihr mit `ENOTDIR` —, es bleibt also ein Zeitfenster und kein Stillstand.
  Nichts daran ist mit diesem Auftrag angefasst.
