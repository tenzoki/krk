# Planersitzung — 260826-1811

**Auftrag:** Umsetzungsplan für die fünf schweren Befunde der Vollbaum-Durchsicht vom 260826 (ein kritischer, vier hohe); die 116 übrigen folgen in einem zweiten Plan.
**Dispatch:** vom Orchestrator (`260826-1807-orchestrator-session.md`), Executors `coder, ontocoder, analyst`, kein Circle-Parameter, kein aktiver Circle, alles unter `shared/`.
**Git-HEAD:** `26e8039`
**Status:** Complete

## Einrichtung

- `fusion-workbench-root` → `/Users/k1/Projects/productive/krk`; `fusion-rules planner` (fünf Regeldateien, beide Stilprofile `de`, `design-diagrams.md`); `fusion-paths planner` → alle `OUT_*` und `SCAN_*` auf `shared/`.
- Kennung über `fusion-identity`: Kai Stalmann <kai@stalmann.org>, Checkout 6c11b1f2.
- Offene Entscheidungen unter `shared/decisions/` gelesen (Namen); keine berührt die fünf Befunde; die vier `_a_`-Datensätze betreffen andere Runden.

## Was gelesen wurde

Die fünf Datensätze ganz, mit dem Nachtrag R5 an Befund 4. Am Baum: `operation/verschieben.rs`, `kopieren.rs`, `loeschen.rs:95-115`, `fortschritt.rs` (Steuerung, Uebersprungen), `mod.rs` (`Ablauf`, `ziel_klaeren`); `verzeichnis/sys.rs:200-262` und `:795-930` samt Prüfmodul; `tasten/belegung.rs` (Aufzählung, `KENNUNGEN`, `kennung`); `tests/belegung.rs:1690-1725`; `tests/gemeinsam/mod.rs:240-360`; `tests/umfang.rs:255-300`; `tests/text.rs:548-574`; `tests/zeit.rs:64-86`; `krk-bench/src/messen.rs` (Messreihe, Gesamtlauf, `unterordner_sicherstellen`, `ordner_beschreiben`, Proben), `bericht.rs:90-110`, `:200-225`, `fixture.rs:300-340`, `:490-580`; `appkit/menue.rs:425-445`; `Cargo.lock` (`syn` vorhanden, `strum` nicht).

## Befunde beim Planen

- `kennung()` ist am HEAD schon eine `const fn` (`belegung.rs:1133`), der Datensatz zitiert noch `:1107`; die Zeilen sind gewandert, der Befund nicht.
- Jeder Weg ohne Ankunft in `kopieren.rs` und `ziel_klaeren` ruft `ueberspringen`; der Zählstand der Steuerung ist damit ein vollständiger Zeuge, und ein dritter `Ablauf`-Wert ist nicht nötig (50 Stellen in sieben Dateien blieben unberührt).
- Der Datensatz zu Befund 4 sagt, die Prüfung auf `1 passed` schließe alle drei stillen Wege. Den zweiten (Umgebungsvariable fehlt) schließt sie nicht: ein Kind, das früh zurückkehrt, zählt als bestanden. Der Plan schließt ihn strukturell, mit einem Auftragsnamen statt sechs.
- `Ordnermodell::eintraege()` führt auch die ausgeblendeten Einträge; der Steckbriefvergleich in Schritt 6 fängt damit auch einen `.DS_Store`. Eine erste Fassung des Plans hatte das Gegenteil behauptet und ist vor dem Abschluss berichtigt worden.
- Stabiles Rust zählt Varianten nicht (`variant_count` unstabil); die Wahl zwischen Quelltextprobe und Ableitungsmakro bindet elf `ALLE`-Listen und ist als Entscheidungsdatensatz abgelegt.

## Geschrieben

- Plan: `shared/planning/260826-1811_o_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`, sechs Schritte, alle `coder`; kein Schritt braucht `ontocoder` oder `analyst`.
- Entscheidung: `shared/decisions/260826-1811_o_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`.
- Kein Defekt gefiltert: der Fehlschluss im Datensatz zu Befund 4 (Weg 2) steht im Plan an der Stelle, die ihn behebt, und ist kein eigener Defekt am Baum.

## Nicht getan

Keine Behebung, kein Code, kein Dispatch, keine Vorwegnahme der 116 übrigen Befunde.
