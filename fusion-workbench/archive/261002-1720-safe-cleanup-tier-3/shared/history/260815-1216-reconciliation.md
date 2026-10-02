# Abgleich der Sitzung 260815-0912

**Datum:** 260815-1216
**Agent:** reconciler
**Domäne:** `code`
**Stand:** `9a2d0e0`, Spanne `c3fcdef..HEAD` mit drei Commits
**Status:** Complete
**Urteil:** review-needed

---

## Was abgeglichen wurde

| Fläche | Gelesen | Geändert |
|---|---|---|
| Plandateien (Spec und Plan der Runde 10, zwei im gemeinsamen Speicher) | 4 | 2 |
| Entscheidungsdatensätze (alle 136 in allen Speichern) | 136 | 3 |
| Defektdatensätze der Sitzung und des Circles der Runde 10 | 28 | 0 |
| Durchsichten der Sitzung | 1 | 0 |
| Circle-Datensätze | 11 | 0 |
| Neu angelegt | — | 2 Defektdatensätze |

Kein Code und keine Probe ist angefasst; der Auftrag war ein reiner Abgleich der Nachweisdateien. Kein Testlauf ist gefahren.

## Die sechs Markerläufe der Sitzung: alle tragen

| Datensatz | Lauf | Fußzeile | Ziel auflösbar |
|---|---|---|---|
| `circles/260814-1551-…/decisions/260814-1830_i_bleibt-der-filtertext-…` | `_o_` → `_a_` → `_i_` | `Answered:` und `Implemented:` | ja, `897605e` und `shared/history/260815-0912-orchestrator-session.md` |
| `shared/issues/260815-1047_c_c1-9-und-der-doc-kommentar-…` | neu, sofort `_c_` | `Resolved:` | ja |
| `shared/issues/260815-1047_c_vier-verweise-im-code-…` | neu, sofort `_c_` | `Resolved:` | ja |
| `shared/issues/260815-1047_d_die-bedingung-der-moeglichkeit-2-…` | neu, sofort `_d_` | `Deferred:` | ja |
| `shared/issues/260815-1047_o_die-directive-der-runde-10-…` | neu, `_o_` | keine nötig | — |
| `shared/decisions/260815-1145_o_schreiben-zitate-im-code-…` | neu, `_o_` | keine nötig | — |

Dazu ein siebter, den die Aufgabenstellung nicht nennt: `shared/issues/260815-1019_o_die-wettrennprobe-des-oeffnens-…`, neu und offen, ohne Fußzeile und ohne Bedarf für eine.

**Der `Implemented:`-Vermerk ist einzeln gegen den Baum gelesen.** `filtertext_ueberlebt` steht nur noch als Wort im Kommentar, der seinen Wegfall begründet (`crates/krk-ui/src/tabs.rs:601`); `Tabliste::ordner_setzen` trägt den Filtertext ohne Bedingung (`tabs.rs:596-610`); die drei genannten Proben stehen in `tabs.rs:1384`, `:1416` und `:1444`.

