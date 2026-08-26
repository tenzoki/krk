# Abgleich 260826-1452 — nach der Vollbaum-Durchsicht

**HEAD:** `de1e2db` · **Domäne:** code · **Kein aktiver Circle**, alles im gemeinsamen Speicher.
**Directive der Sitzung:** Tiefe Durchsicht des ganzen Quelltexts (155 Rust-Dateien, 126.707 Zeilen) gegen Maximen und Architektur. Der Quellbaum ist unverändert (`git diff --name-status 004ff72..HEAD` trifft nur `fusion-workbench/`).

## Umfang

- Pläne gelesen: 0 geändert (kein Plan dieser Sitzung; die Durchsicht lief ohne Plan).
- Berichte: 15 unter `shared/reviews/260826-1[2-4]*-coderev-*.md`, keine Zeile geändert.
- Defektdatensätze: 122 neue mit Stempel `260826-12*` bis `260826-14*`; 15 stichprobengeprüft, 2 nachbearbeitet.
- Entscheidungsdatensätze: 4 neue geprüft, 1 bestehender bewegt.

## Stichprobe: je Bericht ein Datensatz gegen den Baum `de1e2db`

| Datensatz | zitierte Stelle | Befund |
|---|---|---|
| `1221_o_der-schwungleser-oeffnet-mit-file-open-…` | `verzeichnis/sys.rs:229-236` | trägt `File::open` mit `metadata()` danach — stimmt |
| `1221_o_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-…` | `operation/verschieben.rs:111-129`, `kopieren.rs:115-118` | `kopieren_nach` gibt bei `Err` `Weiter`, `baum_entfernen` folgt — stimmt |
| `1223_o_kommando-kennung-vergleicht-ueber-as-u8-…` | `tasten/belegung.rs:1107-1117` | `kommando as u8 == self as u8` — stimmt; `Kommando` zählt 79 |
| `1225_o_juengste-mit-anzahl-null-…` | `leseprofil/datei.rs:524-528`, `bausteine.rs:645-648` | `gekappte_anzahl` nimmt 0 an, `truncate` leert, `Wert::Nicht` — stimmt |
| `1302_o_ein-lauf-ohne-runden-besteht-das-gate-…` | `krk-bench/src/messen.rs:579-583`, `:622-625` | `gehalten == runden` bei 0/0 — stimmt |
| `1303_o_ein-platzhalter-steht-in-einer-meldung-die-nicht-formatiert` | `tests/leseprofil.rs:664-665` | `expect_err("der Wert {wert:?} …")` ohne `format!` — stimmt |
| `1307_o_ein-messbericht-kann-einen-frueheren-still-ueberschreiben-…` | `bericht.rs:495-498`, `messen.rs:2008-2011` | Minutenstempel im Namen — stimmt |
| `1325_o_lesezeichen-anlegen-meldet-angelegt-…` | `appkit/anwendung.rs:2135-2148`, `tabelle.rs:3306-3309` | unbedingtes `antwort_zeigen` nach `lesezeichen_aendern`, Ersetzen der Antwort — stimmt |
| `1327_o_umbruch-setzen-ruft-setcontainersize-…` | `appkit/editor.rs:2864-2868` | `setContainerSize` — stimmt |
| `1334_o_frei-zeigen-sagt-die-vorgabe-stehe-ausgewaehlt-…` | `blaetter/namenseingabe.rs:95-98`, `pfadeingabe.rs:67` | Doc-Kommentar sagt „ausgewaehlt", `selectText` nur in `pfadeingabe.rs` — stimmt |
| `1416_o_die-nummernspalte-kopiert-bei-jeder-textaenderung-…` | `appkit/nummernspalte.rs:314-321` | `string().to_string()` und `Zeilenindex::neu` je Aufruf — stimmt |
| `1420_o_der-modulkopf-von-fokus-rs-spricht-von-rund-fuenfzig-…` | `kommandos/fokus.rs:34` | „rund fuenfzig Befehle"; `awk` zählt 79 — stimmt |
| `1423_o_zwei-zaehlangaben-zu-inhalt-in-vorschaumodell-rs-…` | `vorschaumodell.rs:552-555`, `:1162-1169` | „ein siebter Inhalt", „alle sechs Werte"; `Inhalt` trägt 7 — stimmt |
| `1446_o_krk-sign-identity-mit-einem-strich-signiert-ad-hoc-…` | `xtask/src/sign.rs:254-261`, `:234-238` | `aus_umgebung` prüft nur auf leer; `--sign` nimmt den Namen — stimmt |
| `1442_o_die-liste-der-gemaechlichen-arten-in-auffrischung-rs-…` | `auffrischung.rs:796-812` | `[Art; 3]` und „alle vier Operationsarten" — stimmt |

