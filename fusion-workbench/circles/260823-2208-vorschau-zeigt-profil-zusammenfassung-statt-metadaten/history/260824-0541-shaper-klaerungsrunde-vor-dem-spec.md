# Shaper: die Klärungsrunde vor dem Spec der Runde 16

**Datum:** 2026-08-24
**Agent:** shaper (user-direct, aktiver Circle in Bereich)
**Status:** Complete — angehalten mit drei Fragen an den Nutzer, kein Spec geschrieben

## Was eingegeben war

Der Circle-Datensatz `_t_circle.md` mit der geschärften Directive und der Grundlagenaufnahme, die zwei am 260824-0530 beantworteten Entscheidungsdatensätze (nur Ordner; mitgeliefertes und wirksames fusion-Profil), die vier Festlegungen der Klärungsrunde vom 260823 und der Backlogeintrag `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md` mit den sechs skizzierten Zusammenfassungen.

## Was am Baum erhoben ist, Stand `278a008`

- `krk-ui/src/vorschaumodell.rs`: `Inhalt` trägt fünf Werte ohne Auffangzweig, `laden` gibt für jeden Eintrag, der keine gewöhnliche Datei ist, `Inhalt::Metadaten` zurück. Das ist die Anzeige, die ein Profil ersetzt.
- `krk-core/src/ablage/pfade.rs`: `Datei::ALLE` führt sechs Einträge, `Datei::format` und `Datei::leerbefund` sind vollständige Fallunterscheidungen ohne Auffangzweig. `readers.toml` wird an drei Stellen die siebte, und der Übersetzer fordert alle drei ein.
- `krk-core/src/ablage/einstellungen.rs`: `AUSLIEFERUNGSTEXT` über `include_str!`, `anlegen_falls_fehlt` schreibt beim ersten Start wörtlich über `atomar::schreiben` und fasst die Datei danach nie wieder an. Das ist der Weg, den `readers.toml` nach dem Entscheid vom 260824-0530 nimmt.
- Fehlerweg: `Geladen<T>` trägt `Option<Ersetzung>`, `Grund` unterscheidet `NichtLesbar`, `Beschaedigt`, `NichtAnlegbar` und `ZuGross`, und `krk-ui/src/appkit/anwendung.rs:1510` sammelt die Meldungen beim Start in die Statuszeile.
- `Cargo.lock` führt weder eine Kiste für reguläre Ausdrücke noch eine für Mustervergleiche noch eine für JSON. Ein regulärer Ausdruck in `readers.toml` kostet deshalb eine fremde Kiste oder eine eigene Maschine.
- Der Beispielbestand an der Wurzel: `.fusion-setup` ist einzeiliges JSON, `.active-circle` eine nackte Zeile, `agentstate.yaml` stand am 260824-0541 gar nicht da.

## Warum kein Spec entstanden ist

Drei Fragen an den vier Bausteinen sind so unbestimmt, dass jedes Abnahmekriterium darüber von ihrer Antwort abhängt. Der sechste skizzierte Fall, die Zusammenfassung eines einzelnen Circles, bekommt ohne die erste Antwort überhaupt kein Kriterium: die Directive ist ein mehrzeiliger Abschnitt, und ob der Baustein „ein Feld aus einer Datei" ihn trägt, ist offen. Ein Spec auf einer geratenen Bausteinform wäre bei einer anderen Antwort nicht zu ergänzen, sondern neu zu schreiben.

## Was entstanden ist

Drei Entscheidungsdatensätze unter `decisions/` dieses Circles, jeder mit zwei oder drei ausgearbeiteten Möglichkeiten, ihren Kosten und einer Empfehlung:

- `260824-0541_o_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`
- `260824-0541_o_was-heisst-die-juengsten-zehn-und-was-ist-ihr-titel.md`
- `260824-0541_o_was-zeigt-die-zusammenfassung-wenn-ein-baustein-ins-leere-greift.md`

## Was ohne Rückfrage entschieden ist und im Spec als abgeleitet steht

Vier Festlegungen sind aus den vorliegenden Antworten abgeleitet und am Spec-Tor überstimmbar. Welches Profil gewinnt, wenn mehrere passen: das erste in der Datei, unter den Pfadmustertreffern zuerst und erst danach unter den Kennzeichendateitreffern, was die Vorrangregel des Nutzers vollständig und überschneidungsfrei ausbuchstabiert. Dass die Zählung flach über einen Ordner läuft und nicht über seinen Unterbaum, weil alle sechs skizzierten Zusammenfassungen mit einer Ebene auskommen. Dass die Zusammenfassung wie die Metadaten heute allein beim Auswählen entsteht. Dass sie eine Obergrenze gelesener Einträge trägt, nach dem Vorbild der zwei Grenzen, die die Vorschau schon führt.

## Nächster Schritt

Der Nutzer beantwortet die drei Fragen. Danach ist der Shaper erneut zu beauftragen; der Spec entsteht dann unter `planning/` dieses Circles.
