# Wie kommen die AppKit-Proben von `krk-ui` auf den Hauptfaden: zweiter Kistenkopf oder Bibliotheksziel?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md, 260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md, 260810-0212_*_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md, 261101-1320-appkit-proben-laufen-auf-dem-hauptfaden.md, 261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.md

---

## Question

Die Frage `260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md` ist zurückgestellt und damit abgeschlossen; ein zurückgestellter Datensatz kehrt nach den Regeln der Werkbank nicht in einen offenen Zustand zurück. Dieser Datensatz nimmt sie deshalb neu auf und zitiert sie. Der Anlass ist ein anderer als 260810: `make check` bricht auf dem Mac des Nutzers (macOS 26.6.2, gelinkt gegen SDK 27.0) ab, weil AppKit ab dem gelinkten SDK 26 die Fadenbindung einer `NSView` durchsetzt und die Proben unter `appkit::` auf Fäden von `libtest` laufen. Zu entscheiden ist, auf welchem Weg diese Proben den echten Hauptfaden bekommen. Betroffen sind 49 Proben in drei Dateien, Stand `4b25d3c5`; die Liste und der Befehl, der sie erhebt, stehen im Plan unter `## Current State`.

## Was am 261101 neu gemessen ist

Gemessen in der Kratzablage dieser Sitzung mit `cargo 1.97.1`, Ausgabe 2024, auf macOS 26.6.2:

- **Ein `[[test]]` mit `harness = false` übersetzt mit `cfg(test)`.** Ausgegeben wurde `cfg(test)=true`. Die Nachprüfung vom 260810-1139 nahm das Gegenteil an und schloss daraus, Möglichkeit 5 lege Prüfcode ins Bündel. Die Annahme ist widerlegt. `#[test]`-Funktionen streicht der Übersetzer in einem solchen Ziel trotzdem, weil `--test` fehlt.
- **Das Ziel läuft auf dem Faden `main`** und wird von `cargo test` mitgefahren, wie am 260810 gemessen.
- **Eine Kiste darf sich selbst als Entwicklungsabhängigkeit mit einem Merkmal nennen.** Das Merkmal ist dann in jedem Prüfbau der Kiste an und in `cargo build` aus.
- **Ein Bibliotheksziel dokumentiert `cargo doc` ohne seine privaten Teile**, ein Binärziel mit ihnen. Mit `--document-private-items` meldet `krk-core` heute sieben Fehler (`heimordner/bereitstellen.rs`, `heimordner/mod.rs` viermal, `archiv/sqlite.rs`, `verzeichnis/durchlauf.rs`).
- **Ein Prüfbau von `krk-ui` dauert auf dem M2 Max rund 10 s**, das Binärziel daneben 5 s, beide parallel (Zeitbericht `cargo --timings`, Kiste allein neu übersetzt).

Die Aussage vom 260810, ein Bibliotheksziel berühre „jeden `crate::`-Pfad in zwölf Modulen“, trägt nicht: `crate::` meint in einer Bibliothek deren Wurzel, und der Modulbaum bleibt derselbe. Das ist aus den Regeln der Sprache erschlossen und nicht am Baum gemessen (`inference:`).

## Options

1. **Zweiter Kistenkopf als Prüfziel ohne Harness.** Das ist Möglichkeit 5 aus 260810-1044, ohne ihren widerlegten Einwand. `crates/krk-ui/src/hauptfaden.rs` ist eine zweite Kistenwurzel mit derselben `mod`-Liste wie `main.rs`, eingetragen als `[[test]] name = "hauptfaden"` mit `harness = false`. Die Proben bleiben in ihren Prüfmodulen `#[cfg(test)] mod tests`. Sie verlieren `#[test]` und stehen dafür je Datei in einer Liste, die ein Läufer unter `appkit/` auf dem Hauptfaden abfährt. Der Läufer behält die Namen und die Auswahl über `cargo test <name>`.
   - Pros: Kein `#[cfg(test)]\nmod tests {` ändert sich. An dieser Zeile schneiden die Quelltextproben Betriebscode und Prüfcode: `betriebscode` in `crates/krk-core/tests/gemeinsam/mod.rs` und eigene Schnitte in `vorschaumodell.rs`, `tabs.rs`, `editormodell.rs`, `markdown.rs`, `appkit/textmerkmale.rs`, `appkit/vorschau.rs`, `appkit/tabelle.rs`, `appkit/quicknote.rs`, `appkit/editor.rs` und `appkit/anwendung.rs`. Kein Bibliotheksziel, kein Merkmal, keine Änderung am Dokumentationstor, kein neues `unsafe` außerhalb von `appkit/`. `make check` bleibt dasselbe Kommando.
   - Cons: `krk-ui` wird in `cargo test` und `cargo clippy --all-targets` ein drittes Mal übersetzt, gemessen rund 10 s mehr. Die zweite Wurzel wiederholt die `mod`-Liste; eine Zählprobe macht jede Abweichung rot. Sie trägt `#![allow(dead_code, unused_imports)]`, weil in ihrem Bau alle `#[test]`-Funktionen fehlen und ihre Helfer brachliegen. Die Erlaubnis gilt allein für dieses Prüfziel. Die zwei anderen Bauten derselben Dateien warnen weiter.
