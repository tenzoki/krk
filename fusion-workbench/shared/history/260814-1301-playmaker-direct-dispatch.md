# Playmaker — Portfolio-Lauf nach dem Abschluss der Runde 9

**Status:** Complete
**Gefahren am:** 260814-1301
**Sitzung:** 260814-1301-playmaker-direct-dispatch
**Auslöser:** direct-dispatch durch den Nutzer, nach dem `_t_` → `_b_` der Runde 9
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Portfolio:** `fusion-workbench/portfolio.md` (vollständig neu erzeugt)

## Bestand

Zehn Circle-Datensätze, gezählt am 260814-1301:

| Marker | Anzahl | Circles |
|---|---|---|
| `_t_` aktiv | 0 | — |
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_c_` kohärent | 1 | `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` |
| `_b_` beschränkt | 8 | die Runden 1 bis 7 und 9 |
| `_s_` / `_d_` | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss; keine der vier Zeigerwarnungen ist ausgelöst worden.

Neu gegenüber dem Lauf vom 260813-2203 ist ein Datensatz:
`260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` trägt seit dem 260814-1300 `_b_`.

## Rangfolge der vorgesehenen Circles

**Rang 1 und einziger Kandidat:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Eine Rangfolge mit einem Element ist keine Auswahl; die Empfehlung stützt sich auf absolute
Signale. Zwei haben sich seit dem 260813-2203 bewegt: der Auslieferungsweg wird wieder angehalten
(kein Tag auf `HEAD`, zwölf Commits seit `v0.2.1`), und die Runde 9 hat gezeigt, dass eine
Handabnahme des Nutzers allein für einen kohärenten Abschluss nicht reicht — es fehlte die Bindung
der Beobachtungen an die Abnahmekriterien.

Die Prüfung „alle Abhängigkeiten kohärent abgeschlossen" ist für diesen Circle nicht eingerechnet:
seine einzige Kante führt auf die Runde 1, die `_b_` trägt, und ein Endzustand wird nicht
zurückgenommen.

## Ideenspeicher

- Gelesen: drei Einträge unter `shared/backlog/`. Ein `_o_`, kein `_p_`, zwei `_c_`.
- Unterschiedene Ideen in den offenen Einträgen: 1. Kein Split vorgeschlagen.
- Dublettengruppen: 0. Mit einem einzigen offenen Eintrag ist die Prüfung gegenstandslos.
- An `## Warnings` abgegeben: 1 Befund, defektförmig (Punkt 2 des Portfolios).

**Rang 1:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— die selbstgestellte Vorbedingung des Eintrags ist beantwortet: die Systemeinstellung für die
Funktionstasten erklärt die Hakeligkeit nicht (gemessen am 260802-1137, KRK belegt den Tastencode
und kann `fn` nicht sehen), und die Runde 9 hat für `notizzettel` mit `f2` und `cmd+k` denselben
Schritt gerade gebaut. Der Eintrag ist damit shapebar, ohne eine Untersuchung davor.

Der zweite Eintrag vom 260813, das Scratchpad, steht seit dem 260813-2334 auf `_c_`: aus ihm ist
die Runde 9 geworden. Der Lauf vom 260813-2203 hatte ihn auf Rang 1 empfohlen.

## Warnungen im Portfolio

1. Die Runde 9 nennt einen Weg zu einem kohärenten Abschluss (zweite Abnahmeliste, 21 unbelegte
   Kriterien, rund zwanzig Minuten), und der Marker geht ihn nicht mit: `_b_` ist ein Endzustand.
2. Der offene Ideeneintrag beschreibt zur Hälfte einen Defekt: `f4` ist die einzige Funktion der
   Norton-Reihe mit nur einem Weg, obwohl der umgesetzte Nutzerentscheid vom 260802-1409 „F4
   Bearbeiten" unter seinen sechs nennt.
3. `CLAUDE.md` beschreibt vier Runden, es sind neun. Zweiter Lauf in Folge.
4. Doppelt belegter Ausgabeort von `cargo xtask bundle` und `cargo xtask release`, bei der Abnahme
   der Runde 9 zum ersten Mal praktisch getroffen und von Hand umgangen.
5. 89 offene Defekte, 11 im gemeinsamen Speicher, 18 aus der Runde 9.
6. 19 offene Entscheidungsdatensätze, einer beantwortet und nicht umgesetzt. Unverändert.
7. Kein Abhängigkeitszyklus.
8. Ein Vermerk zu gealterter Grundlage angehängt.
9. Der Datensatz des Web-Betrachters trägt 914 Zeilen und dreizehn Playmaker-Abschnitte.

## Angehängte Abschnitte

- `## Dependency warning`: keine. Der gerichtete Graph über die nicht terminalen Circles hat einen
  Knoten und keine Kante innerhalb dieser Menge.
- `## Parent grounding stale`: einer, an
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`.
- `## Activation proposal`: einer, an demselben Datensatz.

## Ereignisse

```
parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster child=260813-2332-notizzettel-als-blatt-mit-zwei-zetteln
```

Zur Auslösebedingung: erfüllt ist die eine Hälfte, das Kind trägt `_b_`. Der Abschnitt
`## Grounding snapshot` des Elternteils stammt vom 260804 und zitiert das Kind nicht. Der Vermerk
steht trotzdem, wie in den vier Läufen davor, weil die Runde 9 an zwei Stellen gearbeitet hat,
durch die jeder Befehl des Betrachters laufen wird: die Ausnahme im Ersthelfervorbehalt hat jetzt
einen gebauten Präzedenzfall in beide Richtungen (`crates/krk-ui/src/appkit/blaetter/zettel.rs`,
Modulkopf), und ein neuer Befehl kommt beim Nutzer mit eigener `keymap.toml` unbelegt an
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

## Stilprofile

`chat-voice-de.yaml` und `default-voice-de.yaml` sind beide vorhanden und angewandt. Keine Lücke zu
melden.
