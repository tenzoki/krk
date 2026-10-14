Der Schnitt sieht einen zerlegt geschriebenen Umlaut als anderen Eintrag als den zusammengesetzten

---

`gleicher_eintrag` (`crates/krk-ui/src/kommandos/kontextmenue.rs`) faltet seit `F-5` ueber
`to_lowercase()`, also ueber ganz Unicode, und schliesst damit den ASCII-Fall. Offen bleibt die
Normalform: `Ü` als ein Zeichen und `U` mit nachgestelltem Trema sind fuer den Schnitt zwei
Eintraege, auf APFS in der Vorgabe aber einer. Die Probe
`ein_zerlegt_geschriebener_umlaut_bleibt_quelle` haelt diese Grenze ausdruecklich fest, und der
Doc-Kommentar schreibt sie aus.

---

**Filed by:** orchestrator, aus dem Bericht zu `F-5`

## Warum es nicht in derselben Runde behoben ist

Die Antwort waere eine Normalform und damit eine Zerlegungstabelle, die es in diesem Baum nur als
fremdes Paket gibt. Die Runde 17 hat ausdruecklich keine weitere fremde Kiste aufgenommen; die
Pruefung, was eine solche Kiste an Abhaengigkeiten hereinzoege, ist nicht gefahren.

## Die Folge, wenn es eintritt

Der Nutzer packt in einen Ordner, dessen vorhandenes Archiv den Namen in der anderen Normalform
traegt. Der Schnitt erkennt es nicht als dasselbe, das Archiv bleibt in den Quellen, und der Lauf
packt es in sich selbst hinein — die Lage, die die dritte Nutzerzusage dieser Runde ausschliessen
soll. Zerstoert wird nichts.

## Wie ein Name in die andere Normalform geraet

Nicht durch KRK. Ein Name aus einer fremden Quelle, etwa von einem Datentraeger eines anderen
Systems oder aus einem anderswo gepackten Archiv, kann sie tragen.

## Umfang

`krk-ui`, `kommandos/kontextmenue.rs`, `gleicher_eintrag`; dazu die Frage, ob dieses Vorhaben
dafuer eine fremde Kiste aufnimmt.