**Der `Deferred:`-Vermerk nennt keinen Zieltermin, sondern einen Auslöser** („Gebrauchserfahrung"). Das ist zulässig: die Vorschrift verlangt eine genannte Bedingung für das Wiederaufgreifen, nicht ein Datum.

**Eine Abweichung in der Kopfzeile**: der Entscheidungsdatensatz `260814-1830_i_` trug im Rumpf weiter `**Status:** open`. Auf `implemented` gezogen. Zwölf ältere Datensätze tragen dieselbe Abweichung, sie sind als eigener Defekt aufgenommen (siehe unten).

## Die Behauptungen der Sitzung am Baum

**C1.9 hält.** Die fünf Wege, auf denen ein stehender Filtertext verschwindet, sind einzeln nachgelesen und alle fünf da: `Esc` (`anwendung.rs:4587` → `tabelle.rs:1823`), die Rückschritt-Taste (`tabelle.rs:1796`), das Schließen des letzten Tabs (`tabs.rs:504-513`), der Auswurf eines Datenträgers unter einem verdeckten Tab (`tabs.rs:440-450`, einziger Aufrufer `tabelle.rs:617-625`) und der Neustart (`krk-core/src/ablage/sitzung.rs:82-113` führt weder Filtertext noch Filter der Tiefe). **Kein sechster Weg gefunden**: `Ordnermodell::tief_setzen` lässt den Filtertext stehen, und der sichtbare Tab geht beim Auswurf über `ordner_lesen` und trägt ihn mit.

**Eine Ergänzung zur Genauigkeit, kein Widerspruch.** Das Schließen eines Tabs, wenn mehrere stehen, nimmt den Filtertext ebenfalls weg (`tabs.rs:515`). C1.9 deckt das mit dem Satz „daneben fällt er mit dem Tab, der ihn hält"; in der Aufzählung der drei weiteren Wege steht es nicht, weil dort der Sonderfall des **letzten** Tabs gemeint ist, bei dem der Tab gerade nicht fällt. Wer die drei zählt, ohne den Satz davor zu lesen, zählt einen Weg zu wenig. Im Reconciliation Log des Spec festgehalten.

**Der Sichtbarkeitsvorbehalt im Doc-Kommentar von `Tabliste::ordner_setzen` hält in jedem Einzelteil.** `Rang::ALLE` trägt sechs Werte mit `Filterstand` auf Platz 5 (`statuszeile.rs:235-242`); `zeile` läuft erst über den Rang und dann über die aktive Seite (`statuszeile.rs:526-528`); `fenstermeldung_loeschen` hat genau zwei Aufrufer, `ordner_lesen` und `tab_gewechselt`, und beide räumen allein das Feld ihres eigenen Dateifensters (`tabelle.rs:721`, `:781`, `:2088`). Die Aussage, die Sichtbarkeit sei nicht zugesagt, ist damit belegt und nicht behauptet.

**Der Nebenbefund des geschlossenen Defekts hält ebenfalls:** `tabs.rs:1412` nennt heute `DateifensterQuelle::ordner_aufwaerts`, den richtigen der beiden Typen.

## Die offenen Datensätze der Runde 10

**Keiner ist durch diese Sitzung beantwortet oder gegenstandslos geworden.** Zwei sind schwerer geworden und haben eine Notiz bekommen:

**`decisions/260814-1552_o_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`.** Der Baum fährt seit der Runde 10 auf der Empfehlung, Möglichkeit 2, ohne Bestätigung des Nutzers: `Rang::Filterstand` steht über `Rang::Markierungsstand`. Zwei Angaben im Rumpf sind überholt, die Frage spricht von „fünf Werten" und die Kopfzeile von „den fünf Rängen", der Baum hat sechs. **Neu ist ein zweiter Abhängiger**: seit dem 260815 hängt an dieser Rangfolge nicht mehr nur C4.1, sondern die Bedingung des Entscheids `260814-1830_i_`, und der Nutzer hat die Lage am 260815-1055 festgehalten statt sie zu beheben. Ein vergessener Filtertext, den die Zeile nicht nennt, war die Ausnahme und ist der Regelfall.

**`decisions/260814-1830_o_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`.** Der Baum fährt auf Möglichkeit 2: Blatt, laufende Operation, dann Filtertext (`anwendung.rs:4565-4588`, „der dritte Rang"). **Die Tragweite ist gewachsen**: `Esc` ist seit `897605e` der einzige Griff, der einen stehenden Filtertext in einem Zug wegnimmt, denn kein Ordnerwechsel räumt ihn mehr ab. Am Abwägungsstoff der drei Möglichkeiten ändert das nichts.

**`issues/260815-0020_o_verdeckten-tab-setzen-…` ist korrekt nachgezogen.** Die Befundtabelle trägt für `ordner_setzen` jetzt „ja, unbedingt", die Zeilennummern stimmen, und die Einordnung als „unentschieden" ist ausdrücklich zurückgenommen. Der Datensatz bleibt zu Recht offen: an `verdeckten_tab_setzen` hat sich nichts geändert, und die Entwurfsfrage, ob die vier Werte an einer Stelle übertragen werden, ist unbeantwortet.

## `CLAUDE.md`: keine Aussage ist falsch geworden

Der Absatz über die Runde 10 (Zeile 127) ist Satz für Satz geprüft und hält. `krk-core/src/verzeichnis/filter.rs` existiert und trägt beide Regeln; `traegt_ein_dateiname` hat genau zwei Rufer (`belegungsmodell.rs:684`, `appkit/tabelle.rs:1146`), `traegt_die_folge` ebenfalls genau zwei (`verzeichnis/durchlauf.rs:304`, `verzeichnis/modell.rs:606`); der Vergleich ist eine Teilzeichenfolge ohne Rücksicht auf Groß- und Kleinschreibung und faltet keine Umlaute (`filter.rs:117-134`).

**Eine Lücke, kein Fehler**, und sie ist dem Nutzer gemeldet statt hier behoben: der Absatz sagt nichts über die Lebensdauer des Filtertextes. Die eine Regel „der Filtertext übersteht jeden Ordnerwechsel" ist seit dem 260815 die tragende Eigenschaft des Filters, sie hat eine sicherheitsnahe Nachbarin im Absatz über die Rückschritt-Taste (Zeile 139), und sie steht in `CLAUDE.md` nirgends. Ob sie hinein soll, entscheidet der Nutzer.

## Neue Defektdatensätze

**`shared/issues/260815-1216_o_sieben-verweise-dieser-sitzung-nennen-einen-marker-den-ihr-ziel-nicht-mehr-traegt.md`.** Neun Verweise nennen am Stand `9a2d0e0` einen Marker, den ihr Ziel nicht mehr trägt; sieben davon hat diese Sitzung geschrieben, fünf waren in dem Commit falsch, der sie schrieb. Es ist derselbe Fehlertyp, den die Sitzung als `260815-1047_c_vier-verweise-im-code-…` geschlossen hat: die Berichtigung las nur `crates/` und `xtask/` und lief vor den beiden neuen Zitaten in `tabs.rs`.

**`shared/issues/260815-1216_o_vierzehn-entscheidungsdatensaetze-tragen-im-rumpf-einen-anderen-stand-als-im-dateinamen.md`.** Vierzehn von 136 Datensätzen widersprechen sich zwischen Kopfzeile und Dateiname. Zwei davon sind mit diesem Abgleich weg, zwölf bleiben.

## Nachgezogene Nachweisdateien

1. `circles/260814-1551-…/decisions/260814-1830_i_bleibt-der-filtertext-…` — Kopfzeile `**Status:**` von `open` auf `implemented`.
2. `circles/260814-1551-…/planning/260814-2102_o_plan-…` → `_c_`. Der Schritt **E2** trug als einziger der vierzehn keinen `[DONE]`-Vermerk, obwohl er gefahren ist; belegt an `resources/default-keymap.toml:405-408` und an der Zahl von 84 `[[funktion]]`-Einträgen, ausgeführt laut `history/260814-2320-ontocoder-e2-eintrag-der-tiefen-suche.md`. Vermerk nachgetragen, `**Status:**` von `Entwurf` auf `Complete`, Reconciliation Log angehängt. Der Plan war der einzige einer geschlossenen Runde, der noch `_o_` trug; alle acht Pläne der Runden 1 bis 9 stehen auf `_c_`. Kein Verweis außerhalb der Speicher, für die die Ortsregel gilt, zeigt auf den alten Namen.
3. `circles/260814-1551-…/planning/260814-1830_o_spec-…` — Reconciliation Log angehängt. Der Marker bleibt `_o_`: vier der neun geschlossenen Runden führen ihren Spec auf `_c_`, fünf auf `_o_`, eine Regel steht nirgends, und eine Umbenennung ohne Regel hinterließe allein neue Verweise ins Leere.
4. `circles/260814-1551-…/decisions/260814-1552_o_wo-steht-die-filterzahl-…` — Abgleichsnotiz angehängt, Marker unverändert.
5. `circles/260814-1551-…/decisions/260814-1830_o_an-welcher-stelle-…` — Abgleichsnotiz angehängt, Marker unverändert.

## Nicht angefasst

Der Abschnitt `## Directive` des Circle-Datensatzes der Runde 10 sagt weiter, der Filter werde beim Ordnerwechsel geleert. Das ist bekannt, aufgenommen als `shared/issues/260815-1047_o_die-directive-der-runde-10-…`, und der Abschnitt gehört weder dem Orchestrator noch dem reconciler. Die Closure-Notiz derselben Datei hält den überholten Stand ausdrücklich fest; wer die Datei liest, wird nicht in die Irre geführt.
