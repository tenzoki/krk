Ein Archivname aus Punkten macht den angezeigten Ordner oder seinen Elternordner zum Entpackziel

---

`kontextmenue::ordnername_zum_archiv` kann `.` oder `..` als Ordnernamen liefern, und kein Aufrufer prueft das Ergebnis. Aus einer Datei `..zip` wird der Zielordner `<angezeigter Ordner>/.`, aus `...zip` der Zielordner `<angezeigter Ordner>/..`. Wer im Konfliktblatt "Ueberschreiben" waehlt, raeumt damit den angezeigten Ordner oder dessen Elternordner in den Papierkorb; wer nicht ueberschreibt, bekommt den Archivinhalt in den angezeigten Ordner oder eine Ebene darueber geschrieben.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/kommandos/kontextmenue.rs:343-353` — `ordnername_zum_archiv` gibt den Stamm aus `namen_teilen` unveraendert zurueck.
- `crates/krk-ui/src/kommandos/kontextmenue.rs:420-423` — `paar` baut daraus `ordner.join(stamm)`, ohne den Namen zu pruefen.
- `crates/krk-core/src/operation/entpacken.rs:165-196` — `zielordner_klaeren` nimmt `ziel` als gegeben hin; `name_pruefen` laeuft allein im Zweig `UmbenennenIn`.

## Die Rechnung

`krk_core::operation::umbenennen::namen_teilen` trennt am **letzten** Punkt, sofern er nicht an Stelle 0 steht (`crates/krk-core/src/operation/umbenennen.rs:177-182`). Nachgerechnet mit einer eigenstaendigen Fassung der drei Funktionen am 260825:

| Dateiname | `ist_zipname` | Stamm | Zielordner |
|---|---|---|---|
| `a.zip` | ja | `a` | `<ordner>/a` |
| `..zip` | ja | `.` | `<ordner>/.` |
| `...zip` | ja | `..` | `<ordner>/..` |
| `␣␣.zip` | ja | `␣␣` | `<ordner>/␣␣` |

`PathBuf::join` normalisiert nichts, und `fs::symlink_metadata("<ordner>/..")` loest das `..` auf und trifft den Elternordner. Alle vier Namen sind auf macOS anlegbar.

## Warum die zwei bestehenden Sperren nicht greifen

`ZipFile::enclosed_name` und `kette_anlegen` sperren jeden Weg **aus dem Zielordner heraus**, und beide arbeiten korrekt. Sie arbeiten aber relativ zu dem `ziel`, das die Oberflaeche gerechnet hat, und genau dieses `ziel` stammt hier aus einem fremden Dateinamen. Der Ausbruch geschieht vor den Sperren und nicht an ihnen vorbei.

## Warum es heute nicht ausloest

Die Schritte 6 und 7 des Plans fehlen, also ruft niemand ausser den Proben in `kontextmenue` hinein. Der Befund ist trotzdem einer am gebauten Stand: die Regel steht, sie ist die Rechnung, die Schritt 7 verwenden wird, und `krk_core::operation::umbenennen::name_pruefen` weist `.` und `..` schon heute mit `Namensfehler::Punktname` ab, ohne dass dieser Weg sie fragt.

## Vorschlag

Den gerechneten Ordnernamen durch `name_pruefen` schicken, an genau einer Stelle, naemlich in `paar`. Faellt er durch, auf `ERSATZSTAMM` zurueckfallen oder das Archiv aus dem Befund heraushalten und in der Statuszeile melden. Damit deckt der Weg zugleich den Namen `␣␣`, den `name_pruefen` als `Leer` abweist.

## Umfang

`krk-ui`, das Modul `kommandos/kontextmenue`. Der Kern ist nicht zu aendern: er tut, was der Auftrag sagt.
