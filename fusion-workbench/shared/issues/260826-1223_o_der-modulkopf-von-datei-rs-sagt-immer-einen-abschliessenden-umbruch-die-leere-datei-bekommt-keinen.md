Der Modulkopf von `text/datei.rs` sagt „immer einen abschließenden Umbruch", die leere Datei bekommt keinen

---

Zwei Prosastellen derselben Datei widersprechen einander: der Modulkopf sagt die drei Sicherungseigenschaften ohne Ausnahme zu, `sicherungsform` nennt hundert Zeilen weiter unten die Ausnahme, die der Kopf ausschließt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die zwei Stellen

`crates/krk-core/src/text/datei.rs:83-85`, Abschnitt „Der Preis dieser Wahl, ausgeschrieben":

> KRK schreibt beim Sichern **immer** Unix-Zeilenenden, **immer** einen abschliessenden Umbruch und **nie** eine Bytefolgenmarke, unabhaengig von der Form, die die Datei mitbrachte.

`crates/krk-core/src/text/datei.rs:867-869`, an `sicherungsform`:

> **Der leere Stand bleibt leer.** Eine Datei ohne Zeile braucht keinen Zeilenabschluss, und ein angehaengtes `\n` machte aus einer Datei von null Bytes eine von einem.

Der Rumpf (Zeilen 875-881) folgt der zweiten Stelle: `if stand.is_empty() || stand.ends_with('\n')` gibt den Stand unverändert zurück.

## Welche der beiden gilt

Die zweite. Sie steht an der Funktion, die die Regel trägt, sie ist begründet, und der Rumpf hält sie. Der Modulkopf ist die Stelle, die nachzuziehen ist — er verspricht einem Leser, der die Zusage von dort nimmt, eine Eigenschaft, auf die er sich für die leere Datei nicht stützen darf.

## Schwere

Niedrig, und trotzdem zu richten: dieser Kopf ist die Stelle, an die `CLAUDE.md` und der Kopf von `text/mod.rs` für die tragende Zusage des ganzen Verzeichnisses verweisen („Die Zusage, ohne die `zeilen` und `suche` anders aussehen muessten, steht in `datei` und wird hier nicht zum zweiten Mal formuliert", `text/mod.rs:29-31`). Ein Satz mit einem „immer", das nicht immer gilt, ist an dieser Stelle teurer als anderswo.

Vorschlag: das zweite „immer" auf „einen abschließenden Umbruch, sobald der Stand eine Zeile trägt" bringen und für die Ausnahme auf `sicherungsform` zeigen, statt die Begründung ein zweites Mal zu schreiben.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
