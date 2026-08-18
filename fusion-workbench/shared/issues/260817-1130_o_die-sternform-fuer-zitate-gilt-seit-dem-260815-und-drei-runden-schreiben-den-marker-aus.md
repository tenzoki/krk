Die Sternform für Zitate gilt seit dem 260815, und drei Runden schreiben den Marker weiter aus

---

`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`
trägt den Marker `_i_`: der Nutzer hat am 260815-1230 die Sternform `_*_` gewählt, und der
Commit `e49412a` hat 163 Zitate darauf gebracht. Der Datensatz benennt seinen Geltungsbereich
wörtlich: „Umgestellt wird, was heute gilt: `crates/`, `xtask/`, `CLAUDE.md`, die
Circle-Datensätze und die Spec- und Plandateien unter `planning/`."

Seither sind drei Runden gefahren, und ihre Spec-, Plan- und Circle-Dateien schreiben den
Marker wieder aus. Am 260817-1129 über den lebenden Text gezählt: **52 ausgeschriebene
Zitate** in zehn Dateien, davon 47 in Artefakten, die nach `e49412a` entstanden sind.

---

**Die Zählung nach Datei.** Gezählt sind Zeichenfolgen der Form `YYMMDD-HHMM_x_thema` im
lebenden Text, ohne Selbstverweise einer Datei auf den eigenen Namen:

| Datei | Zahl |
|---|---|
| `shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md` | 7 |
| `circles/260816-1321-…/planning/260816-1359_*_plan-inhaltsfilter-der-dateiliste.md` | 10 |
| `circles/260816-2255-…/planning/260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md` | 9 |
| `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md` | 6 |
| `circles/260817-0833-…/_t_circle.md` | 6 |
| `circles/260816-2255-befehle-absetzen-und-makros-speichern/_d_circle.md` | 5 |
| `circles/260817-0833-…/planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md` | 4 |
| `shared/planning/260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md` | 3 |
| zwei Pläne der Runden 2 und 7, aus der Zeit vor der Entscheidung | 2 |

**Ein Teil davon ist die ausdrückliche Ausnahme und kein Verstoß.** Der Datensatz nimmt die
Stellen aus, an denen der Marker die Aussage selbst ist. Das trifft mindestens die zwei
Zeilen des Plans der laufenden Runde, die den Wechsel eines Markers beschreiben (Schritt 16:
der Datensatz „wandert von `_i_` auf `_s_`"; Schritt 17: „nennt den Pfad … mit dem Marker
`_o_`; die Datei trägt `_c_`"). Die übrigen sind reine Zeiger und fallen unter die Regel.

**Der Schaden ist schon eingetreten.** Drei der 52 Zitate zeigen bereits auf einen Marker,
den ihr Ziel nicht mehr trägt: `260802-1036_o_spec-navigator-geruest.md` heißt seit der
Schließung der Runde 1 `260802-1036_c_…`, und der falsche Marker steht in
`shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md`, in
`shared/decisions/260817-0536_*_bekommt-f8-den-papierkorb-…` und in einem Sitzungsprotokoll.
Die beiden ersten sind lebender Text; das dritte fällt unter die Ortsregel und bleibt stehen.
Genau diese Alterung war der Grund der Entscheidung: sie hatte 52 von 111 Zitaten erfasst,
also 47 Prozent, bevor `e49412a` sie berichtigte.

**Die Ursache steht im Datensatz selbst.** Er hält fest, dass die Antwort „**ohne** die
Prüfung im Bau" angenommen wurde, und nennt die Folge: „ohne Prüfung im Bau bemerkt weiterhin
niemand ein totes Zitat, bis jemand ausdrücklich danach sucht." Der jetzige Befund ist der
erste Beleg dafür, dass die Verabredung ohne Prüfung nicht hält, und zwar über drei Runden
und mehrere Bearbeiter hinweg.

**Was zu tun bleibt, ist zwei Dinge und nicht eins.** Die 52 Stellen einmal umzustellen ist
der kleinere Teil; der größere ist die Frage, ob die Verabredung ohne Prüfung ein zweites Mal
gefahren wird. Beides gehört an den Nutzer und nicht an den Abgleich: das erste ist eine
Textänderung an fremden Runden, das zweite die Wiederaufnahme einer beantworteten Frage.

**Herkunft:** neben der Arbeit am Circle
`260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` gefunden. Der Befund
reicht über drei Runden und geht keine ihrer Directives an, deshalb im gemeinsamen Speicher.

**Gefunden von:** reconciler, Abgleich 260817-1129
**Domain:** code

---
Also seen: 260818-0024 by coderev — der Schritt 17 der laufenden Runde (`da716c1`) hat elf weitere
ausgeschriebene Marker in zwei Datensätze der Runde 1 eingetragen, sechs davon reine Zeiger und
damit unter der Regel: `_b_circle.md:18`, `:66`, `:93` und
`planning/260802-1036_c_spec-navigator-geruest.md:12`, `:278`, `:281`, jeweils
`260817-0536_i_wie-wird-jeder-loeschweg-abgesichert-…` in Sätzen der Form „Bindend ist …" und
„An seine Stelle tritt …". Die übrigen fünf nennen `260802-0842_s_loeschen-papierkorb-oder-endgueltig`
in der Form „steht als überholt (…)", wo der Marker die Aussage selbst ist; sie fallen unter die
ausdrückliche Ausnahme. Die Commit-Nachricht von `24bbccc` begründet die Schreibweise eigens,
wägt dabei aber `_i_` gegen `_a_` ab und nicht gegen die Sternform — die Entscheidung vom
260815-1230 war dem Bearbeiter nicht gegenwärtig. Das ist die vierte Runde nach `e49412a`.

Also seen: 260818-0710 by reconciler — die vorhergesagte Alterung ist eingetreten und gemessen:
`circles/260817-0833-…/issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`
zählt 22 tote Zeiger in sechs Dateien lebenden Textes, alle auf die fünf Entscheidungsdatensätze,
die Schritt 16 der laufenden Runde in `24bbccc` bewegt hat. Fünf davon stehen in `crates/`, dem
ersten Geltungsbereich, den die Entscheidung vom 260815-1230 nennt. Der 23. tote Zeiger ist der
hier schon gemeldete auf `260802-1036_*_spec-navigator-geruest.md`. Damit sind es nicht mehr drei
von 52, sondern 23 in einem einzigen Durchgang.
