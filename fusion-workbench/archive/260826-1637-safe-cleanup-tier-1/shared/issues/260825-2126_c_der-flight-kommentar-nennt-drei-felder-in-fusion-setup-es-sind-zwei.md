Der flight-Kommentar nennt drei Felder in `.fusion-setup`, es sind zwei

---

`resources/default-readers.toml:646-647` sagt: „Die Kennzeichendatei `.flight-setup` trägt
dieselben drei Felder wie `.fusion-setup`, also greifen dieselben Muster." `.fusion-setup`
trägt an einer mit fusion 10.7.0 eingerichteten Werkbank zwei Felder und nicht drei.
Dieselbe Datei sagt das vierhundert Zeilen weiter oben selbst (`:248-254`).

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:248-254` und `:646-647`;
`shared/issues/260825-2044_o_die-zeile-projekt-der-werkbankprofile-haengt-an-einem-feld-das-fusion-nicht-mehr-schreibt.md`

## Was gemessen ist

Gelesen am 260825-2126, Baum `8478753`:

```
fusion-workbench/.fusion-setup
  {"setup_at":"2026-08-25T16:54:47+0200","plugin_version":"10.7.0"}

/Users/k1/Projects/productive/example/flight-workbench/.flight-setup
  {"setup_at":"2026-07-19T19:55:48+0200","setup_pwd":"/Users/kai/Dropbox/ops/K/Reisen/2026-Sommer-Adria","plugin_version":"0.8.0"}
```

Zwei Felder gegen drei. Der Schluss des Satzes stimmt trotzdem — dieselben Muster greifen —,
und zwar mit dem Unterschied, dass zwei der drei Muster an beiden Dateien einen Wert liefern
und das dritte allein an `.flight-setup`. Genau das steht auch im Datensatz `260825-2044`.

## Warum das zählt

Der Satz ist die Begründung dafür, dass die flight-Profile die drei Feldmuster der
fusion-Profile übernehmen dürfen. Er stimmt in seiner Wirkung und nicht in seiner Prämisse,
und die falsche Prämisse zeigt gerade auf den Unterschied, den der offene Datensatz
`260825-2044` festhält: bei flight liefert „Projekt" einen Wert, bei fusion nicht. Wer den
Satz liest und danach die leere „Projekt"-Zeile an der fusion-Werkbank sieht, sucht den Fehler
an der falschen Stelle.

## Was zu tun wäre

Den Satz umschreiben, etwa: „`.flight-setup` trägt dieselben Felder wie `.fusion-setup` und
dazu `setup_pwd`, das fusion nicht mehr schreibt; alle drei Muster greifen, und die Zeile
„Projekt" liefert deshalb hier einen Wert und dort den Platzhalter." Der Verweis auf den
Absatz bei `:248-254` gehört dazu.

**Schwere:** niedrig. Eine Prosaangabe, die der eigenen Datei widerspricht.

---

Resolved: Der Satz steht nicht mehr im Kommentarkopf des flight-Abschnitts von
`resources/default-readers.toml`. An seiner Stelle steht, was gemessen ist: `.flight-setup`
trägt dieselben Felder wie `.fusion-setup` **und dazu** `setup_pwd`, das fusion nicht mehr
schreibt; alle drei Muster greifen, aber nicht mit derselben Ausbeute — zwei liefern an beiden
Dateien einen Wert, das dritte allein an `.flight-setup`, und die Zeile „Projekt" steht deshalb
bei flight mit einem Wert da und an einer fusion-Werkbank leer. Der Verweis auf den Absatz beim
fusion-Wurzelprofil, den dieser Datensatz verlangt, steht im letzten Halbsatz.

Die Prämisse ist damit berichtigt und der Schluss unverändert stehen geblieben: er stimmte
schon vorher. Nachgemessen an einer Prüfwerkbank mit `.flight-setup` in der Gestalt aus „Was
gemessen ist": die drei Feldzeilen liefern `Adria`, den Einrichtungszeitpunkt und `0.8.0`, alle
drei mit Wert.

Bearbeitet von ontocoder im Zuge der Vertiefung der vier flight-Profile.