2. **Bibliotheksziel mit Prüfmerkmal und einer Läuferfunktion.** Das ist Möglichkeit 4 mit Möglichkeit 2. `src/lib.rs` trägt den Modulbaum, `main.rs` ruft eine öffentliche Funktion. Dazu kommt ein Merkmal `hauptfadenproben` über eine Selbstabhängigkeit und ein Prüfziel unter `tests/`, das eine öffentliche Läuferfunktion ruft.
   - Pros: kein dritter Übersetzungslauf. „Öffentlich“ bekommt in `krk-ui` wieder eine Bedeutung, was `260810-0212_*_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md` berührt.
   - Cons: Ein Integrationsziel sieht die Bibliothek ohne `cfg(test)`. Die Proben und jeder Helfer, den sie brauchen, müssen deshalb unter dem Merkmal ohne `cfg(test)` stehen. Alle oben genannten Schnitte läsen sie dann als Betriebscode, und die Umlaut- und die Senkenprobe in `baum.rs` würden rot: Die Probe in `blaetter/mod.rs` reicht zum Beispiel `"Löschen"` an `Schaltflaeche::neu`. Auswege sind ein Einhängen von Dateien unter `tests/` per `#[path]` in die Module oder eine Erweiterung jedes Schnitts. Daneben verliert das Dokumentationstor die privaten Teile von `krk-ui`, bis `make doc` `--document-private-items` setzt. Das setzt vorher die sieben Fehler in `krk-core` voraus. Auch `main_prueft_die_anmeldehilfe_vor_jeder_andern_handlung` in `anmeldehilfe.rs` ist nachzuziehen.
3. **Bibliotheksziel mit öffentlichen Messstücken.** Das ist Möglichkeit 4 mit Möglichkeit 1: die Proben ziehen nach `tests/` um und rufen öffentlich gemachte Stücke.
   - Pros: Die Proben sind gewöhnliche Integrationsprüfungen.
   - Cons: Die Proben greifen auf Felder und private Bauwege zu, etwa `DateifensterQuelle::neu`, `quelle.ivars().tabs`, `Eintragsansicht::bauen`, `felder_der_zeile` und `textflaeche_bauen`. Jedes davon würde öffentlich, ohne Aufrufer im Programm. Das ist das Muster, das 260810-0212 als Befund führt, vervielfacht.

## Constraints

- `make check` wird auf dem Gerät des Nutzers als Ganzes grün, und es bleibt ein Kommando (Zusage aus 260810-1044).
- „Stehen lassen“, also Möglichkeit 3 aus 260810-1044, steht nicht mehr zur Wahl. Auf SDK 27 erreicht sie das Ziel des Pakets nicht.
- Die Messungen, die drei Entscheidungen der Runde 2 tragen (`260810-0748`, `260810-0750`, `260810-0512`), bleiben im Baum.
- Kein neues `unsafe` außerhalb von `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/`; `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` bleibt unverändert grün.
- Keine neue Kiste, nichts über macOS 15.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie löst die Fadenfrage vollständig und lässt die eine Zeile unberührt, an der die Quelltextproben in den unter Möglichkeit 1 genannten Dateien Prüfcode erkennen. Ihr Preis ist gemessen und klein: rund 10 s Übersetzung. Möglichkeit 2 ist der größere Umbau. Ihr Gewinn liegt außerhalb dieses Pakets, und erkauft wird er mit einem Eingriff in die Schnittregeln oder mit einem ungewohnten Einhängen von Dateien unter `tests/`. Möglichkeit 3 verfehlt die Zusage aus 260810-0212. Der Plan `261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.md` ist allein für Möglichkeit 1 geschrieben.

Answered: 261101 — Möglichkeit 1 (zweite Kistenwurzel als Prüfziel ohne libtest-Harness), gewählt vom Nutzer mit „1“ zusammen mit der Freigabe des Plans `261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.md`.
