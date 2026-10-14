Fünf Proben in `krk-ui` stellen ihren Prüffall über entzogene Rechte her und fahren die Frage in drei eigenen Formen weiter

---

Der Nutzerentscheid vom 260907-0823 gibt der Frage „was tut eine Probe, die
unter `root` nichts messen kann" eine Antwort, und der Kern trägt sie seither an
einer Stelle: `gemeinsam::rechtesperre_haelt_oder_abbruch` in
`crates/krk-core/tests/gemeinsam/mod.rs`. `krk-ui` erreicht diese Stelle nicht —
seine Proben stehen in `#[cfg(test)]`-Modulen neben dem Code, und die Kiste hat
kein Bibliotheksziel. Fünf Proben dort stellen denselben Prüffall her und
beantworten die Frage in drei verschiedenen Formen weiter.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Die fünf Stellen, und was jede unter `root` täte

Keine der fünf prüft ihre Voraussetzung.

- `crates/krk-ui/src/kommandos/pfadeingabe.rs`,
  `ein_ordner_ohne_leserecht_meldet_und_navigiert_nicht`: der gesperrte Ordner
  ließe sich betreten, die Probe fiele mit „ein Ordner ohne Leserecht fuehrte zu
  einer Navigation" aus und nennt `root` nirgends.
- `crates/krk-ui/src/editormodell.rs`,
  `ein_gescheitertes_schreiben_laesst_den_stand_stehen`: das Sichern gelänge, die
  Probe fiele mit „das Sichern haette scheitern muessen" aus, `root` ungenannt.
- `crates/krk-ui/src/appkit/abwurf.rs`,
  `ein_ordner_ohne_schreibrecht_meldet_nein`: fiele mit „C6: ein Ordner mit
  0o500 meldet ein Schreibrecht" aus, `root` ungenannt.
- `crates/krk-ui/src/belegungsausgabe.rs`, in
  `in_ordner_schreiben_deckt_die_vier_ausgaenge_ab`: fiele aus, und der
  Meldetext hängt die Frage „läuft der Lauf unter root?" an eine fachliche
  Zusicherung. Das ist die dritte Form: ein Hinweis statt einer Prüfung.
- `crates/krk-ui/src/leistenmodell.rs`,
  `zehn_textmarken_kosten_je_eine_frage_und_keinen_lesevorgang`: bliebe grün,
  weil die Gültigkeitsprüfung gar nicht öffnet. Ihr Kopf sagt das ausdrücklich
  („Unter `root` sagt diese Haelfte weniger … falsch anschlagen kann sie deshalb
  nicht") und ist damit die einzige der fünf, die ihre Lage schon benennt.

## Warum es nicht mit einem `use` getan ist

`krk-ui` hat kein Bibliotheksziel, nur `[[bin]] name = "krk"`; eine Datei unter
`crates/krk-ui/tests/` erreicht nichts aus der Kiste, und umgekehrt erreicht die
Kiste nichts aus `crates/krk-core/tests/gemeinsam/`. Eine Antwort für `krk-ui`
ist deshalb entweder eine zweite Fassung derselben Funktion neben
`crates/krk-ui/src/pruefordner.rs` — mit derselben Begründung, dieselbe
Kistengrenze wie bei den drei Prüfordner-Fassungen — oder ein Verzicht mit
Vermerk. Welches von beidem, ist nicht entschieden.

## Abnahme

Entweder tragen die vier ungeprüften Stellen denselben Abbruch wie die acht
Aufrufstellen im Kern, oder ihre Köpfe sagen wie der von `leistenmodell.rs`, was
unter `root` von ihrer Aussage übrig bleibt. Eine Meldung, die `root` nur als
Frage anhängt, zählt für keines von beiden.

Gefunden bei der Erhebung zu Auftrag 2 der Aufgabe K5,
`260907-0858-drei-kleine-nutzerantworten-in-code.md`.
