# Shaper: der Git-Bereich als vorgesehener Circle

**Date:** 2026-08-30 10:45
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
**Dispatched by:** `/fusion:direct`

## Auftrag

Ein Entwurf des Nutzers: KRK bekommt einen Git-Bereich als sechsten Bereich der Fensterzeile, lesend, mit `gix`. Der Entwurf verwies auf die frische Machbarkeitsanalyse `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` und auf fünf Entscheidungsdatensätze unter demselben Stempel, die in die Klärung gehören sollten.

## Gelesen

Die Analyse ganz, alle fünf Entscheidungsdatensätze, `CLAUDE.md`, der Bestand unter `circles/` und `shared/backlog/`. Der Entwurf kam als Fließtext und nicht als Verweis auf einen Backlog-Eintrag; es ist deshalb keiner geschlossen worden.

## Klärung

Eine Runde mit vier Fragen und drei angekündigten Vorbelegungen. Der Nutzer hat mit `1a, 2a, 3a, 4b` geantwortet und keiner Vorbelegung widersprochen.

| Frage | Antwort |
|---|---|
| Nimmt der Git-Bereich den Tastaturfokus? | Ja, sechster Wert `Fokus::Git`; die zehn Nachzugsstellen samt der vier stillen werden bezahlt |
| Wie stehen die Git-Marken in der Dateiliste? | Als fünfte Spalte neben Name, Größe, Datum, Typ, mit eigener Überschrift und Breite |
| Was zeigt KRK ohne Repository? | Alles ruhig: ein Satz im Bereich, Ankreuzfeld eingeschaltet und wirkungslos, Spalte leer |
| Welche Zustände unterscheidet eine Marke? | Fünf: geändert, vorgemerkt, neu, in Konflikt, umbenannt |

Die drei stehenden Vorbelegungen: der Git-Bereich folgt dem aktiven Dateifenster und die Marken gelten in beiden Dateifenstern; die Stufe A schreibt den aufgefrischten Index nicht zurück; der Git-Bereich wird dritter Bewerber um die Fläche von Vorschau und Editor und bekommt keine eigene Stelle in der Zeile.

Der Nutzer hat ausdrücklich festgehalten, dass `3a` und `4b` zusammen heißen: die fünfte Spalte steht auch in einem Ordner ohne Repository und bleibt dort leer.

## Zweite Runde: nicht gefahren, und warum

Zwei der fünf Datensätze sind unbeantwortet geblieben, `wohnt-die-git-anbindung-in-krk-core-oder-in-einer-fuenften-kiste-krk-git` und `wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht`. Beide sind bewusst nicht vorgelegt worden. Die erste entscheidet über Modulgrenzen und Kistenzuschnitt, die zweite über den Wortlaut einer Zusage im Baum; keine der beiden ändert, was der Nutzer am fertigen Programm sieht. Sie gehören damit vor den Plan und nicht vor die Direktive. Beide bleiben `_o_` und stehen im `## Grounding snapshot` des Circle-Datensatzes als das, was die Aktivierung zu beantworten hat.

## Geschrieben

- `circles/260830-1045-git-bereich-liest-status-branch-verlauf/` mit `_a_circle.md` und den sechs Artefaktverzeichnissen
- `shared/decisions/260830-1006_o_bekommt-der-git-bereich-…` nach `_a_`, mit `Answered:`-Zeile
- `shared/decisions/260830-1006_o_was-zeigen-git-bereich-ankreuzfeld-…` nach `_a_`, mit `Answered:`-Zeile
- diese Datei

Kein Spec, kein Plan, keine Aktivierung. Der Circle trägt `_a_` und wartet auf `/fusion:next`.
