Der Defektspeicher zählt zwei von vier Markern, und vier Datensätze fallen durch

---

Das Profil „fusion-Werkbank: ein Defektspeicher" (`resources/default-readers.toml:229-243`) trägt
zwei Zählungen, `_o_` und `_c_`. Das Markervokabular der Werkbank kennt vier Marker für einen
Defektdatensatz. Ein Datensatz auf `_p_` oder `_d_` erscheint in keiner der zwei Zeilen, und weil
das Profil keine Gesamtzahl führt, kann der Nutzer die Lücke nicht bemerken.

---

**Gemessen am 260824-1649 am Bestand dieser Werkbank**, mit `regex` 1.13.1 und den Mustern der
Auslieferungsfassung gegen die neunzehn Defektspeicher (`shared/issues` und achtzehn
`circles/*/issues`):

| Marker | Datensätze | vom Profil gezählt |
|---|---|---|
| `_o_` | 163 | ja, Zeile „Offen" |
| `_c_` | 439 | ja, Zeile „Geschlossen" |
| `_d_` | 4 | **nein** |
| `_p_` | 0 | **nein** |

Für ``shared/issues`` allein: 82 Datensätze, davon 54 offen und 27 geschlossen. Das Profil zeigt
54 und 27, zusammen 81. Der eine fehlende ist
`shared/issues/260815-1047_d_die-bedingung-der-moeglichkeit-2-…` (Marker `_d_`, zurückgestellt).

**Zwei Stellen sind betroffen, und die zweite ist die Ursache der ersten.**

1. `resources/default-readers.toml:228`: „Der Marker im Dateinamen trägt den Stand: `_o_` offen,
   `_c_` geschlossen." Das Vokabular hat vier Marker, nicht zwei:
   `rules/fusion-workbench-conventions.md` `## State Markers — issues and planning` führt
   `_o_` offen, `_p_` in Arbeit, `_c_` geschlossen, `_d_` zurückgestellt.
2. `resources/default-readers.toml:233-239`: die zwei Zählungen. Das Nachbarprofil „ein Speicher"
   führt mit „Datensätze" (`\.md$`) eine Gesamtzahl; der Defektspeicher führt keine. Ohne sie
   ergibt sich die Lücke aus nichts, was auf dem Bildschirm steht.

**Zwei Wege, und beide kosten nichts im Haushalt.** Eine dritte Zeile „Datensätze" mit
`zaehlung = { muster = '\.md$' }` macht die Lücke sichtbar, ohne eine Aussage über die Marker zu
treffen; eine vierte Zeile „Zurückgestellt" mit `_d_` schließt sie. Beide sind eine `zaehlung`
ohne `ordner` und benutzen den ohnehin gelesenen Ordner: der Defektspeicher bleibt bei einem
Leselauf und zehn Öffnungen, weit unter den Grenzen 12 und 24 aus C6.4.

Der Kommentar in Zeile 228 ist in jedem Fall zu berichtigen: er nennt zwei Marker, wo vier gelten.

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

---
Resolved: Der Nutzer hat am 260824-1935 **beide** Zeilen gewählt, und beide stehen:
`resources/default-readers.toml` führt im Profil „ein Defektspeicher" jetzt fünf Zeilen statt
drei, nämlich „Datensätze" (`zaehlung = { muster = '\.md$' }`), „Offen", „Geschlossen",
„Zurückgestellt" (`_d_`) und „Die jüngsten zehn". Begründung des Nutzers, die im Kommentarkopf
des Profils steht: die Summe geht damit auf, und die Gesamtzahl fängt jeden künftigen Marker ab,
den keine eigene Zeile führt.

Der Kommentar in der vormaligen Zeile 228 ist berichtigt: er nennt jetzt alle vier Marker des
Vokabulars (`_o_`, `_p_`, `_c_`, `_d_`) und sagt, warum `_p_` keine eigene Zeile bekommt.

**Am Bestand dieser Werkbank nachgerechnet am 260824-1739**, mit `regex` 1.13.1 und den Mustern
der geänderten Datei gegen die neunzehn Defektspeicher: 622 Datensätze, davon 178 offen, 440
geschlossen, 4 zurückgestellt und 0 in Arbeit. 178 + 440 + 4 + 0 = 622, die Summe geht auf. Die
Zahlen dieses Datensatzes sind seit dem 260824-1649 gewachsen, weil die Durchsicht selbst
Datensätze angelegt hat; die Verteilung ist dieselbe.

**Der Haushalt hält, und die Grenze im Rundenprofil ist unberührt.** Alle fünf Zeilen sind eine
`zaehlung` oder eine `juengste` **ohne** `ordner`, benutzen also den einen ohnehin fälligen
Leselauf. Gemessen nach der Änderung: der Defektspeicher steht bei **einem** Leselauf und **zehn**
Öffnungen, unverändert gegen vorher und weit unter den Grenzen 12 und 24 aus C6.4. Die vier Zahlen
der Probe `die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` sind
gleichgeblieben: das Rundenprofil bei (5, 11), die Wurzel bei (3, 5). `make check` grün, Exit 0.
