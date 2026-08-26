`Abschluss::ist_abgebrochen` hat ausserhalb der Proben keinen Rufer im Baum

---

`Abschluss::ist_abgebrochen` (`crates/krk-core/src/verzeichnis/leser.rs:68-71`) wird im ganzen
Arbeitsbereich einmal gerufen, und zwar in `crates/krk-core/tests/verzeichnis.rs:155`. Kein
Produktivcode fragt danach. Weil `krk-core` eine Bibliothek ist, sieht `dead_code` es nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/verzeichnis/leser.rs:67-77`
**Tree state:** `004ff72`
**Domain:** code

## Die Erhebung

```
$ grep -rn 'ist_vollstaendig\|ist_abgebrochen' crates/
crates/krk-core/tests/verzeichnis.rs:111:        abschluss.ist_vollstaendig(),
crates/krk-core/tests/verzeichnis.rs:155:        abschluss.ist_abgebrochen(),
crates/krk-bench/src/messen.rs:232:                if !abschluss.ist_vollstaendig() {
crates/krk-ui/src/kommandos/fokus.rs:698:   (ein Probennamen, nicht diese Methode)
crates/krk-core/src/operation/fortschritt.rs:64:  (eine gleichnamige Methode eines anderen Typs)
```

`ist_vollstaendig` hat mit `krk-bench/src/messen.rs:232` einen Rufer ausserhalb der Proben.
`ist_abgebrochen` hat keinen. Die Oberflaeche kommt ohne beide aus: `krk-ui/src/tabs.rs:1118`
verzweigt direkt ueber die Variante,

```rust
if let Abschluss::Fehler(fehler) = &abschluss {
```

und behandelt `Vollstaendig` und `Abgebrochen` gleich.

## Warum das gemeldet gehoert

Die Durchsicht war ausdruecklich auf tote Zweige angesetzt, und dies ist einer der wenigen, die
der Uebersetzer in diesem Baum grundsaetzlich nicht finden kann. Zwei Nachbarmodule fuehren
denselben Umstand ausdruecklich in ihrem Modulkopf, statt ihn zu lassen: `umfang.rs:146-149`
und `arbeitsbaum.rs:173-176` schreiben beide aus, dass `dead_code` sie nicht trifft, weil
`krk-core` eine Bibliothek ist. `leser.rs` sagt zu seinen beiden Praedikaten nichts.

Der Baum hat den Fall schon einmal als Defekt gefuehrt und geschlossen:
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1419_c_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`
— derselbe Titel, dieselbe Lage, ein anderes Praedikat.

## Richtung

Zwei ehrliche Ausgaenge, und die Wahl haengt daran, ob `Abschluss` seine Praedikate als API
fuehren will:

1. **Streichen.** `ist_abgebrochen` faellt, die eine Probe verzweigt ueber die Variante wie
   `tabs.rs:1118` es tut. `ist_vollstaendig` bleibt, es hat seinen Rufer.
2. **Behalten und den Grund aufschreiben.** Ein Satz im Modulkopf nach dem Vorbild von
   `umfang.rs:146-149`, der sagt, dass die Methode als Bibliotheks-API dasteht und heute nur
   die Probe sie ruft — so wie `sys.rs:39-45` es fuer `ortszeit` tut, samt Anweisung, wann der
   Absatz zu streichen ist.

Der zweite Weg ist der billigere und der, den dieser Baum sonst geht. Ein dritter — einen
Rufer erfinden — steht nicht zur Wahl.

Also seen: 260826-1221 by coderev — die gleichnamige Methode `operation::Abschluss::ist_abgebrochen` (`crates/krk-core/src/operation/fortschritt.rs:64-66`) hat im ganzen Arbeitsbereich nicht einmal eine Probe als Rufer; festgehalten in `shared/issues/260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`.
