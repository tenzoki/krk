Zwei Verweise in lebenden Dokumenten der Runde 6 tragen einen ausgeschriebenen Marker, den ihr Ziel nicht mehr hat

---

Zwei Zeigerstellen dieser Runde nennen ihr Ziel mit ausgeschriebenem
Zustandsmarker, und beide Ziele haben den Zustand seither gewechselt:

| Stelle | zitiert | ist heute |
|---|---|---|
| `_t_circle.md:7` (`**Active spec/plan:**`) | `planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` | `_c_` |
| `planning/260812-1145_c_…:366` | `decisions/260812-1145_o_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md` | `_i_` |

Beide Pfade laufen ins Leere: eine Datei dieses Namens gibt es nicht mehr.

---

**Maschinell erhoben** am 260812-2253 über alle Verweise der Form
`YYMMDD-HHMM_X_slug` im ganzen Baum, **mit und ohne** Endung `.md`. Das
Suchmuster ist ausdrücklich um die Kurzform erweitert worden, weil
`CLAUDE.md` festhält, dass jedes Muster mit `\.md` einen blinden Fleck hat und
fünf frühere Erhebungen dieselben acht Stellen nicht gesehen haben
(`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`).

**Der Ort entscheidet, und beide liegen außerhalb der Ausnahme.** `CLAUDE.md`
nimmt Aufzeichnungen eines Standes je Datei nach ihrem Ort aus: `history/`,
`reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`.
Der Circle-Datensatz und `planning/` stehen nicht darunter. Für `planning/` ist
das an diesem Projekt schon zweimal entschieden worden
(`shared/issues/260810-1746_*_spec-und-plan-der-runde-2-tragen-sechs-verweise-mit-ausgeschriebenem-zustandsmarker.md`,
geschlossen).

**Dieselbe Klasse ist in dieser Runde schon einmal aufgetreten** und behoben
worden: `issues/260812-1920_*_ein-verweis-im-ueberholten-statuszeilen-datensatz-zeigt-auf-einen-nicht-mehr-vorhandenen-dateinamen.md`.
Seine Abschlussnotiz nennt die Abhilfe und ihren Grund: die Sternform aus
`rules/circle-records.md`, Abschnitt `### Citation form in the portfolio`. Ein
ausgeschriebener Marker ist ein Zeiger, der beim ersten Zustandswechsel seines
Ziels stirbt. Beide Stellen oben sind so gestorben, die erste beim Abschluss des
Plans, die zweite beim Umsetzen des Rechtsklick-Entscheids.

**Abhilfe:** an beiden Stellen `_*_` schreiben. Der Circle-Datensatz gehört dem
Orchestrator, der Plan dem Planner; der reconciler schreibt in keinen von
beiden hinein.

**Ein Nebenbefund derselben Erhebung**, außerhalb dieser Runde und deshalb
getrennt abgelegt: der Circle-Datensatz der Runde 5 trägt sieben solcher Stellen
(`shared/issues/260812-2253_*_sieben-verweise-im-circle-datensatz-der-runde-5-tragen-einen-gestorbenen-marker.md`).

**Gewicht:** niedrig einzeln, mittel als Klasse — sie wächst mit jeder Runde.

**Herkunft:** Abgleich der Runde 6 am 260812-2253.
