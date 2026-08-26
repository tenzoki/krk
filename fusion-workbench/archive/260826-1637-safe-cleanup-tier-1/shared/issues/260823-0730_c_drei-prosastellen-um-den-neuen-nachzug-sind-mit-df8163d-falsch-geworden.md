Drei Prosastellen um den neuen Nachzug sind mit `df8163d` falsch geworden

---

`df8163d` hat vier Prosastellen mitgezogen und drei weitere nicht. Zwei davon standen vor dem
Commit richtig und sind durch ihn falsch geworden (`anwendung.rs:4500-4506` zählt die Aufrufer
von `aufteilung_nachziehen`, `anwendung.rs:3462-3468` sagt, `nach_dem_sichtbarkeitswechsel`
lege die Fensterzeile nicht neu aus); die dritte ist mit `df8163d` neu geschrieben und war bei
ihrem Entstehen falsch (`anwendung.rs:4174-4177` datiert den Nachzug in `anlass_ausfuehren`
auf diesen Befund).

---

**Gemessen am Baumstand `df8163d`.**

## 1. Die Zählung der Aufrufer ist von zwei auf vier gewachsen, der Satz nennt weiter zwei

`crates/krk-ui/src/appkit/anwendung.rs:4500-4506`, im Doc-Kommentar von
`bildschirmbreiten_uebernehmen`:

```
    /// **Die beiden uebrigen Aufrufer von [`Self::aufteilung_nachziehen`]
    /// brauchen keine Messung davor.** Beim Aufbau der Oberflaeche gibt es noch
    /// keine Ziehbewegung, die verlorenginge, und die Fortsetzung nach einer
    /// Rueckfrage aus C4 laeuft hinter einem Blatt: ...
```

`aufteilung_nachziehen` hat am Stand `df8163d` **fünf** Aufrufstellen:

