# Jeder Prüfordner wird gegen seine zugesagte Eintragszahl gehalten

**Agent:** coder
**Datum:** 260826-2210
**Auftrag:** Schritt 6 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`; Datensatz `shared/issues/260826-1301_o_kein-pruefordner-ausser-dem-l6-unterordner-wird-gegen-seine-zugesagte-eintragszahl-gehalten.md`
**Ausgangsstand:** HEAD `17e5e4e`
**Status:** Complete

## Was geändert ist

Zwei Dateien, beide in `crates/krk-bench/src/`.

**`messen.rs`**

- Zwei Konstanten neben `EINTRAEGE_L6`: `EINTRAEGE_A = 10_000` (gilt für A und B) und `EINTRAEGE_GROSS = 100_000`. Ihre Doc-Kommentare tragen die Begründung — L3 sagt 400 ms für 10.000 Einträge zu, L10 4 s für 100.000, und die Zahl ist Bestandteil der Zusage. Die drei Feld-Doc-Kommentare an `Gesamtlauf` verweisen jetzt auf die Konstanten, statt die Zahl ein zweites Mal zu nennen.
- Neu: `fn pruefordner_pruefen(ordner: &Path, erwartet: usize) -> io::Result<()>`. Es trägt die zwei Regeln, die bis dahin allein `unterordner_sicherstellen` für den L6-Unterordner hielt: ein Steckbrief mit anderer Zahl ist ein Fehler, der beide Zahlen nennt; ein Ordner ohne Steckbrief ebenso („auf unbekanntem Bestand misst diese Strecke nicht"). Die Meldung nennt für beide Fälle den Weg zurück und trennt dabei den L6-Unterordner (den der Lauf selbst neu anlegt) von den drei Prüfordnern aus C8 (die `krk-bench fixture` erzeugt).
- `unterordner_sicherstellen` behält allein das Anlegen: es legt an, wo weder Steckbrief noch Ordner steht, und ruft danach `pruefordner_pruefen`. Die Fallunterscheidung ist damit an einer Stelle statt an zweien. Ein Ordner ohne Steckbrief wird weiterhin **nicht** überschrieben, sondern fällt an die Prüfung — ein Kommentar an der Stelle sagt es.
- `Gesamtlauf::fahren` ruft `pruefordner_pruefen` für A, B und 100k neben `is_dir()`, in derselben Schleife: `[(&ordner_a, EINTRAEGE_A), (&ordner_b, EINTRAEGE_A), (&ordner100k, EINTRAEGE_GROSS)]`.
- `Messreihe::fahren` hält nach der Gleichheitsprüfung der Läufe die gelesene Zahl gegen den Steckbrief, **wenn** einer daliegt; Abweichung ist `Err` mit beiden Zahlen. Ohne Steckbrief bleibt die Reihe zulässig — der Kommentar an der Stelle nennt den Grund (`messen --kopflos` darf auf einen Ordner zeigen, den es nicht selbst erzeugt hat, wie `fixture::steckbrief_lesen` es in seinem Kopf schon sagt).
- `Gesamtrohrunde` bekommt `eintraege_a` und `eintraege_gross`; `eine_gesamtrunde` füllt sie aus `reihe_a.eintraege` und `reihe_gross.eintraege`, die bis dahin fallengelassen wurden. `Gesamtergebnis` bekommt dieselben zwei Felder.
- Neu: `fn ueber_runden_einig(runden, lesen, was) -> io::Result<usize>`. Es zieht eine Zahl aus allen Runden zusammen, die in jeder dieselbe sein muss, und liefert sonst `Err` mit Rundennummer und beiden Zahlen. Der Bericht weist **eine** Zahl aus, also muss es eine geben.
- Neu: `ordner_beschreiben_mit_gelesenen(ordner, gelesen)` neben dem bestehenden `ordner_beschreiben`. Es nimmt die Form, die der Bericht der kopflosen Strecke schon führt: `{gelesen} (laut Steckbrief: {brief})`.

**`bericht.rs`**

- `gesamt_verfassen` schreibt für „Pruefordner A" und „Pruefordner 100k" die gelesene Zahl über `ordner_beschreiben_mit_gelesenen`. „Pruefordner B" bleibt bei `ordner_beschreiben`: er trägt keine eigene kopflose Reihe und wird nicht gelesen, für ihn ist der Steckbrief die einzige Auskunft. Ein Kommentar an der Stelle sagt es.
- Die bestehende Probe `der_abnahmebericht_traegt_alle_zehn_zusagen_und_den_vollen_kopf` füllt die zwei neuen Felder und prüft, dass `Eintraege je Lauf: 10000` und `Eintraege je Lauf: 100000` im Kopf stehen.

## Rot vor grün

Probe (a), `eine_messreihe_verwirft_einen_ordner_der_seinem_steckbrief_widerspricht`: `fixture::erzeugen(…, 10, 1)`, dann eine elfte Datei in den Ordner gelegt. Vor der Behebung, am unveränderten Baum, `cargo test -p krk-bench eine_messreihe_verwirft_einen_ordner_der_seinem_steckbrief_widerspricht` wörtlich:

```
running 1 test
test messen::tests::eine_messreihe_verwirft_einen_ordner_der_seinem_steckbrief_widerspricht ... FAILED

