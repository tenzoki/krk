# Acht Kurzverweise in Spec und Plan der Runde 2 sind ausgeschrieben

**Agent:** ontocoder
**Datum:** 2026-08-11, 11:55
**Status:** Complete
**Quelle:** `shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`
**Domain:** data

## Auftrag

Verweise auf Datensatzdateien, die den Dateinamen mit Auslassungspunkten abkürzen, sind auf
den vollen Namen mit Sternstelle zu ziehen. Umfang: allein
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/`. Kein Code, keine
Circle-Datensätze, kein Commit, keine Markeränderung an den beiden Planungsdateien.

## Der neu erhobene Bestand

Das Suchmuster des Datensatzes findet die Kurzform mit dem Unicode-Auslassungszeichen. Die
Stellen im Baum tragen aber **drei ASCII-Punkte**, nicht `…`. Erhoben mit

```sh
grep -rnoE '26[0-9]{4}-[0-9]{4}_[aoicdspb*]_[a-zA-Z0-9…-]*(\.\.\.|…)' \
  fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/
```

Neun Treffer statt der acht gemeldeten. Die acht des Datensatzes stimmen in Datei und Zeile;
der neunte ist `plan:1587`, der die Sternstelle schon trug und allein im Namen gekürzt war —
deshalb konnte ihn keine Erhebung nach festen Markern führen.

| Stelle | Kurzform | Eingesetzt |
|---|---|---|
| `spec:556` | `260807-0010_o_kann-der-auffrischungsaufschub-entfallen...` | `circles/260802-0842-…/decisions/260807-0010_*_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md` |
| `spec:556` | `260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben...` | `circles/260802-0842-…/decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md` |
| `plan:492` | `issues/260809-2148_c_...` | `issues/260809-2148_*_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md` |
| `plan:690` | `issues/260808-0931_c_...` | `issues/260808-0931_*_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md` |
| `plan:701` | `issues/260808-0931_c_...` | wie oben |
| `plan:716` | `issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...` | `issues/260808-1413_*_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md` |
| `plan:853` | `issues/260809-2148_c_...` | wie oben |
| `plan:884` | `issues/260809-2148_c_...` | wie oben |
| `plan:1587` | `issues/260809-1738_*_...` | `issues/260809-1738_*_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md` |

Jeder volle Name ist gegen den Dateibestand aufgelöst und nicht geraten. Der Zeitstempel
`260808-1413` trägt sechs Datensätze; die Kurzform nannte den Namensanfang und ist damit
eindeutig.

Das Vorbild ist `c0b96a6`, das drei Verweise derselben Liste auf Zeile 1352 ausgeschrieben
hat: der bestehende Verzeichnisanteil bleibt stehen, der Marker wird zur Sternstelle, der
Name wird vollständig, die Endung kommt dazu.

## Zwei Stellen weichen vom reinen Ausschreiben ab

**`spec:556` bekommt einen Verzeichnisanteil.** Die beiden Kurzformen dort standen ohne
Pfad. Ein bloßer Dateiname wird in diesem Projekt relativ zum eigenen Circle gelesen, und
diese beiden Datensätze liegen im Circle der Runde 1; der ausgeschriebene Name ohne Pfad
liefe ins Leere. Der Spec nennt Datensätze der Runde 1 an drei anderen Stellen (526, 527,
667) mit vollem Circle-Pfad, und die Ergänzung folgt dieser eigenen Form.

**`plan:1587` ist nicht Teil der gemeldeten acht** und ist trotzdem mitgezogen, weil er
dieselbe Gestalt und dieselbe Wirkung hat: gekürzter Name, kein `.md`, unsichtbar für jede
Suche über Dateinamen.

## Nichts stehengelassen

Keine der neun Stellen ist eine, an der der Marker die Aussage trägt. Alle neun stehen in
Prosa über einen Sachverhalt, mit dem Datensatz als Beleg. Die Stellen, an denen der Marker
die Aussage ist, stehen im Reconciliation Log und sind unberührt geblieben — bis auf die
zwei Nachträge unten, die selbst Marker ausschreiben, weil sie den Befund benennen.

## Zwei Absätze im Reconciliation Log sind nachgezogen

Beide sind nach der Handhabung dieser Datei mit einem **Nachtrag** versehen und nicht
umgeschrieben; ein Abgleichsbericht hält fest, was zu seinem Zeitpunkt galt.

1. **`### 260810-0805`, Punkt „Sieben Verweise im Plan tragen weiter einen festen Marker".**
   Zwei Angaben waren falsch. Die Zahl zählt Zeilen und keine Verweise: die damalige Zeile
   1330 trug drei Verweise, sieben Stellen waren neun Verweise. Und „Der Spec trägt null"
   stimmt nicht — der Spec trug zwei, beide auf Zeile 556.
2. **`### 260810-1404`, Absatz „Alle Verweise dieses Plans lösen auf".** Die 40 geprüften
   waren nicht alle: sieben standen zu dem Zeitpunkt in der Kurzform und fielen aus dem
   Suchmuster. „Kein toter Verweis" galt damit für die 40 und nicht für den Plan — der
   Verweis auf die damalige Zeile 696 nannte `260808-1413_o_`, während die Datei `_c_` trug.

## Prüfung

- Erneute Erhebung mit dem Muster oben: kein Treffer, Rückgabewert 1.
- Jeder Verweis der Form `<zeitstempel>_<marker>_<name>.md` aus beiden Dateien gegen den
  Dateibestand aufgelöst, Marker offengelassen: kein Fehlschlag.
- `git status`: nur die zwei Planungsdateien geändert. Die Dateinamen und damit die Marker
  `_o_` und `_c_` sind unverändert.

## Was offen bleibt

Der Datensatz ist **nicht** auf `_c_` gezogen und nichts ist committet; beides war
ausgeschlossen. Der Datensatz nennt daneben eine Lehre, die über diese Behebung hinausgeht:
jedes Suchmuster dieses Projekts, das `\.md` verlangt, hat denselben blinden Fleck. Sie ist
in den zweiten Nachtrag mit aufgenommen, aber an keiner Stelle außerhalb dieses Circles
umgesetzt.
