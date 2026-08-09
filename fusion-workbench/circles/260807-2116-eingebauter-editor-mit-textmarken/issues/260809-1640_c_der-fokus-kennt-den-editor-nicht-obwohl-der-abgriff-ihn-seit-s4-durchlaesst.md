# `Anwendungsdelegierter::fokus` kennt den Editor nicht, obwohl der Abgriff ihn seit S4 durchlässt

---
**Domain:** code
**Schwere:** High
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:2116-2159`
**Cross-references:** `crates/krk-ui/src/appkit/ereignisse.rs:443-459`, `crates/krk-ui/src/kommandos/fokus.rs:157-176`, `crates/krk-core/src/tasten/belegung.rs:194-212` (`Wirkungsbereich::Navigator`), S4, S5, S16, S17

---

## Der Befund

S4 hat den Fokusvorbehalt für die Textfläche des Editors geöffnet, S16 hat die
Textfläche gebaut und in die Aufteilung gehängt. Die Gegenseite fehlt:
`Anwendungsdelegierter::fokus` liefert **niemals** `Fokus::Editor`.

```rust
// anwendung.rs:2149-2158
let in_der_vorschau = self.ivars().vorschau.get().is_some_and(|vorschau| { … });
if in_der_vorschau {
    Fokus::Vorschau
} else {
    Fokus::Dateifenster
}
```

Die Funktion fragt die Leiste ab und die Vorschau. Den Editor fragt sie nicht,
obwohl der Delegierte ihn seit S16 in `ivars().editor` hält und mit
`ist_editorflaeche` (`anwendung.rs:1245-1250`) genau den Vergleich schon
geschrieben hat, der hier fehlt.

Damit steht die Schreibmarke in der Textfläche, und `fokus()` antwortet
`Fokus::Dateifenster`.

## Warum das zählt

Der Fokusvorbehalt hat zwei Hälften, und S4 hat nur die erste umgestellt. Vorher
fing die Klassenprüfung in `ersthelfer_gehoert_appkit` jeden Tastendruck ab,
sobald der Ersthelfer eine `NSTextView` war; die zweite Hälfte wurde nie
gebraucht. Seit S4 läuft jeder Tastendruck mit dem Fokus im Editor in den
Nachschlag und von dort in `kommando_ausfuehren`, wo `fokus::wirkt` ihn gegen
einen falschen Fokuswert hält.

Die Folgen sind nicht theoretisch:

- **`Wirkungsbereich::Navigator` läuft leer.** `wirkt(Navigator,
  Fokus::Dateifenster)` ist `true`. `up`, `down` und `tab` bewegen mit der
  Schreibmarke im Editor die Auswahl im Dateifenster, statt die Schreibmarke zu
  bewegen und einen Tabulator zu schreiben. Genau das war der Zweck des Umzugs
  aus S5, und genau das erste Abnahmekriterium von C7 ist damit weiter gebrochen.
- **Jeder Befehl mit `Wirkungsbereich::Dateifenster` feuert.** `delete` wirft die
  Auswahl des Dateifensters in den Papierkorb, `f5` startet eine Kopie, `space`
  markiert einen Eintrag, `right` öffnet einen Ordner — jeweils mit der
  Schreibmarke im Editor.
- **Der Zustand ist schlechter als vor S4.** Vorher gab der Abgriff jeden dieser
  Tastendrücke unverändert an AppKit weiter.

## Der Kommentar sagt heute das Gegenteil

Der Doc-Kommentar über `fokus()` steht seit der Runde 1 und ist durch S4
unwahr geworden (`anwendung.rs:2124-2130`):

> Drei Fälle. … Die Schreibmarke in einem Textfeld kommt hier nicht vor: der
> Ereignisabgriff reicht den Tastendruck dann weiter und erzeugt gar kein
> Kommando.

Für die Textfläche des Editors reicht der Abgriff seit S4 gerade **nicht** mehr
weiter. Die beiden Nachbarstellen, die denselben Wert brauchen, tragen ihren
Platzhalterhinweis (`fokus_setzen`, `anwendung.rs:1117-1122`: „**S17 löst diese
Zeile ab**"; `bereichskommando`, `anwendung.rs:1604-1611`: dasselbe). `fokus()`
trägt keinen. Sie ist damit die einzige der drei Stellen, an der der fehlende
Editor-Zweig nirgends vermerkt ist.

## Wie weit es heute reicht

Erreichbar, aber schmal. Der Bereich `Editor` steht ab Werk auf `false`
(`krk-core/src/ablage/sitzung.rs:229-236`), und kein gebauter Befehl blendet ihn
ein: `EditorSchliessen` fällt in `bereichskommando` und liefert `false`,
`FokusEditor` erreicht `fokus_holen` gar nicht, weil `kommando_ausfuehren` ihn
nicht namentlich führt. Wer `session.toml` von Hand auf `[sichtbarkeit] editor =
true` setzt, sieht die Fläche, kann hineinklicken und hat den Befund vor sich.
Mit S17/S18 wird er der Normalfall.

## Vorschlag

`fokus()` bekommt denselben Zweig, den `ist_editorflaeche` schon schreibt, und
zwar **vor** dem Rückfall auf `Fokus::Dateifenster`:

```rust
let im_editor = haupt
    .firstResponder()
    .is_some_and(|ersthelfer| self.ist_editorflaeche(&ersthelfer));
