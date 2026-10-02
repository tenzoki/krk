# `CLAUDE.md` nennt die dritte Pflichtstelle für ein neues Kommando

**Agent:** coder
**Datum:** 260826-2245
**Auftrag:** Schritt 5 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`; Datensatz `shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`
**Ausgangsstand:** HEAD `17e5e4e`, Schritt 4 als `9a4e495` gelandet
**Status:** Complete

## Was geändert ist

- `CLAUDE.md`, Absatz „Etliche Fallunterscheidungen sind vollständig und haben keinen Auffangzweig": zwischen der Aussage über den Übersetzer und der über den Ausführungszweig stehen jetzt drei Sätze über `Kommando::KENNUNGEN` als dritte Pflichtstelle jedes neuen Kommandos. Sie nennen den Ort (dieselbe Datei wie `wirkungsbereich`), den Preis einer fehlenden Zeile (das Kommando übersetzt, `Kommando::aus_kennung` findet den Namen nicht und es lässt sich in keiner Belegung an eine Taste binden, `kennung()` und `tag_des_kommandos` stürzen ab) und den Halter: die Probe `jede_variante_von_kommando_steht_genau_einmal_in_kennungen` in `crates/krk-core/tests/belegung.rs`, die die Varianten aus dem Quelltext der Aufzählung liest.

## Warum die Stelle so gewählt ist

Der Plan sagt, der Satz über `wirkungsbereich` und `bereich_des_kommandos` bekomme die dritte Stelle. Die dritte Stelle **in** diesen Satz zu nehmen, hätte den unmittelbar folgenden Satz „Diese Stellen hält der Übersetzer" falsch gemacht: `KENNUNGEN` hält er gerade nicht. Der Zusatz steht deshalb hinter jenem Satz, wo der Unterschied die Ordnung des Absatzes trägt, statt sie zu brechen. Der Absatz führt damit dieselbe Dreiteilung wie zuvor, nur vollständig: was der Übersetzer hält, was eine Probe hält, was gar nichts hält.

## Keine Zahl

Die Längenangabe von `KENNUNGEN` ist im Text ohne ihre Zahl genannt („nur die Zahl der Einträge und nie ihre Namen"). Die Zahl wächst mit fast jeder Runde, und `CLAUDE.md` legt aus Grundsatz ab, was mit der nächsten Runde falsch wird; für `Kommando` steht das Zählkommando schon oben unter „Projektstand".

## Verifikation

`cargo test -p krk-core --test belegung jede_variante_von_kommando_steht_genau_einmal_in_kennungen` — exit 0, `test result: ok. 1 passed`. Die zitierte Probe steht also im Baum und läuft grün.

Kein Rot-vor-grün: die Änderung ist reine Prosa und ändert kein Verhalten. Der Plan verlangt für diesen Schritt den unveränderten Baum als Beleg.

`git diff CLAUDE.md` zeigt eine Änderung an genau einem Absatz (Zeile 133) und an keinem anderen. Die daneben geänderten Dateien `crates/krk-bench/src/messen.rs` und `bericht.rs` gehören dem parallel laufenden Schritt 6 und sind nicht angefasst.

## Was offen bleibt

Der Planschritt 5 ist nicht selbst auf `[DONE]` gesetzt: die Plandatei schreibt gerade der parallele Auftrag. Der Nutzer zieht den Marker nach. Committet wird ebenfalls vom Nutzer.