failures:

---- messen::tests::eine_messreihe_verwirft_einen_ordner_der_seinem_steckbrief_widerspricht stdout ----

thread 'messen::tests::eine_messreihe_verwirft_einen_ordner_der_seinem_steckbrief_widerspricht' (2584737) panicked at crates/krk-bench/src/messen.rs:2662:14:
die Reihe haette den Ordner verwerfen muessen: Messreihe { ordner: "…/krk-bench-probe-steckbrief-widerspruch-92491-0", cache: Warm, wiederholungen: 2, eintraege: 11, groessen: […] }

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 56 filtered out
```

Der Rückgabewert selbst ist der Befund: die Reihe lieferte `Ok` mit `eintraege: 11`, während der Steckbrief 10 sagt. Nach der Behebung grün.

Probe (b), `ein_pruefordner_wird_gegen_seine_zugesagte_eintragszahl_gehalten`: ein Ordner mit Steckbrief 3.000 gegen `EINTRAEGE_A` ist `Err` und nennt beide Zahlen; derselbe Ordner gegen 3.000 ist `Ok`; ein Ordner ohne Steckbrief ist `Err` und nennt den fehlenden Steckbrief. Sie misst eine Funktion, die es vorher nicht gab, und ist deshalb keine Rot-vor-grün-Probe im engen Sinn.

Probe (c), `eine_messreihe_liefert_je_messgroesse_einen_wert_pro_lauf`, bleibt unverändert grün (Steckbrief 3.000, gelesen 3.000).

## Prüfung

- `cargo test -p krk-bench`: `58 passed; 0 failed`.
- `make check` — exit 0, „alle vier gruen".
- `cargo run -q -p krk-bench -- messen --kopflos --ordner <Ordner ohne Steckbrief> --ziel <Berichtsordner>` auf einem von Hand angelegten Ordner mit 25 Dateien: exit 0, Bericht geschrieben, Kopfzeile `Eintraege je Lauf     25 (laut Steckbrief: unbekannt (kein Steckbrief neben dem Ordner))`. Die Toleranz der kopflosen Strecke steht also.

## Nicht gemessen

**Kein Messlauf.** Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; die neue Kopfzeile des Abnahmeberichts zeigt sich beim nächsten Lauf des Nutzers. Die Abnahmebedingung „`Gesamtlauf::fahren` weist einen Prüfordner A mit 3.000 Einträgen vor der ersten Runde ab" ist damit nicht am laufenden Gerät gemessen, sondern an der Funktion, die den Ausschlag gibt (Probe b), und am Aufrufpunkt gelesen (`Gesamtlauf::fahren`, die Schleife über die drei Ordner).

## Nicht getan

Kein Commit. Der Datensatz `shared/issues/260826-1301_o_…` bleibt auf `_o_` und bekommt seine `Resolved:`-Zeile samt Commit-Hash beim Commit durch den Orchestrator. Der Planschritt 6 steht auf `[DONE]`. Unberührt: `crates/krk-core/`, `crates/krk-ui/` und `CLAUDE.md` — daran arbeitet ein paralleler Auftrag.