if im_editor {
    return Fokus::Editor;
}
```

Damit gibt es weiterhin **eine** Stelle, die die Nämlichkeitsfrage beantwortet,
und `fokus()` ruft sie, statt den Vergleich ein zweites Mal zu schreiben.

Ob die Zeile nach S16 gehört oder nach S17, ist eine Planfrage. Die Sachlage ist,
dass S4 und S16 zusammen einen Zustand hergestellt haben, in dem der
Fokusvorbehalt für den Editor offen ist und die Fokusabfrage ihn nicht kennt;
solange beides gilt, ist die Textfläche kein Editor, sondern eine zweite
Bedienung des Dateifensters.

Gemeldet von: `coderev`, Durchsicht Turn 2.

---
Resolved: S17 am 260809-1738. `Anwendungsdelegierter::fokus` liefert
`Fokus::Editor`, sobald der Ersthelfer die Textfläche des Editors ist.

Gebaut ist nicht der vorgeschlagene vierte `if`, sondern die Ursache darunter.
`fokusansicht(ziel)` in `anwendung.rs` ordnet jedem Fokuswert die Ansicht zu,
die seinen Ersthelferrang trägt — eine erschöpfende Fallunterscheidung ohne
Auffangzweig, die der Übersetzer erzwingt. `fokus()` läuft `Fokus::ALLE` durch
und hält den Ersthelfer gegen jede dieser Ansichten; `fokus_setzen()` geht
dieselbe Zuordnung in der Gegenrichtung. Ein vierter `if` hätte denselben
Fehler beim fünften Bereich wieder zugelassen.

Der Rückfall auf `Fokus::Dateifenster` steht weiter, trägt aber nur noch den
Fall "Ersthelfer gehört zu keinem der fünf Werte". Was er dort weiterhin falsch
beantwortet, führt
`260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`.

Der irreführende Doc-Kommentar ("Die Schreibmarke in einem Textfeld kommt hier
nicht vor") ist ersetzt. Vom Agenten abgenommen: die vier Abnahmekommandos und
die Probe `im_editor_wirkt_kein_befehl_des_dateifensters_und_jeder_des_fensters`
in `crates/krk-ui/src/kommandos/fokus.rs`. Nutzerarbeit bleibt der Nachweis am
laufenden Bündel, dass der Ersthelferrang tatsächlich auf der Textfläche steht.
