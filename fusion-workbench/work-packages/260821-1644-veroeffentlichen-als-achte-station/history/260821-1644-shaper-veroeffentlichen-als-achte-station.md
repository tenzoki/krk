# Shaper: Veröffentlichen als achte Station

**Datum:** 2026-08-21 16:44
**Status:** Complete
**Agent:** shaper (Modus: anticipated-circle, zweite und abschließende Runde)
**Baumstand:** `ca84a59`

## Auftrag

Den Circle für die bereits gefahrene Runde „Artefakt und Release" nachtragen. Die Arbeit ist
gebaut, der Träger des Abschlusses fehlte; festgestellt hat es der Abgleich
`shared/history/260821-1532-reconciliation.md`. Die Vorrunde lag nicht im Gedächtnis dieses
Laufs vor, sondern vollständig im Auftragstext.

## Was der Nutzer entschieden hat

**Der Verzeichnisname.** Von drei vorgelegten Namen hat der Nutzer
`veroeffentlichen-als-achte-station` gewählt, weil er das Gebaute benennt und zur Form der
vierzehn vorhandenen Runden passt, die alle sagen, was entstanden ist.

**Kein zweiter Circle.** Der andere Strang der Sitzung, die Untersuchung zum Lesezeichenverlust
und die zwei Kernkorrekturen `073448e` und `d771ec6`, bekommt keinen eigenen Circle und bleibt im
gemeinsamen Speicher. Defektarbeit zwischen den Runden liegt in diesem Projekt dort, und der
Strang trägt weder Spec noch Plan noch Abnahmekriterien. Er ist im neuen Circle nur an der einen
Stelle genannt, an die er sachlich gehört: der Verlust vom 17.08. ist der Anlass der Runde und
steht deshalb im Grundlagenschnappschuss.

## Was angelegt wurde

`circles/260821-1644-veroeffentlichen-als-achte-station/` mit dem Datensatz `_a_circle.md` und
den sechs Artefaktverzeichnissen `planning/`, `issues/`, `decisions/`, `history/`, `reviews/`
und `analyses/`. Die fünf leeren bleiben leer.

Der Datensatz führt die Kopffelder auf Spec und Sitzungsprotokoll, die Zeigerzeile statt einer
zweiten Fassung der Directive, den Grundlagenschnappschuss, zwei Abhängigkeiten und ein leeres
Turnprotokoll. Eine Schließungsnotiz und ein `**Status:**`-Kopffeld trägt er nicht: die geltende
Vorlage führt keins, und die Notiz entsteht beim Übergang auf einen Endmarker.

## Was gelesen wurde

Der Spec `shared/planning/260821-1115_*_spec-artefakt-und-release.md` ganz, der Abgleich
`260821-1532`, das Shaper-Protokoll der Spec-Runde `260821-1115`, der Kopf der Untersuchung
`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md` und der Circle-Datensatz
der Runde 8 auf die Tagpflicht hin. Dazu die Marker aller sechzehn Circles und der Datensatz der
zuletzt geschlossenen Runde als Formvorlage.

Am Baum nachgemessen statt übernommen: `gh` fehlt (`command -v gh` gibt nichts zurück), lokal
stehen 14 Tags, `d577295` trägt S23 und `f5300f4` die eigenständige Beglaubigung.

## Zwei Abweichungen vom Auftragstext, beide bewusst

**Der Auftrag trug keine `**Draft:**`-Zeile.** Der Moduskontrakt verlangt sie, und ihr Fehlen ist
sonst ein Abbruchgrund. Hier ist der Lauf durchgefahren, weil die Zeile im Modus genau eine
Aufgabe hat: die Directive zu liefern, die sonst erfunden würde. Diese Runde schreibt gar keine
Directive, sondern die Zeigerzeile auf einen vorhandenen Spec, und der Auftrag gibt ihren
Wortlaut vor. Die Fläche, die der Abbruch schützt, ist also leer. Der Abbruch hätte eine Runde
gekostet und nichts erbracht.

**Die Marker in den Pfaden stehen als `_*_` und nicht ausgeschrieben.** Der Auftrag nannte
`260821-1115_o_spec-artefakt-und-release.md`. Der Marker dieses Specs ist der Gegenstand der
offenen Frage `shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-…`, wandert
also voraussichtlich; ein ausgeschriebener Marker wäre ein Zeiger, der bei dieser Umbenennung
stirbt. Der zuletzt geschlossene Circle-Datensatz schreibt seine Kopffelder aus demselben Grund
in Sternform. Die Datei, auf die der Auftrag zeigt, ist unverändert dieselbe.

Alle zwölf Pfadzitate des Datensatzes sind nach dem Schreiben einzeln gegen den Speicher
aufgelöst; alle zwölf treffen. Eine Berichtigung war nötig: der Kurzname der Suchpfad-Entscheidung
lautet `ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist`.

## Nicht getan

Kein Spec, kein Plan, kein Defekt, keine technische Wahl. Nichts verschoben und nichts kopiert:
Spec, Plan, Durchsichten und Protokolle bleiben im gemeinsamen Speicher, weil bei ihrer
Entstehung kein Circle aktiv war. Keine Datei außerhalb des neuen Verzeichnisses angefasst,
insbesondere nicht `.active-circle`. Kein vorhandener Circle geändert. Kein Backlog-Eintrag
berührt, weil der Entwurf aus keinem stammt.

Die Aktivierung, die Kopffelder der Runde, der beschränkte Abschluss und das Nachziehen des
Portfolios gehören dem Orchestrator.
