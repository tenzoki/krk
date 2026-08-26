Zwei verschiedene Typen unter `verzeichnis/` heissen beide `Lesestand`

---

`verzeichnis::leser::Lesestand` (`leser.rs:169`) ist der oeffentliche Rueckgabewert von
`lesen_hoechstens` und traegt `eintraege` samt `abgeschnitten`.
`verzeichnis::durchlauf::Lesestand` (`durchlauf.rs:610`) ist der private Zustand des **einen**
gerade gelesenen Ordners und traegt `leser`, `pfad`, `stapel`, `vorrat`, `erschoepft`. Sie haben
kein Feld gemeinsam und beantworten verschiedene Fragen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Affected:** `crates/krk-core/src/verzeichnis/leser.rs:169`,
`crates/krk-core/src/verzeichnis/durchlauf.rs:610`,
`crates/krk-core/src/verzeichnis/mod.rs:109-122`
**Tree state:** `004ff72`
**Domain:** code

## Warum das nicht bloss Geschmack ist

Es ist buchstaeblich der Befund, den dieser Modulbaum am 260817 fuer `Befund` aufgeloest hat
(`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1419_*`,
geschlossen), und `mod.rs` schreibt die Lehre daraus in einem eigenen Absatz aus:

```
// crates/krk-core/src/verzeichnis/mod.rs:109-113
//! **Er heisst ausdruecklich nicht `Befund`, denn dieser Name gehoert hier einem
//! anderen Typ.** Der Wortstamm traegt in diesem Modulbaum mehrere Typen, und
//! diese drei gehoeren zusammen: [`modell::Befund`], ... [`Befundmeldung`] ...
//! [`Inhaltsbefund`] ...
```

Der Absatz zaehlt die `Befund`-Familie vollstaendig auf und nennt die zweite Doppelung nicht,
obwohl sie im selben Verzeichnis steht und **aelter** ist als er: `durchlauf::Lesestand` kommt
aus der Runde 10, `leser::Lesestand` aus der Runde 16.

Die Verwechslungsgefahr ist hier groesser als beim `Befund`, denn der oeffentliche der beiden
reist aus dem Modul heraus: `crates/krk-core/src/leseprofil/bausteine.rs:191` fuehrt
`use crate::verzeichnis::leser::{self, Lesestand};` und nennt ihn danach unqualifiziert in
zwoelf Zeilen (`bausteine.rs:354`, `:361`, `:380`, `:396`, `:417`, `:444`). Wer von dort in
`verzeichnis/` nachliest und im `durchlauf` auf `Lesestand::neu(leser, pfad)`
(`durchlauf.rs:518`) stoesst, liest einen anderen Typ als den, mit dem er gekommen ist.

## Was der Uebersetzer hier nicht leistet

Dasselbe wie beim `Befund`: er trennt sie, eine Verwechslung uebersetzt nicht. Der Schaden
trifft den Leser und nicht den Bau, und genau deshalb faengt ihn keine Probe.

## Richtung

Die Kriterien des `Befund`-Entscheids, auf diesen Fall angewandt, zeigen in dieselbe Richtung
wie damals: umbenannt wird der **juengere und engere** von beiden. Das ist hier der private
`durchlauf::Lesestand`, mit vier Fundstellen in einer Datei, gegen den oeffentlichen mit
Ruferstellen in `leseprofil/` und in `krk-core/tests/`. Der Wortstamm bleibt und der Gegenstand
kommt davor, so wie bei `Inhaltsbefund` und `Loeschzielbefund`: `Ordnerlesestand` oder
`Ebenenstand` benennen, was er ist, naemlich der Stand **des einen offenen Ordners**.

Der Absatz in `mod.rs:109-122` waere dabei um die zweite Familie zu erweitern, sonst zaehlt er
weiter eine Doppelung vollstaendig auf und uebergeht die daneben.
