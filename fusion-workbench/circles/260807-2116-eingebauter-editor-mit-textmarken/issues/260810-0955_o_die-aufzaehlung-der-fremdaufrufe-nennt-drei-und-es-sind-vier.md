# Die Aufzählung der Fremdaufrufe nennt drei, und es sind vier

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, beim Umbau von `oeffnen` auf den Deskriptor
**Betroffen:** `crates/krk-core/src/lib.rs:11`, `crates/krk-core/src/verzeichnis/mod.rs:11-13`
**Cross-references:** `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`

---

## Der Befund

`verzeichnis/sys.rs` bindet seit dem 260810 einen vierten Fremdaufruf,
`fcntl(2)`, für `ohne_warten_oeffnen`. Zwei Dateiköpfe außerhalb dieses Moduls
zählen die Aufrufe namentlich auf und stehen seither auf drei:

- `crates/krk-core/src/lib.rs:11` — "das Modul `verzeichnis::sys` bindet die
  Systemaufrufe `getattrlistbulk`, `copyfile` und `renamex_np`".
- `crates/krk-core/src/verzeichnis/mod.rs:11-13` — "[`sys`] ist die einzige
  Stelle im Kern mit einem Fremdaufruf und bindet `getattrlistbulk(2)` für das
  Lesen sowie, seit Schritt 15, `copyfile(3)` und `renamex_np(2)` für die
  Operationsmaschine."

Der Modulkopf von `sys.rs` selbst ist nachgezogen: er sagt "die vier
Fremdaufrufe" und führt `fcntl(2)` in seinem Diagramm.

## Warum das zählt

Die beiden Sätze sind keine Prosa, sondern die Stelle, an der ein Leser nachsieht,
welche Fremdbindungen der Kern hat und wo die eine Ausnahme von
`#![deny(unsafe_code)]` liegt. Eine Aufzählung, die eine Bindung verschweigt,
verschweigt genau das, was sie aufzuzählen behauptet. Der Satz in `mod.rs` trägt
außerdem die Aussage "die einzige Stelle im Kern mit einem Fremdaufruf", und die
bleibt wahr — falsch ist allein die Liste dahinter.

## Was zu tun ist

`fcntl(2)` in beiden Aufzählungen mitnennen, in `mod.rs` mit dem Anlass
(`ohne_warten_oeffnen` für `text::datei::oeffnen`). Kein Verhalten, nur zwei
Dateiköpfe.

## Warum es nicht mitgekommen ist

Beide Dateien lagen außerhalb der Dateigrenze des Arbeitspakets, das die vierte
Bindung angelegt hat. Die Grenze war ausdrücklich gesetzt, weil parallel andere
Agenten im Baum arbeiten.
