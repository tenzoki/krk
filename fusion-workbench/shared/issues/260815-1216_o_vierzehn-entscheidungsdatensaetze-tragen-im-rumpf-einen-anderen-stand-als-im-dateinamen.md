Vierzehn Entscheidungsdatensätze tragen im Rumpf einen anderen Stand als im Dateinamen

---

Die Kopfzeile `**Status:**` eines Entscheidungsdatensatzes und der Marker in seinem Dateinamen sagen dasselbe und laufen bei vierzehn von 136 Datensätzen auseinander. Verbindlich ist nach `CLAUDE.md` der Dateibestand; wer den Rumpf liest, bekommt bei diesen vierzehn die falsche Auskunft.

---

**Gefunden am:** 260815-1216, Stand `9a2d0e0`
**Gefunden von:** reconciler, beim Abgleich der Sitzung 260815-0912
**Herkunft:** kein Circle war aktiv; der Befund verteilt sich über den gemeinsamen Speicher und fünf Circles und liegt deshalb im gemeinsamen Speicher.

## Der Befund

Erhoben über alle 136 Datensätze unter `fusion-workbench/shared/decisions` und `fusion-workbench/circles/*/decisions`:

| Dateiname sagt | Rumpf sagt | Zahl |
|---|---|---|
| `_i_` (umgesetzt) | `answered` | 8 |
| `_i_` (umgesetzt) | `open` | 2 |
| `_d_` (zurückgestellt) | `open` | 2 |

Nachzuzählen mit:

```sh
cd fusion-workbench
for f in shared/decisions/*.md circles/*/decisions/*.md; do
  m=$(basename "$f" | sed -nE 's/^[0-9]{6}-[0-9]{4}_([a-z])_.*/\1/p')
  s=$(grep -m1 '^\*\*Status:\*\*' "$f" | sed 's/^\*\*Status:\*\* *//')
  echo "$m $s $f"
done
```

Die Zuordnung ist `_o_`/open, `_a_`/answered, `_i_`/implemented, `_d_`/deferred, `_s_`/superseded.

**Zwei der vierzehn sind mit dem Abgleich vom 260815-1216 schon weg**: `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_i_bleibt-der-filtertext-…` stand auf `open` und ist auf `implemented` gezogen, weil er aus der abzugleichenden Sitzung stammt. Die übrigen zwölf sind älter und hier aufgenommen statt nebenbei angefasst.

## Warum es der Rede wert ist

Die Muster dieses Projekts lesen den Marker aus dem Dateinamen, nicht die Kopfzeile — die `find`-Aufrufe in `CLAUDE.md` unter `## Bindende Grundlage` tun genau das. Ein Leser tut es nicht: er öffnet die Datei und liest oben `**Status:** open` über einem Rumpf, der unten eine Zeile `Implemented:` trägt. Es ist derselbe Fehlertyp wie bei den ausgeschriebenen Markerzitaten (`shared/issues/260815-1216_o_sieben-verweise-dieser-sitzung-…`): eine zweite Stelle, die dieselbe Auskunft führt und beim Weiterschieben nicht mitwandert.

## Zwei Auswege

1. **Die vierzehn nachziehen** und die Kopfzeile künftig bei jedem Markerlauf mitschreiben. Billig, ändert nichts an der Bauart, und die nächste Abweichung entsteht beim nächsten Lauf, den jemand vergisst.
2. **Die Status-Kopfzeile aus der Vorlage nehmen.** Dann führt den Stand allein der Dateiname, und zwei Stellen können nicht mehr auseinanderlaufen. Kostet die Auskunft nicht, denn der Dateiname steht in jeder Auflistung mit dabei — aber die Vorlage gehört `fusion` und nicht diesem Projekt, also ist der Ausweg von hier aus nicht gangbar.

**Weg 1 ist der einzige, den dieses Projekt selbst gehen kann.** Zu entscheiden bleibt, ob er sich lohnt oder ob die Abweichung als bekannt hingenommen wird.
