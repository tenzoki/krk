Zwei oeffentliche Zugaenge der Ablage haben im ganzen Arbeitsbereich keinen Rufer

---

`Lesezeichenliste::eintrag` (`crates/krk-core/src/ablage/lesezeichen.rs:368-370`) und
`Nachbardatei::ziel` (`crates/krk-core/src/ablage/atomar.rs:123-125`) werden von keiner Stelle
in `crates/` oder `xtask/` gerufen, weder im Quelltext noch in einer Probe. Beide sind `pub` in
einer Bibliothekskiste, also warnt der Uebersetzer nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/ablage/lesezeichen.rs:368`,
`crates/krk-core/src/ablage/atomar.rs:123`
**Tree state:** `004ff72`
**Domain:** code

## Die Messung

```sh
grep -rnE "\.eintrag *\(" crates xtask     # keine Zeile
grep -rnE "\.ziel *\(\)" crates xtask      # keine Zeile
```

Zum Vergleich: der unmittelbare Nachbar `Nachbardatei::nachbarpfad` (`atomar.rs:128-130`) hat
zwei Rufer, beide in `crates/krk-core/tests/ablage.rs:2369` und `:2493`. Die Liste selbst wird
ueberall ueber das oeffentliche Feld `Lesezeichenliste::eintraege` gelesen
(`lesezeichen.rs:353`), nie ueber `eintrag`; damit ist der Zugang nicht nur ungerufen, sondern
auch nicht die Form, die der Baum benutzt.

## Was daran haengt

Nichts an der Laufzeit. Der Preis ist die Frage, die jede der beiden Zeilen dem naechsten Leser
stellt: warum gibt es einen gepruefenden Zugang neben dem offenen Feld, und wer braucht das
Ziel einer Nachbardatei, die ihr Ziel in `umbenennen` ohnehin selbst kennt. `eintrag` traegt
zudem eine Zusage („Das Lesezeichen an dieser Stelle"), die keine Probe haelt.

Zu entscheiden ist je Zeile, ob sie faellt oder ob der Baum sie benutzen soll — `eintrag` ist
die sichere Form gegenueber `eintraege[stelle]`, und wenn sie das sein soll, gehoert sie
gerufen.

**Verwandt, aber nicht dasselbe:**
`shared/issues/260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`
zaehlt dieselbe Bauart in anderen Modulen ab; dort hat ein Teil der Namen wenigstens seine
eigene Probe, diese zwei haben auch die nicht.

**Gefunden:** coderev, Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
260826-1225.
