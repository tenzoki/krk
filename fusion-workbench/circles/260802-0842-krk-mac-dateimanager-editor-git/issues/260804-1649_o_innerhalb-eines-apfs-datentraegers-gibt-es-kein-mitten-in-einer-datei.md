Innerhalb eines APFS-Datenträgers gibt es kein "mitten in einer Datei"

---

`COPYFILE_CLONE` macht eine Kopie innerhalb eines Datenträgers zu einer Sache von Mikrosekunden. Damit ist die Zusage aus C4, dass eine laufende Operation einen Fortschritt zeigt und sich abbrechen lässt, für den häufigsten Fall gegenstandslos, und das Abnahmekriterium von Schritt 15 lässt sich mit der Auslieferungseinstellung nicht prüfen.

---

## Gemessen am 260804-1649

Beide Läufe auf demselben APFS-Datenträger, dieselbe 500-MB-Datei, derselbe Rechner:

| Weg | Kennzeichen | Dauer |
|---|---|---|
| Klon | `COPYFILE_ALL \| COPYFILE_CLONE` | 0,42 ms |
| Bytes | `COPYFILE_ALL` | über 400 ms, nach 40 ms abgebrochen bei 32 MiB |

Ein Abbruch, der nach 0,42 ms ankommt, kommt nie an. `copyfile(3)` ruft den Statusrückruf beim Klonen überhaupt nicht: die Handbuchseite sagt "if cloning is successful, progress callbacks will not be invoked", und das deckt sich mit der Messung.

## Was daraus folgt

**Für den Nutzer ist das kein Schaden, sondern das Gegenteil.** Eine Kopie von 50 GB innerhalb eines Datenträgers ist sofort fertig; ein Fortschrittsblatt dafür wäre eine Zumutung und ein Abbruchbefehl ohne Gegenstand. Die 150-ms-Regel aus `### Frage 6` des Plans fängt den Fall von selbst ab: nach 150 ms ist der Klon längst durch, und es erscheint kein Blatt.

**Für die Abnahme ist es einer.** Drei Stellen im Plan und im Spec sprechen so, als hinge der Fortschritt an der Datenmenge:

- C4: "Eine Operation über mehr als 100 Einträge oder mehr als 100 MB zeigt einen Fortschritt und lässt sich mit einem Tastenbefehl abbrechen."
- C4: "Nach einem Abbruch nennt KRK, wie viele Einträge bereits übertragen wurden."
- S15: "Abbruch mitten in einer 500-MB-Datei kehrt binnen 100 ms zurück und meldet die bis dahin übertragene Zahl."

Alle drei treffen innerhalb eines Datenträgers auf eine Operation, die vorbei ist, bevor sie beginnen konnte. Zutreffend sind sie für das Kopieren **über** eine Datenträgergrenze, auf ein Netzlaufwerk und auf einen Datenträger ohne Klonunterstützung; genau dort ist eine 500-MB-Kopie auch wirklich langsam.

## Wie Schritt 15 damit umgegangen ist

`crates/krk-core/src/verzeichnis/sys.rs` führt `Uebertragungsart` mit den beiden Werten `KlonenWennMoeglich` (Vorgabe, die Oberfläche lässt sie stehen) und `ImmerBytes`. Die Abnahmeprüfung `crates/krk-core/tests/operation.rs` prüft beide Wege:

- `der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` mit `ImmerBytes`: Rückkehr nach 2,0 ms, gemeldet 32 MiB von 500 MB.
- `dieselben_500_mb_sind_als_klon_lange_vor_der_frist_fertig` mit der Vorgabe: 0,42 ms, vollständig.

Damit ist beides belegt, aber das Kriterium ist streng genommen an einer Einstellung geprüft, die im laufenden Programm nicht vorkommt.

## Was zu entscheiden ist

Ob C4 und S15 die Unterscheidung aufnehmen sollen, also ausdrücklich sagen, dass Fortschritt und Abbruch für das Kopieren über Datenträgergrenzen zugesagt sind und innerhalb eines Datenträgers gegenstandslos. Die Alternative wäre, `COPYFILE_CLONE` fallen zu lassen, und das wäre teuer erkauft: eine Kopie von 50 GB dauerte dann Minuten statt Millisekunden und verbrauchte 50 GB Plattenplatz.

**Aufgefallen bei:** der Umsetzung von Schritt 15 am 260804-1649.
