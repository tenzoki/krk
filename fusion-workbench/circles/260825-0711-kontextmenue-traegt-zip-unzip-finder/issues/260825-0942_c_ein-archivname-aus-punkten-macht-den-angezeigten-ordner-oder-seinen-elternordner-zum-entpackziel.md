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

---
Resolved: `crates/krk-ui/src/kommandos/kontextmenue.rs` — die neue private Funktion `brauchbarer_stamm` schickt jeden gerechneten Stamm durch `krk_core::operation::umbenennen::name_pruefen` und fällt bei jedem der vier `Namensfehler` auf `ERSATZSTAMM` zurück. Keine zweite Namensprüfung daneben; die bestehende deckt zugleich den Stamm `␣␣` aus `␣␣.zip`.

Gewählt ist der Ersatzname und nicht die Statuszeile: die Directive stellt den Satz der Statuszeile für den Fall bereit, dass ein Befehl **nichts vorfindet**, und Unzip findet hier etwas vor — der Nutzer hat auf eine Datei geklickt, die die Endung sichtbar trägt. `kein_archiv()` wäre vor seinen Augen die Unwahrheit, und den Eintrag stillschweigend fallen zu lassen wäre schlechter. Unbrauchbar ist allein der gerechnete Name, und für den steht die Antwort seit dem Wurzelverzeichnis schon da; sie ist erweitert und nicht verdoppelt. `kommandos/operationen.rs` ist unangetastet.

Abweichung vom Vorschlag: die Prüfung steht in `ordnername_zum_archiv` und nicht in `paar`. Die Zusage „das ist ein Name" gehört der Funktion, die den Namen herausgibt; `ordnername_zum_archiv` ist `pub`, und eine Prüfung im Aufrufer ließe den öffentlichen Rückweg weiterhin `..` liefern. `archivname` geht denselben Weg, damit das Paar in beiden Richtungen dieselbe Antwort gibt; die Umkehrbarkeit bleibt für jeden Namen erhalten, den `name_pruefen` durchlässt.

Vier neue Proben neben dem Code, darunter je eine für `..zip` und `...zip` über den vollen Weg `entpackziel` und eine, die über zehn Namen die **Gestalt** des Zielordners prüft statt einer Liste erwarteter Namen. Gegenprobe: ohne die Sperre werden genau diese vier rot. `cargo test -p krk-ui` exit 0, `cargo clippy -p krk-ui --all-targets -- -D warnings` exit 0, `cargo fmt --all --check` exit 0. Protokoll: `history/260825-1033-coder-b1-archivname-aus-punkten.md`.