15 von 15 tragen an der zitierten Zeile, was der Datensatz behauptet. Kein neuer Defekt aus der Stichprobe.

## Altbefunde

- `shared/issues/260826-1442_*_die-frage-welche-tasten-die-schaltflaechen-behalten-…`: trifft zu. `crates/krk-ui/src/appkit/belegungsansicht.rs:190-206` (`SCHALTFLAECHEN`), `:710`, `:745-752` (`Taste::EingabeMitBefehl`) und `belegungsmodell.rs:701-709` tragen Möglichkeit 1; erster Commit `ced0ee7` (260813). `shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-…` trägt jetzt `Answered:` (Spec C1.16, `shared/planning/260813-0053_*_spec-suche-in-der-belegung-…:183`; Empfehlung, keine Nutzerantwort) und `Implemented:` und ist `_o_` → `_i_` benannt. Der Defekt ist mit `Resolved:` auf `_c_` gesetzt.
- `shared/issues/260826-1306_*_claude-md-nennt-cargo-test-als-zweiten-greifer-…`: zitiert richtig (`CLAUDE.md:129` wörtlich; `messen.rs:1029`, `:1661`, `:2720-2721`, `:2769`). Bleibt `_o_` mit Vermerk; die Änderung an `CLAUDE.md` gehört dem Curator.
- Der Bericht `260826-1302` meldet `260810-1925` als „gilt nicht mehr" — der Datensatz trägt schon `_c_`, nichts zu bewegen. Der Bericht `260826-1424` prüft sieben Altbefunde, alle „gilt weiter", Nachtrag an `260815-0020` bereits angehängt (`8bab018`). Kein weiterer Bericht meldet einen Altbefund als erledigt.

## Der `#[must_use]`-Durchgang

Zwölf Datensätze, nach Dateigruppen disjunkt; die Liste und die Berührungen ohne Doppelung stehen im Abschnitt `## Coherence` der Sitzungsdatei `shared/history/260826-1114-orchestrator-session.md`. Kein Sammeldatensatz angelegt, keiner geschlossen.

## Abweichungen zwischen Auftrag und Bestand

- Der Auftrag nennt „rund 125" neue Defektdatensätze und 5 neue Entscheidungen; der Bestand trägt 122 Defekte (`ls shared/issues/260826-1[2-4]*.md`; `git diff --name-status 004ff72..HEAD` zählt 121 `A` und 15 `M` mit `Also seen`) und 4 Entscheidungen (`260826-1221`, `-1223`, `-1225`, `-1302`).
- Ein Datensatz liegt nicht eingecheckt im Arbeitsbaum: `shared/issues/260826-1445_o_the-playmakers-ranking-rewards-a-stale-grounding-because-no-criterion-asks-whether-the-directive-is-still-true.md` (englisch, Gegenstand ist das Framework).
- Die Sitzungsdatei sagt im Kopf `**Directive:** noch nicht gesetzt`; `agentstate.yaml` trägt die Directive. Nicht angefasst — der Kopf gehört dem Orchestrator.

## Bestandszahlen

Offene Defekte (`_o_`+`_p_`) über beide Speicher: 315 (shared 199); vor der Sitzung 194. Offene Entscheidungen (`_o_`): 40 (shared 19); vor der Sitzung 37.

## Geänderte und angelegte Dateien

- `fusion-workbench/shared/decisions/260813-0053_o_…` → `260813-0053_i_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md` (umbenannt, zwei Zeilen gefüllt)
- `fusion-workbench/shared/issues/260826-1442_o_…` → `260826-1442_c_die-frage-welche-tasten-die-schaltflaechen-behalten-ist-seit-runde-7-gebaut-und-steht-noch-offen.md` (umbenannt, `Resolved:` angehängt)
- `fusion-workbench/shared/issues/260826-1306_o_claude-md-nennt-cargo-test-als-zweiten-greifer-auf-den-messplan-der-defekt-ist-seit-260811-geschlossen.md` (Vermerk angehängt)
- `fusion-workbench/shared/history/260826-1114-orchestrator-session.md` (`## Coherence` angehängt)
- `fusion-workbench/shared/history/260826-1452-reconciliation.md` (dieses Protokoll)
