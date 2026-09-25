# Planung: der Inhaltsfilter der Dateiliste (elfte Runde)

**Datum:** 2026-08-16
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Baumstand:** `eae7b1c`

## Was entstanden ist

- `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md` — zwölf Schritte in sieben
  Strängen, drei Mermaid-Bilder, je Schritt eine Zeile „woran abzulesen".
- `decisions/260816-1359_a_welche-aussage-schreibt-die-dateizelle-…` — beantwortet, Möglichkeit 1:
  die Markierung schreibt die Zelle, die Dämpfung weicht, gedämpft wird mit `secondaryLabelColor`.
- `decisions/260816-1359_a_in-welcher-reihenfolge-stehen-die-satzteile-…` — beantwortet,
  Möglichkeit 1: Kern, Lesehinweis, Größenhinweis, Markierungshinweis; gekürzt wird von AppKit,
  und der vorhandene Kurzhinweis trägt den vollen Satz.
- `decisions/260816-1359_o_beendet-ein-tabwechsel-den-durchlauf-…` — offen, Nutzerfrage. Der Spec
  verlangt in C4.5 den Abbruch beim Tabwechsel, die Runde 10 hat das Gegenteil gebaut und
  begründet. Der Plan fährt auf der Empfehlung.
- `issues/260816-1359_o_die-probe-gegen-zeitmessung-im-filter-erreicht-zwei-dateien-…` — die Probe
  `im_filter_steht_keine_zeitmessung` nennt fünf Dateien, der Filterweg reicht über sieben, und
  `verzeichnis/sys.rs` kann der Liste nicht beitreten, weil es `Duration` zur Umrechnung der
  Änderungszeit führt und die Nadel Umrechnung nicht von Messung trennt.

## Die vier tragenden Entwurfsentscheidungen

**Der Inhaltsfilter bekommt keine zweite Maschine, sondern eine zweite Auftragsart im Durchlauf
der Runde 10.** Faden, Kanal, Abbruchkennzeichen, `Befund` mit drei Werten und der Einzugstakt
stehen bereits und beantworten dieselbe Art Frage.

**Der Leseweg zieht die private Hülle der Vorschau nach `krk-core`**, statt eine dritte zu
schreiben. `text::datei::bis_zur_grenze_lesen` nimmt die Grenze als Argument; die 1 MB bleiben in
`krk-ui` und reisen über `Durchlauf::starten` als `Option<u64>` herein. `text::datei::lesen` bleibt
unberührt, weil es den offenen Deskriptor zurückgibt.

**Der Vergleich bekommt seinen dritten Rufer in einer eigenen Datei**, `verzeichnis/inhalt.rs`, und
die Zählprobe steigt bewusst von zwei auf drei. Die Alternative wäre gewesen, den Inhaltsvergleich
in `durchlauf.rs` zu schreiben und die Zahl bei zwei zu lassen; das mischte „lies eine Datei" mit
„schreite ein Verzeichnis ab".

**Gelesen wird die ganze Datei und nicht streifenweise.** „Ist das Text" beantwortet dieser Baum
mit `String::from_utf8` über die gelesenen Bytes; streifenweise müsste die Frage je Streifen
beantwortet werden, und eine Datei, die erst spät ungültig wird, hätte aus ihren ersten Streifen
schon Treffer gemeldet. C1.6 verlangt, dass sie gar nicht steht.

## Was nicht geplant wurde

Keine elfte Zeitzusage und keine Messstrecke. Der Nutzer hat das am 260816 entschieden; an ihre
Stelle treten zwei ohne Messstrecke prüfbare Kriterien, und der Inhaltsdurchlauf ist als fünfter
Gegenstand einer späteren Messrunde benannt.

Kein Schritt braucht `analyst`: die Runde erzeugt keinen strategischen Datensatz, den nicht der
Planer selbst geschrieben hätte.
