# Die neue Regel verweist jeden Rufer an sich selbst — einer der fünf sagt dort nichts

---

`52fba42` hat die Aufzählung der Rufer von `aufteilung_nachziehen` durch eine Regel ersetzt: „Ob
ein Aufrufer eine Messung davor braucht, steht **an ihm** und nicht hier." Das ist die richtige
Antwort auf `260823-0730`. Vier der fünf Rufer sagen es an sich; `sichtbarkeit_aendern` sagt es
nicht, und der Verweis läuft dort ins Leere.

---

**Am Baum gezählt.**

## Die Regel

`crates/krk-ui/src/appkit/anwendung.rs:4566-4577`, im Doc-Kommentar von
`bildschirmbreiten_uebernehmen`:

```
/// **Ob ein Aufrufer von [`Self::aufteilung_nachziehen`] eine Messung davor
/// braucht, steht an ihm und nicht hier.** Eine Aufzaehlung an dieser
/// Stelle war zweimal um eins daneben … Zwei der Aufrufer tragen ihre
/// Begruendung mit: beim Aufbau der Oberflaeche … und die Fortsetzung nach
/// einer Rueckfrage aus C4 … [`Self::aktives_setzen`] misst nicht und
/// begruendet es auch nicht; **das ist keine gepruefte Ausnahme, sondern ein
/// offener Befund**
```

## Die fünf Rufer

`grep -n "aufteilung_nachziehen()" crates/krk-ui/src/appkit/anwendung.rs`, jeder mit seiner
umgebenden Methode:

| Zeile | Rufer | Was an ihm steht |
|---|---|---|
| 1325 | `oberflaeche_aufbauen` | begründet: beim Aufbau gibt es keine Ziehbewegung |
| 3239 | `kommando_ausfuehren` | misst selbst, bei `:3025` |
| 4264 | `sichtbarkeit_aendern` | **nichts** |
| 4381 | `aktives_setzen` | als offener Befund `260823-0731` benannt |
| 6949 | `anlass_ausfuehren` | begründet: die Fortsetzung läuft hinter einem Blatt |

Der Doc-Kommentar von `sichtbarkeit_aendern` (`anwendung.rs:4202-4252`) ist ausführlich und
begründet die Reihenfolge des Nachzugs gegenüber den Fokussetzern in vier Absätzen. Er sagt kein
Wort dazu, ob vorher gemessen sein muss. Das ist die eine Adresse, an die die neue Regel verweist
und an der nichts steht.

## Ein Loch im Verhalten ist es nicht

`sichtbarkeit_aendern` hat zwei Rufer, `bereich_umschalten` (`:4083`) und `bereich_einblenden`
(`:4180`), und beide werden nur über `kommando_ausfuehren`, `anlass_ausfuehren` oder
`editorausgang_behandeln` erreicht. Alle drei messen. Der Rufer braucht also keine eigene Messung,
und der Grund dafür ist genau der, den die zwei begründeten Rufer für sich ausschreiben: zwischen
der letzten Messung und diesem Griff in das Modell kann keine Ziehbewegung liegen.

**Zu belegen ist das aber am Ort, und dort steht es nicht.** Der Satz, der `aktives_setzen` als
den einen benennt, der weder misst noch begründet, liest sich als vollständige Abrechnung über die
übrigen. Sie ist es nicht.

## Vorschlag

Zwei Sätze an den Doc-Kommentar von `sichtbarkeit_aendern`: dass es nicht misst, und warum es
das nicht muss, nämlich weil seine beiden Rufer nur aus messenden Wegen erreicht werden. Damit
trägt die neue Regel an allen fünf Adressen etwas, und `aktives_setzen` bleibt der einzige, an dem
ein offener Befund steht statt einer Begründung.

**Schwere:** Low. Kein Verhalten ist betroffen; die Lücke ist eine der Prosa, und zwar in genau
dem Absatz, der sie für diese Klasse zu schließen unternimmt.

**Gefunden:** coderev, Auslieferungsdurchsicht `28cbb7b..b58e9d1`, Baumstand `b58e9d1`

**Domain:** code

**Cross-references:** `shared/issues/260823-0730_c_drei-prosastellen-um-den-neuen-nachzug-*`,
`shared/issues/260823-0731_o_ein-klick-in-das-andere-dateifenster-*` (nicht angefasst)

---
Resolved:
