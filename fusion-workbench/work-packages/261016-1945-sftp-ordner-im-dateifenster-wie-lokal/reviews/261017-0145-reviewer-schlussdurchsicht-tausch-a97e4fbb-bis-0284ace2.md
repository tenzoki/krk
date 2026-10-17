# Schlussdurchsicht des Tauschs beim Überschreiben, `a97e4fbb..0284ace2`

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `a97e4fbb..0284ace2`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.evidence.json`
**Domain:** code
**Work-item:** 261016-1945-sftp-ordner-im-dateifenster-wie-lokal
**Maßstab:** die vier Issues `261017-0113-*` mit ihrer Abnahme; die Frage der Dispatch, ob ein Weg weder altes Ziel noch vollständigen Ersatz noch Quelle unter einem genannten Namen lässt, oder die Quelle ohne vollständige Ankunft entfernt
**Urteil:** revise

## Zusammenfassung

Die vier Befunde der Nachdurchsicht sind am Code behoben. `tauschen` legt das Ziel per `RENAME` beiseite, setzt den Ersatz ein und entfernt das Alte zuletzt. Auf jedem Weg, den ich verfolgt habe, bleibt eine vollständige Fassung stehen.

Drei neue Befunde:
- **Medium:** Auf einem eingehängten Netzlaufwerk kann die Frage nach der Quelle aus dem Namenscache des Clients beantwortet werden. Dann verliert Verschieben mit „Überschreiben“ über zwei Wege die einzige Kopie. `inference:`, nicht gemessen.
- **Low:** Ein `RENAME` ohne Antwort wegen einer Trennung kann einen Zwischennamen hinterlassen, den der Bericht nicht nennt.
- **Low:** Verschieben über eine Grenze nennt einen vollständigen Ersatz „nicht vollständig kopiert“.

Geprüft am Baum `0284ace2`: `cargo test -p krk-core --test fern --test fern_operation`, 30 und 17 Proben grün. `make check` habe ich nicht gefahren.

## Summen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 2 |

## Die vier Befunde der Nachdurchsicht

| Befund | Stand | Beleg |
|---|---|---|
| N1 Zielordner zur Hälfte | behoben | `tauschen`: `beiseite_legen`, dann `zwischen.umbenennen_in`, dann `alt.baum_entfernen`; Probe `ueberschreiben_entfernt_einen_zielordner_nie_zur_haelfte` |
| N2 Rechte und Zeit | behoben für Kopieren | `Angabenweg::Ersatz`, `hinweise_nachtragen`, `eintrag_fertig` erst nach dem Tausch; beim Verschieben über eine Grenze neuer Befund L2 |
| N3 zwei Wege | behoben ohne Client-Cache | `tauschen` fragt nach Schritt 1 `quelle.angaben`; mit Cache neuer Befund M1 |
| N4 Texte | behoben | `FernRechnerUngueltig` in drei Sprachen mit Probe; `VorgangErsatzNichtWeggeraeumt` ohne „Abbruch“; `Erlaubnisbefund::erlaubt` entfernt; `HowTo.md` neu gefasst |

## Die Wege durch den Tausch

Verfolgt für Kopieren (`kopie_ersetzt_ziel`, Quelle `Some`) und Verschieben auf demselben Server (`umbenennen_ersetzt_ziel`, Quelle `None`). Das Verschieben über eine Grenze geht durch `kopie_ersetzt_ziel` und entfernt die Quelle danach allein ohne Übersprungenes (`ueber_die_grenze`).

| Schritt scheitert | Ziel | Ersatz / Quelle | Bericht nennt |
|---|---|---|---|
| Ersatz anlegen oder füllen, auch Abbruch | unverändert | Ersatz weggeräumt; scheitert das, liegt er | `.krk-ersatz-…` über `rest_wegraeumen` |
| Schritt 1, Beiseitelegen | unverändert | beim Kopieren weggeräumt; beim Verschieben zurück unter den Quellnamen | Zwischenname, wenn das scheitert |
| Frage nach der Quelle: fehlt | zurückgeholt | weggeräumt, Quelle bleibt | `VorgangQuelleIstTeilDesZiels` |
| Frage nach der Quelle: Fehler | zurückgeholt | weggeräumt | Grund |
| Schritt 2, Ersatz auf den Namen | zurückgeholt | wie Schritt 1 | wie Schritt 1 |
| Zurückholen scheitert | unter `.krk-alt-…` | unter `.krk-ersatz-…` | beide Namen (`beide_beiseite_melden`) |
| Schritt 3, Altes entfernen | Ersatz am Namen | Rest unter `.krk-alt-…` | `VorgangAltesZielBlieb` mit Namen |

Weitere Punkte, die ich gegen den Code gehalten habe:
- **Lokale Umbenennung schließt aus.** `umbenennen_in` geht lokal über `umbenennen::umbenennen` und `im_datentraeger_verschieben(…, true)`, also `renamex_np` mit `RENAME_EXCL`. Ein liegengebliebener `.krk-alt-…` wird nie still ersetzt.
- **Verknüpfung als Ziel.** `fern_baum_entfernen` und `loeschen::baum_entfernen` fragen `lstat`, entfernen also die Verknüpfung und nicht ihr Ziel.
- **Quelle im Ziel über eine Verknüpfung oberhalb.** Beim Kopieren löst das `lstat` nach Schritt 1 den Pfad neu auf und findet die Quelle nicht: `QuelleBeiseite`. Beim Verschieben auf demselben Server liegt die Quelle vor Schritt 1 schon unter `.krk-ersatz-…` außerhalb des Ziels.
- **Abbruch.** Während des Füllens räumt `datei_uebertragen` die halbe Datei weg, danach `rest_wegraeumen` den Rest. Der Tausch selbst fragt keinen Abbruch, und das ist richtig: er besteht aus zwei `RENAME` und einem Entfernen nach dem Einsetzen.
- **Trennung.** Ab der Trennung scheitert jeder Schritt mit `Getrennt`. Nach Schritt 1 ergibt das `BeideBeiseite` mit beiden Namen. Offen bleibt der Fall, in dem der Server ein `RENAME` noch ausgeführt hat (L1).
- **Fall- und Großschreibung auf dem Server.** Erkennt `REALPATH` eine Quelle unter anderer Schreibweise nicht, fängt beim Kopieren die Frage nach Schritt 1 sie, beim Verschieben scheitert Schritt 1 an `NO_SUCH_FILE`, und die Quelle kehrt zurück.

## Neue Befunde

### M1 (Medium). Die Frage nach der Quelle liest auf einem Netzlaufwerk den Namenscache

- `fern.rs` `tauschen`: Schritt 1 benennt über sftp um, dann fragt `quelle.angaben` über den lokalen Pfad.
- Auf einem eingehängten NAS (NFS, SMB) behält der umbenannte Eintrag sein Dateihandle. `inference:` ein Client mit noch gültigem Namenseintrag beantwortet `lstat` positiv.
- Dann wird getauscht, `alt.baum_entfernen` entfernt die Daten der Quelle, und `ueber_die_grenze` entfernt den Namen der Quelle, unter dem jetzt der Ersatz steht. Keine Kopie bleibt.
- `HowTo.md` nennt genau dieses NAS als Beispiel und sagt „die Quelle bleibt“.
- Die Probe `verschieben_mit_ueberschreiben_verliert_keine_quelle_die_ueber_zwei_wege_das_ziel_ist` fährt eine lokale Platte ohne Cache.
- Richtung: eine entscheidbare Frage statt der unentscheidbaren. Lokale Seite ohne `MNT_LOCAL` heißt „unklar“, und `ersetzen_erlaubt` ersetzt beim Verschieben über eine Grenze nicht.
- Issue `261017-0145-die-frage-nach-der-quelle-im-tausch-liest-auf-einem-netzlaufwerk-den-namenscache-und-verschieben-verliert-dann-die-einzige-kopie.md`.

### L1 (Low). Ein `RENAME` ohne Antwort hinterlässt einen ungenannten Zwischennamen

- `beiseite_legen` mit `Getrennt` ergibt `ZielBlieb`. Hat der Server umbenannt, liegt das Ziel unter `.krk-alt-…`, und der Bericht sagt „nicht ersetzt“.
- `umbenennen_ersetzt_ziel`, Umbenennen der Quelle auf den Zwischennamen: bei `Getrennt` liegt die Quelle womöglich unter `.krk-ersatz-…`, ungenannt.
- Inhalt geht nicht verloren. Der Name beginnt mit Punkt, und der Nutzer erfährt ihn nicht.
- Issue `261017-0145-ein-rename-dessen-antwort-mit-der-trennung-verloren-geht-hinterlaesst-einen-zwischennamen-den-der-bericht-nicht-nennt.md`.

### L2 (Low). Verschieben über eine Grenze nennt einen vollständigen Ersatz „nicht vollständig kopiert“

- `kopie_ersetzt_ziel`: `AltesBlieb` und die nachgetragenen Angabenhinweise überspringen unter dem Namen des Ziels.
- `ueber_die_grenze` findet sie in `seither`, `selbst_genannt` ist falsch, und die Quelle bekommt „nicht vollständig kopiert, in der Quelle geblieben“.
- Die Quelle steht zugleich als fertig und als übersprungen im Bericht.
- Issue `261017-0145-verschieben-ueber-eine-grenze-nennt-einen-vollstaendigen-ersatz-nicht-vollstaendig-kopiert.md`.

## `HowTo.md` gegen den Code

Der Absatz „Kopieren und Verschieben“ im sftp-Abschnitt stimmt mit dem Code:
- Reihenfolge der drei Schritte.
- „kommt der Ersatz nicht vollständig an, bleibt das Ziel, wie es war“.
- Der Name `.krk-alt-…` für einen Rest des alten Ziels.
- Ersetzen trotz abgelehnter Rechte und Zeit.

Abweichungen:
- Der Satz zu zwei Wegen zum selben Speicher gilt nicht unter Client-Cache (M1).
- Beim Verschieben über eine Grenze sagt `HowTo.md` nicht, dass die Quelle nach einem Hinweis bleibt (L2).

## Querschnitt

- **Was ein Abfrageergebnis über einen zweiten Weg sagt.** M1 und L1 haben dieselbe Wurzel: der Code liest eine Antwort als Zustand des Servers, die das nicht ist. In M1 ist es ein Cache, in L1 eine fehlende Antwort. Beide Fragen sind vom Client nicht entscheidbar. Die Behebung ist jeweils eine andere Frage oder ein Bericht, der beide Möglichkeiten nennt, keine genauere Abfrage.
- **„Übersprungen“ trägt weiter zwei Bedeutungen.** `kopie_ersetzt_ziel` trennt sie jetzt über `Angabenweg`, `ueber_die_grenze` nicht (L2).

## Reihenfolge

1. Vor dem Nutzertest gegen `ssh.strato.de`: keiner der drei hält ihn auf. Er geht über zwei Server-Wege nur, wenn der Nutzer dasselbe Konto unter zwei Kennungen öffnet, und dort gibt es keinen Client-Cache.
2. M1 vor einer Auslieferung: zumindest die Grenze in `HowTo.md` und im Modulkopf.
3. L1 und L2 als Aufräumen.