| Zeile | Rufer | im Satz genannt | Messung davor |
|---|---|---|---|
| `:1292` | `oberflaeche_aufbauen` | ja („Aufbau der Oberflaeche") | keine, begründet |
| `:3193` | `kommando_ausfuehren` | ja, als der Rufer **mit** Messung | `:2991` |
| `:4205` | `sichtbarkeit_aendern` | **nein** | über den Rufer, siehe unten |
| `:4322` | `aktives_setzen` | **nein** | keine, unbegründet |
| `:6836` | `anlass_ausfuehren` | ja („Fortsetzung nach einer Rueckfrage aus C4") | keine, begründet |

Vier stehen neben `kommando_ausfuehren`, der Satz nennt zwei.

**Der Satz war schon vor `df8163d` um eins daneben.** `:4322` in `aktives_setzen` stammt aus
`537fda53` vom 260804 und ist nie in die Aufzählung gekommen; `git blame` weist es aus.
`df8163d` hat mit `:4205` die vierte hinzugefügt und den Satz nicht angefasst.

**Der Satz ist keine bloße Zählung, sondern eine Zusage.** Er behauptet, jeder Aufrufer außer
`kommando_ausfuehren` sei ohne Messung unbedenklich, und begründet das für zwei von ihnen. Für
`aktives_setzen` steht keine Begründung da, und es gibt auch keine — der Fall ist als eigener
Datensatz gefasst (`shared/issues/260823-0731_*`). Wer den Satz beim Wort nimmt, hält jenen
Fall für geprüft.

## 2. `ordner_angleichen` sagt weiter, die Fensterzeile werde nicht neu ausgelegt

`crates/krk-ui/src/appkit/anwendung.rs:3462-3468`, im Doc-Kommentar von `ordner_angleichen`:

```
    /// Hier
    /// ist er tragend und nicht kosmetisch, denn
    /// [`Self::nach_dem_sichtbarkeitswechsel`] legt die Fensterzeile **nicht**
    /// neu aus: ein hervorgeholtes Dateifenster bekommt seinen Nachzug allein
    /// ueber diesen Wert.
```

Der erste Halbsatz stimmt weiter: `nach_dem_sichtbarkeitswechsel` (`:4222-4252`) ruft
`aufteilung_nachziehen` nicht. Der zweite ist seit `df8163d` falsch. `ordner_angleichen` ruft
`bereich_einblenden` (`:3485`), das über `sichtbarkeit_aendern` läuft, und dort steht der
Nachzug jetzt (`:4205`). Ein hervorgeholtes Dateifenster bekommt seinen Nachzug also **nicht
mehr allein** über den Rückgabewert, sondern zuerst aus der Änderungsstelle selbst.

Die Folge für den Leser ist nicht harmlos. Der Absatz begründet damit, warum die zwei
Leerwege `false` liefern müssen. Der Rückgabewert trägt weiterhin `sitzung_vormerken` und den
Nachzug der übrigen Anzeigen, aber die genannte Begründung trägt ihn nicht mehr.

## 3. Der Nachzug in `anlass_ausfuehren` stammt nicht aus diesem Befund

`crates/krk-ui/src/appkit/anwendung.rs:4174-4177`, mit `df8163d` neu geschrieben:

```
    /// [`Self::anlass_ausfuehren`] traegt denselben Nachzug seit demselben
    /// Befund, dort aber von Hand am Ende der Fortsetzung; ...
```

`git blame -L 6836,6836` weist die Zeile `self.aufteilung_nachziehen();` in `anlass_ausfuehren`
als `d18913e6` vom **260810** aus, dreizehn Tage vor `df8163d` und aus anderer Arbeit. Der
Kommentar darüber ist mit `df8163d` umgeschrieben, die Zeile selbst nicht. „Seit demselben
Befund" ist damit falsch; richtig wäre „seit dem 260810, aus demselben Grund".

Der Unterschied ist nicht nur eine Jahreszahl. Der Satz stellt `anlass_ausfuehren` als eine
zweite, gleichzeitige Antwort auf diesen Defekt dar. Tatsächlich war es die **erste** Antwort
auf dieselbe Klasse, dreizehn Tage früher und an einer einzelnen Stelle — genau die
Einzelfallbehandlung, deren Unvollständigkeit `df8163d` erst zu diesem Defekt geführt hat. Der
Befund wird dadurch unauffindbar, dass die frühere Antwort dieselbe Lücke schon einmal an einer
Stelle geschlossen hat, ohne die Quelle zu erreichen.

## Vorschlag

Zu 1: die Zählung durch eine Regel ersetzen, statt sie auf vier zu korrigieren. Etwa: „Wer
`aufteilung_nachziehen` ruft, hat entweder das Modell geändert oder vorher gemessen. Welche
Rufer messen und welche nicht, steht an ihnen." Eine Zahl an dieser Stelle ist in diesem Baum
zweimal veraltet, ohne dass es jemand bemerkt hätte; `CLAUDE.md` führt denselben Wechsel als
wiederkehrende Fehlerquelle (Abschnitt „Projektstand", zu `Kommando`).

Zu 2: den zweiten Halbsatz streichen oder umschreiben. Der Rückgabewert bleibt tragend, aber
für `sitzung_vormerken` und für die Anzeigen, die keine Sichtbarkeit sind — nicht mehr als
einziger Weg zur Auslegung.

Zu 3: „seit demselben Befund" durch das Datum ersetzen und den Satz dahin führen, dass jene
Stelle die erste, unvollständige Antwort auf dieselbe Klasse war.

**Schwere:** mittel. Kein Fehlverhalten am laufenden Bündel. Alle drei Stellen beschreiben
genau die Verdrahtung, die `df8163d` geändert hat, und die erste verdeckt einen offenen
Verhaltensbefund.

**Gefunden:** coderev, Durchsicht des Commits `df8163d` am 260823-0730, Bereich
`ab11eb8..df8163d`

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:3462-3468`, `:4174-4177`, `:4500-4506`

**Domain:** code

**Verwandt:**
`shared/issues/260823-0731_o_ein-klick-in-das-andere-dateifenster-nimmt-eine-ziehbewegung-zurueck.md`
— der Verhaltensbefund, den Punkt 1 verdeckt.
`shared/issues/260821-1401_o_zwei-mit-d771ec6-neu-geschriebene-prosastellen-der-ablage-geben-ihren-umfang-falsch-an.md`
und `shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`
— dieselbe Klasse in einem anderen Modul: eine Zählung, die eine Regel ersetzt hat, und die bei
ihrem Entstehen falsch war.

---

In Arbeit: 260823-1137 durch coder. Alle drei Stellen sind nachgezogen, und zwei
weitere derselben Wurzel dazu.

1. `bildschirmbreiten_uebernehmen`: die Zaehlung der Aufrufer ist durch eine Regel
   ersetzt. Der Satz nennt keine Zahl mehr und sagt ausdruecklich, dass
   `aktives_setzen` nicht misst und es auch nicht begruendet, mit Verweis auf
   `shared/issues/260823-0731_*`; wer ihn beim Wort nimmt, haelt jenen Fall jetzt nicht
   mehr fuer geprueft.
2. `ordner_angleichen`: der zweite Halbsatz ist umgeschrieben. Er sagt jetzt, dass
   `sichtbarkeit_aendern` die Sichtbarkeit selbst auf den Schirm schreibt und am
   Rueckgabewert `sitzung_vormerken` und die Anzeigen haengen, die keine Sichtbarkeit
   sind.
3. `sichtbarkeit_aendern`: „seit demselben Befund" ist durch „seit dem 260810 und aus
   demselben Grund" ersetzt, mit dem Satz, dass jene Stelle die erste, unvollstaendige
   Antwort auf dieselbe Klasse war.

**Zwei abhaengige Stellen, die dieser Datensatz nicht fuehrt**, beide von `df8163d`
falsch gemacht und aus derselben Menge:

4. Derselbe Doc-Kommentar an `bildschirmbreiten_uebernehmen` sagte „Zwei Anlaesse tun
   das" und zaehlte `kommando_ausfuehren` und `sitzung_bauen`. `df8163d` hat mit
   `editorausgang_behandeln` den dritten hinzugefuegt (`git log -L` weist die Zeile aus)
   und den Satz nicht angefasst. Er traegt jetzt die Regel und keine Zahl.
5. Der Kommentar im Rumpf von `sitzung_bauen` nannte sich „der zweite der **beiden**
   Anlaesse"; dieselbe Menge, dieselbe Verschiebung. Er verweist jetzt auf die Regel an
   `bildschirmbreiten_uebernehmen`.

Bleibt zum Schliessen mit dem Commit.

---
Resolved: `52fba42` — behoben, `make check` gibt 0 zurück. Durchsicht: die Befunde stammen aus
`shared/reviews/260823-0735-coderev-einblenden-erreicht-den-schirm.md` und
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`; was im Einzelnen getan ist, steht
im Protokoll `shared/history/260823-1137-coder-acht-befunde-aus-zwei-durchsichten.md`.
