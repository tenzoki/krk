# Shaper: der Entwurf „ReaderConventions" wird ein vorgesehener Circle

**Datum:** 2026-08-23
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete

## Der Entwurf

Eingabe war der Backlogeintrag `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`. Er beschreibt Profile in einer Definitionsdatei `krk-rc.yaml`, die den Zugriff auf Ordner und Dateien vereinfachen, und nennt dafür zwei Hälften: welche Orte welche Leseoperationen erfordern, und was im Vorschaufenster erscheint. Die zweite Hälfte ist im Eintrag ausgeführt, mit sechs skizzierten Zusammenfassungen für die fusion-workbench; die erste steht als Halbsatz.

## Die Klärungsrunde

Vier Fragen, alle vom Nutzer beantwortet:

1. **Was gehört in die Runde?** Nur die Zusammenfassung. Die Leseoperationen bleiben draußen.
2. **Was darf eine Profilregel ausrechnen?** Einen festen Bausteinsatz, keine eigene Ausdruckssprache.
3. **Woran erkennt ein Profil seinen Ort?** Beides, Pfadmuster und Kennzeichendatei. Der Nutzer hat die Vorrangregel selbst festgelegt: das Pfadmuster geht vor, und ohne Treffer bleibt die heutige Metadatenanzeige stehen.
4. **Wo steht die Definitionsdatei?** In einer eigenen `readers.toml`, nicht in `settings.toml`. Damit fallen Name und Format des Entwurfs, `krk-rc.yaml`, zugunsten des Formats, das der Bestandsort ohnehin trägt.

## Was entstanden ist

Der Circle `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/` mit dem Datensatz `_a_circle.md`, den sechs Artefaktverzeichnissen und zwei offenen Entscheidungsdatensätzen unter `decisions/`: ob ein Profil nur für Ordner oder auch für einzelne Dateien gilt, und ob KRK ein fertiges fusion-workbench-Profil mitliefert. Beide binden den Spec, der bei der Aktivierung entsteht.

Der Grounding-Abschnitt hält sieben Befunde aus dem Baum fest, darunter zwei, die den Zuschnitt der Runde berühren: die Zusammenfassung fällt in die Endbedingung der Zeitzusage L7 („sonst die Metadaten", 100 ms), und für die Ortserkennung über ein Pfadmuster gibt es im Baum noch keinen Mustervergleich.

## Was nicht geschehen ist

Der Backlogeintrag ist **nicht** als übernommen geschlossen. Er nennt zwei Hälften, und diese Runde nimmt nur eine; eine Schließung nähme die Leseoperationen ungelesen mit. Der Eintrag steht unverändert auf `_o_`.
