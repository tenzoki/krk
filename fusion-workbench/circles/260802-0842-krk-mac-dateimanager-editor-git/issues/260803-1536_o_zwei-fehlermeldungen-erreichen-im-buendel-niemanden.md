Die beiden Fehlermeldungen der Oberfläche erreichen im Bündel niemanden

---

Zwei echte Fehlerpfade in `krk-ui` melden sich ausschließlich über
`eprintln!`. Ein über den Finder oder über `open` gestartetes Bündel hat keinen
Standardfehler; die Zeilen laufen ins Leere. In der einzigen Betriebsart, die
das Projekt für die Abnahme zulässt, ist die Fehlerbehandlung damit still.

---

## Die beiden Stellen

**Erstens, der Tastenabgriff.** `crates/krk-ui/src/appkit/anwendung.rs:94-104`:

```rust
match abgriff {
    Some(abgriff) => { … }
    // Ohne Abgriff bewegt keine Taste mehr die Auswahl. Das still
    // hinzunehmen hiesse, eine Anwendung auszuliefern, deren erste
    // Maxime die Tastatursteuerung ist und die keine hat.
    None => eprintln!(
        "krk: der Tastenabgriff liess sich nicht einrichten, die Tastatursteuerung bleibt aus"
    ),
}
```

Der Kommentar benennt die Lage genau richtig und wählt dann den einen Kanal, der
sie nicht heilt. Was der Nutzer sieht, ist ein Fenster mit einer Dateiliste, in
der keine Taste etwas tut, und keinerlei Hinweis darauf, warum.

**Zweitens, der unvollständig gelesene Ordner.**
`crates/krk-ui/src/appkit/tabelle.rs:361-364`:

```rust
Meldung::Fertig { abschluss, .. } => {
    if let Abschluss::Fehler(fehler) = &abschluss {
        eprintln!("krk: Ordner nicht vollstaendig lesbar: {fehler}");
    }
```

Der Fall tritt regelmäßig ein, nicht nur exotisch: ein Ordner ohne Leserecht
liefert `Abschluss::Fehler` schon aus `Schwungleser::oeffnen`
(`crates/krk-core/src/verzeichnis/leser.rs:201-204`), also ohne einen einzigen
Eintrag. Der Nutzer steigt mit Return hinein und sieht eine leere Liste, die von
einem wirklich leeren Ordner nicht zu unterscheiden ist. Weil S7 keinen Rückweg
hat, bleibt er dort.

## Dass die Ausgabe wirklich ins Leere geht, ist im Projekt schon festgehalten

`issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md` stellt es
für denselben Prozess fest: "Eine über `open` gestartete Anwendung hat aber
keine: LaunchServices hängt Standardausgabe **und Standardfehler** eines so
gestarteten Prozesses ins Leere."

Jener Defekt betrifft die Abnahmevorschrift des Protokollmodus und schlägt vor,
das Bündel für die Messung unmittelbar aus dem Terminal zu starten. Das ist für
eine Messung richtig und für diese beiden Pfade keine Antwort: sie treffen den
Nutzer im Alltagsbetrieb, und der startet KRK nicht aus einem Terminal. Deshalb
ein eigener Datensatz.

## Was zu tun ist

Der Umfang gehört in eine eigene Abwägung, aber die Richtung ist vorgezeichnet
und braucht keine neue Maschinerie:

- **Der Ordnerfehler** gehört ins Fenster. Die Zeile, in der er ankommt, ist
  `tabelle.rs:362`; die Stelle, an der er sichtbar würde, ist dieselbe Tabelle,
  die gerade leer ist. Eine Statuszeile am Fuß des Dateifensters trägt ihn und
  trägt später auch den Lesefortschritt und die Zahl der Einträge, die C1 ohnehin
  verlangt.
- **Der fehlende Tastenabgriff** ist ein Startfehler und kein Zustand, in dem
  KRK sinnvoll weiterläuft. Ein `NSAlert` beim Start und ein Abbruch ist die
  ehrlichere Antwort als ein Fenster ohne Tastatur.

Beides ist eine Festlegung, keine Reparatur. Wenn sie über den Zuschnitt von
Runde 1 hinausgeht, ist der richtige Ausgang dieses Datensatzes ein
Entscheidungsdatensatz zur Frage, wie KRK dem Nutzer Fehler zeigt, und die
Zuordnung zu dem Schritt, der die Statuszeile baut. Was nicht bleiben sollte,
ist der Zustand, in dem der Programmtext eine Meldung schreibt und niemand sie
liest.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
