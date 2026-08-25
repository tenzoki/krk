Die Zeile „Projekt" der Werkbankprofile hängt an einem Feld, das fusion nicht mehr schreibt

---

`resources/default-readers.toml` zieht in der Zeile „Projekt" den Projektnamen aus dem
Feld `setup_pwd` von `fusion-workbench/.fusion-setup`. Dieses Feld schreibt fusion seit
einer Fassung zwischen 8.1.0 und 10.7.0 nicht mehr. Die Zeile zeigt an jeder heute
eingerichteten fusion-Werkbank den Platzhalter `--` und kann dort nie einen Wert liefern.

---

**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>

## Was gemessen ist

Der Marker dieses Projekts, am 260825 aus dem Baum und aus der Historie gelesen:

| Fassung | Inhalt von `fusion-workbench/.fusion-setup` |
|---|---|
| 7.2.0 (`f9ebbdc`) | `{"setup_at":"2026-08-11T14:53:41+0200","setup_pwd":"/Users/k1/Projects/productive/krk","plugin_version":"7.2.0"}` |
| 8.1.0 (`7d5a25f`) | `{"setup_at":"2026-08-13T08:05:00+0200","setup_pwd":"/Users/k1/Projects/productive/krk","plugin_version":"8.1.0"}` |
| 10.7.0 (heutiger Stand) | `{"setup_at":"2026-08-25T16:54:47+0200","plugin_version":"10.7.0"}` |

Die schreibende Stelle ist der Block in `skills/setup/SKILL.md` der installierten
fusion-Fassung; er setzt heute genau zwei Felder und `setup_pwd` ist keines davon.

Die Zusammenfassung dieses Projekts, gefahren am 260825-2044 gegen die neue
Auslieferungsfassung:

```
=========== fusion-workbench
Projekt: --
Eingerichtet: 2026-08-25T16:54:47+0200
fusion-Fassung: 10.7.0
```

Die zwei übrigen Feldzeilen desselben Musters stimmen weiter; allein `setup_pwd` fehlt.

## Warum das nicht in Schritt 8 mitbehoben ist

Kein Baustein liefert einen **Ordnernamen**. `feld` liest den Inhalt einer Datei, und die
drei übrigen sehen auf Namen von Einträgen **im** Ordner, nicht auf den des Ordners
selbst. Der Name des erkannten Ordners steht in der Kopfzeile jeder Zusammenfassung
(`Zusammenfassung::name`) und ist von einer Zeile aus nicht erreichbar. Eine Behebung
verlangt daher entweder einen fünften Baustein oder eine andere Quelle für den Namen, und
beides liegt im Mechanismus unter `crates/krk-core/src/leseprofil/`, den Schritt 8 nicht
anfassen darf.

## Was ohne Behebung gilt

- An einer Werkbank, die mit fusion 8.1.0 oder älter eingerichtet und seither nicht
  erneut eingerichtet wurde, liefert die Zeile weiter den Projektnamen. Ein Setup-Lauf mit
  einer neueren fusion überschreibt den Marker und nimmt das Feld mit.
- Das flight-Vorbild unter `/Users/k1/Projects/productive/example/` schreibt `setup_pwd`
  weiterhin (`.flight-setup`, flight 0.8.0). Die Zeile „Projekt" der vier flight-Profile
  liefert dort einen Wert: `2026-Sommer-Adria`. Der Defekt betrifft allein die
  fusion-Profile.
- Betroffen sind zwei Blöcke der Auslieferungsfassung, `fusion-Werkbank: die Wurzel` und
  `Projektwurzel mit fusion-Werkbank`, weil beide dieselben sieben Zeilen führen.

## Möglichkeiten

1. Die Zeile fällt. Sie kostet eine Öffnung, die sie sich mit zwei anderen Zeilen teilt,
   also nichts, und sagt heute nichts.
2. Die Zeile bleibt und der Kommentar sagt, warum sie leer steht. So steht es seit dem
   260825-2044 in der Datei.
3. Der Mechanismus bekommt eine Quelle für den Namen des erkannten Ordners. Das ist die
   einzige Möglichkeit, die die Zeile wieder antworten lässt, und die teuerste.

**Cross-references:**
`fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
nimmt in C3.7 ausdrücklich ab, dass `"setup_pwd":"[^"]*/([^"/]+)"` den Projektnamen
liefert. Das Kriterium war beim Abnehmen wahr und ist es an diesem Baum nicht mehr; es ist
kein Fehler der Abnahme, sondern eine Änderung an fusion darunter.
