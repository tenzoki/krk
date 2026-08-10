# Ein Befehl während der Nachfrage aus C4 wird von der Antwort still überschrieben

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Behebung von `260810-1029`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs` (`nachfrage_zeigen`,
`nachfrage_beantworten`), `crates/krk-ui/src/appkit/ereignisse.rs`
(`ersthelfer_gehoert_appkit`)
**Cross-references:** `issues/260810-1029_*_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`,
`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`,
C2, C4

---

## Der Befund

Der Befund ist derselbe wie `260810-1029`, eine Schicht darüber: **zwei
Öffnungen sind offen, und die zuletzt begonnene verliert.** Behoben ist der
laufende Ladevorgang; unbehoben ist die gelesene Datei, die auf die Nachfrage
aus C4 wartet.

`Editormodell::zurueckgehalten` überlebt jeden weiteren Befehl. Die Abkürzung
`SchonOffen` gibt seit dem 260810 den Ladevorgang auf, fasst dieses Feld aber
nicht an — richtig so, denn die Antwort des Nutzers auf ein offenes Blatt läuft
über `zurueckgehaltenes_uebernehmen` zurück, und ein Feld, das das Modell selbst
geleert hätte, ließe diese Antwort still ins Leere greifen. Der Ort der Behebung
ist deshalb nicht das Modell.

## Der Ablauf, in dem es sich zeigt

Der Editor hält B mit ungesicherter Änderung. Der Nutzer wählt A im
Dateifenster und drückt F4.

1. A wird gelesen und geprüft, `uebernehmen_oder_zurueckhalten` liefert
   `Zurueckgehalten`, und das Blatt aus C4 steht: sichern, verwerfen, abbrechen.
   Genannt ist B, die Datei, deren Stand auf dem Spiel steht. Richtig.
2. Der Nutzer wählt jetzt B im Dateifenster und drückt F4 — statt zu antworten.
   `Editormodell::oeffnen` liefert `SchonOffen`, der Editor wird hervorgeholt,
   Fokus, Titel und Sitzung tragen B.
3. Danach antwortet er dem Blatt, das immer noch steht. "Verwerfen" nimmt A auf
   und wirft B fort; "Sichern" schreibt B und nimmt dann A auf. In beiden Fällen
   hält der Editor am Ende A, obwohl der letzte Befehl des Nutzers B verlangte.

Der Schaden ist derselbe wie in `260810-1029`: kein Text geht verloren, wohl
aber die Wirkung des letzten Befehls. Erreichbar ist die Spanne, solange das
Blatt steht, und das ist im Gegensatz zu `260810-1029` keine Frage von
Millisekunden, sondern eine von der Bedenkzeit des Nutzers.

## Warum der Befehl das Blatt überhaupt erreicht

Das ist der eigentliche Fund, und er reicht über den Editor hinaus. **Der
Fokusvorbehalt hält Tastenbefehle nicht an, solange ein Blatt steht, sondern nur
dann, wenn der Ersthelfer eine Textklasse ist.** `ersthelfer_gehoert_appkit`
(`crates/krk-ui/src/appkit/ereignisse.rs:497`) fragt den Ersthelfer des
Schlüsselfensters und liefert `true` allein für `NSTextView`, `NSTextField` und
`NSText`. Das Blatt aus C4 trägt drei Schaltflächen und kein Textfeld
(`crates/krk-ui/src/appkit/blaetter/ungesichert.rs:77-85`), sein Ersthelfer ist
eine Schaltfläche, und der Tastendruck läuft in den Nachschlag.

`inference:` — geprüft ist der Code, nicht der laufende Fall. Der Ablauf oben
ist am Code gelesen und nicht am Bündel gemessen; messen kann ihn nur der
Nutzer, weil der Abnahmelauf KRK im Vordergrund verlangt.

Der Modulkopf von `ereignisse.rs` beschreibt den Vorbehalt als "Textfelder und
Blätter behalten ihre AppKit-Bedeutung" und nennt die fünf Blätter aus S16 und
S17 als Erben. Der Code löst davon die erste Hälfte ein. Ein Blatt ohne Textfeld
erbt nichts, und `offenes_blatt` (`anwendung.rs:452`) wird allein dafür geführt,
dass `esc` das Blatt schließt, nicht dafür, Befehle anzuhalten. Der Defekt
`260804-1122` hat denselben Vorbehalt schon einmal als zu eng gemeldet, damals
für die Löschtasten.

## Der Weg dahin, ungeprüft

Zwei Möglichkeiten, und die Wahl gehört nicht in diese Behebung:

1. **Der Fokusvorbehalt hält jeden Befehl an, solange ein Blatt steht.** Eine
   Stelle, alle fünf Blätter, und die Aussage des Modulkopfs wäre eingelöst. Der
   Preis ist die Reichweite: der Wechsel des Ordners hinter einem offenen Blatt
   fiele damit ebenfalls weg, und ob das gewollt ist, sagt kein
   Abnahmekriterium.
2. **Der Anwendungsdelegierte lässt die zurückgehaltene Datei fallen, wenn ein
   neuer Öffnungsbefehl kommt**, und schließt das Blatt mit ihr. Enger
   geschnitten, aber es ist eine Regel je Anlass statt einer für alle Blätter.

Ungeprüft ist außerdem, ob dieselbe Spanne bei den vier übrigen Blättern einen
Schaden trägt. Der Fund oben sagt allein, dass Befehle sie erreichen.

## Warum es hier nicht behoben wurde

Die Behebung von `260810-1029` war auf `crates/krk-ui/src/editormodell.rs`
begrenzt. Beide Möglichkeiten oben liegen in `crates/krk-ui/src/appkit/`, an dem
zur selben Zeit ein anderer Agent arbeitete.
