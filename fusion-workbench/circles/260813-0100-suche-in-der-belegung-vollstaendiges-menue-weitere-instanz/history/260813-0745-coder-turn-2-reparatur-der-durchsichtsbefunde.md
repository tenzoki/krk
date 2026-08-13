# Turn 2 der siebten Runde: die Befunde der zwei Durchsichten

**Agent:** coder
**Datum:** 260813-0745
**Status:** Complete
**Auftrag:** die Befunde aus `reviews/260813-0532-ontorev-…` und `reviews/260813-0540-coderev-…`
beheben. Kein Vordergrundlauf, kein Buendelbau. An `resources/default-keymap.toml` allein
Kommentare.

---

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | exit 0, 19 Ziele, 1003 Proben |

Vorher 19 Ziele mit 1000 Proben. Die drei neuen sind
`ohne_sitzungsrecht_entsteht_kein_sitzungsschreiber`,
`nur_benannte_dateien_erreichen_das_atomare_schreiben` und
`eine_aufrufzaehlung_sieht_jede_schreibweise_und_keine_nennung`. Vier Proben sind aus
`krk-core/src/ablage/sperre.rs` nach `krk-core/tests/ablage.rs` umgezogen; die Zahl aendert das
nicht.

## Der Faden durch die Runde: eine Zusage, die an einer Schreibweise haengt, haelt nichts

Der `coderev` nennt es das durchgehende Muster der Runde, und drei Befunde sagen dasselbe aus
drei Richtungen. Sie sind deshalb **gemeinsam** behoben und nicht dreimal einzeln.

**Der Kopf von `crates/krk-ui/src/quellbaum.rs` trug den Fehlschluss.** Dort stand, eine
Erklaerungszaehlung „haelt, was sie verspricht". Die Runde hat den Gegenbeweis selbst geliefert:
eine vierte Pruefordner-Fassung namens `Ordner` stand im Baum, und die Probe, die genau sie
zaehlen sollte, sah sie nicht. Richtig ist: sie haelt gegen eine Kopie **unter demselben Namen**.
Ob irgendwo dieselbe Sache unter anderem Namen oder in anderer Schreibweise noch einmal gebaut
ist, entscheidet keine Suche im Quelltext — das ist keine Nachlaessigkeit der Nadel, sondern eine
Eigenschaft der Frage.

Der Abschnitt zieht drei Folgerungen und ist damit die Bauanleitung fuer jede neue Zaehlprobe
dieses Baums:

1. **Nach dem Gegenstand suchen, wo es geht, und nicht nach seinem Namen.**
2. **Jede Schreibweise erfassen, die der Baum schon kennt.**
3. **Die verbleibende Blindheit am Doc-Kommentar benennen** statt sie im Namen zu ueberschreiben.

Alle drei sind angewandt und nicht nur aufgeschrieben:

- **Gegenstand statt Name.** `genau_drei_pruefordner_fassungen_stehen_im_baum` sucht nicht mehr
  `impl Drop for Pruefordner`, sondern `impl Drop for `, `temp_dir()` und `remove_dir_all` in
  derselben Datei. Das findet jede vierte Fassung, gleich wie sie heisst.
- **Jeden Weg statt einer Schreibweise.** `nur_benannte_dateien_erreichen_das_atomare_schreiben`
  ist der Sonderfall, in dem die Vollstaendigkeit erreichbar ist: es gibt in Rust genau zwei Wege
  an eine fremde Funktion, den Pfad an der Aufrufstelle oder ein `use`, und beide nennen das
  Modul. Die Probe sucht `atomar::schreiben`, `atomar::{` und `atomar::*`, und ein dritter Weg
  besteht nicht.
- **Aufruf statt Aufrufschreibweise.** `quellbaum::aufrufstellen` zaehlt den Namen mit Klammer
  und zieht drei Sorten Nicht-Aufrufe ab: Treffer mitten in einem laengeren Namen, die Erklaerung
  selbst, Nennungen in Kommentaren. Jede Empfaengerform und jeder Pfad bleiben drin.
- **Blindheit benannt.** Vier Doc-Kommentare sagen jetzt, was ihre Probe nicht faengt: eine
  dritte Schreibweise der Ersthelferfrage, eine ueber zwei Dateien verteilte Pruefordner-Fassung,
  ein ueber zwei Zeilen umbrochener Pfad, ein `use … as`.

**Zwei Vorkehrungen gegen den Selbstfund**, und beide sind noetig. Die Nadeln stehen
zusammengesetzt da, wie seit jeher, und die neuen Proben suchen zusaetzlich nur in Code-Zeilen
(`im_code` in `tests/baum.rs`) — ohne das fanden sie sich selbst, weil ihre Doc-Kommentare die
Nadeln im Klartext nennen. Denselben Fehler hat die erste Fassung der Werkzeugprobe gemacht: ihr
Beispielquelltext nannte `zulaessig`, und zwei Zaehlproben in anderen Dateien wurden davon rot.
Der Beispielname ist jetzt erfunden.

## Was einzeln zu berichten ist

**Der Messmodus (C3.9) ist am Typ behoben und nicht mit einem Satz.**
`Sitzungsschreiber::neu` verlangt jetzt ein `&Sitzungsrecht` und liefert `Option<Self>`. Die
Regel „nur die Halterin schreibt die Sitzung" stand bis dahin an einem fehlenden Wert; ein
fehlender Wert ist eine Abmachung, und genau daran ist der Messmodus vorbeigelaufen.
`Messplan::herstellen` nimmt das Recht und bricht ohne es ab — eine Zahl auf fremder Lage waere
keine.

**Die vierte Pruefordner-Fassung ist fort, ohne den Nutzer zu fragen.** Der Datensatz stellt die
Frage „Ist die vierte Fassung erlaubt?" dem Nutzer. Sie ist in `CLAUDE.md` bereits beantwortet
(„genau drei Fassungen, eine je Kiste, und das soll so bleiben"); eine bestehende Zusage
einzuhalten ist keine Entscheidung. Die Begruendung fuer die vierte traegt dazu nicht: keine der
vier Proben in `sperre.rs` ruft das kistenintern sichtbare `Schreibgriff::nehmen`. Sie stehen
jetzt in `tests/ablage.rs` beim anerkannten `Pruefordner`.

**Der Quellbaumleser von `krk-ui` liest jetzt alle Kisten.** Das ist der teurere der zwei
vorgeschlagenen Wege und der einzige, der C2.16 wirklich deckt; der billigere haette die
Doc-Kommentare auf „in `krk-ui`" umgeschrieben und die Zusage ungedeckt gelassen. Neun
Erwartungen in sieben Dateien sind mitgezogen. Vor der Umstellung ist nachgesehen worden, dass
keine der vierzehn Nadeln ausserhalb von `krk-ui` vorkommt; keine Probe hat ihre Aussage
geaendert.

**Der Modulkopf der Ablage sagt jetzt, was die Typen nicht halten.** Er behauptete, „kein
Schreibweg an der Sperre vorbei" sei eine Eigenschaft der Typen. Sie ist es fuer den Weg **aus
der Ablage heraus** und fuer keinen anderen: `atomar::schreiben` ist `pub`, und `Ablage::pfad`
liefert den Pfad ohne Durchgang. Der Vorschlag, `Ablage::pfad` auf `pub(crate)` zu setzen, ist
nicht umgesetzt — der Datensatz nennt drei Aufrufer ausserhalb der Kiste, nachgezaehlt sind es
ueber vierzig, und die grosse Mehrheit **liest** damit. Zwei Proben, die den Weg zum **Schreiben**
nahmen, gehen jetzt durch einen Durchgang; eine davon (`tests/ablage.rs`, `settings.toml`) hatte
der Befund nicht mitgezaehlt.

**Fuenf Zahlen in der Prosa sind gestrichen und nicht nachgezogen.** Eine nachgezogene Zahl
veraltet beim naechsten Schritt wieder, und keine Probe haelt sie; in dieser Runde ist es bereits
zweimal geschehen. Stehen bleiben die neun Obermenues (durch C2.3 und eine Probe gehalten) und
die zehn Eintraege von vorher (ein abgeschlossener historischer Stand).

**Die Belegungsdatei ist ausschliesslich an Kommentarzeilen angefasst.** Nachgesehen mit
`git diff | grep -vE '^[+-]#'`: keine Zeile ausserhalb eines Kommentars ist geaendert. Neben den
zwei Befunden ist die Beobachtung ohne Befundcharakter des `ontorev` mitgenommen — die
Eingabetaste wird vom Blatt **hinter** dem Nachschlag abgefangen und nicht davor.

**Eine Beobachtung ohne eigenen Datensatz ist mitgenommen:** `behandeln` rief
`getipptes_zeichen(ereignis)` zweimal. Der Wert steht jetzt in einer Bindung, ein Fremdaufruf
weniger auf dem Tastendruckpfad, an dem L1 haengt.

## Datensaetze

**Achtzehn geschlossen** (`_o_` → `_c_`, je mit `Resolved:`-Absatz):

`260813-0201` Taste-Variante · `260813-0532` Dateikopf Fokusvorbehalt · `260813-0534`
Blockreihenfolge · `260813-0540` Messmodus · Schreibweg an der Sperre · vierte
Pruefordner-Fassung · Ersthelfer-Zaehlprobe · Zaehlproben in krk-ui · Aufruferzaehlungen ·
setEnabled · Kuerzelfilter · zwei Durchgaenge beim Beenden · Lesezeichenmeldung ·
Doc-Kommentar „Bearbeiten" · Doc-Kommentar in tests/belegung.rs · Kommentar in `behandeln` ·
fuenf Stellen mit alten Zahlen · Buendelort zweimal.

**Vier bleiben offen:**

- `260813-0540_o_die-belegung-wird-weiter-blind-ueberschrieben-…` — unberuehrt, wie beauftragt.
  Zu entscheiden ist, ob der Ablage-Datensatz nachgezogen oder die Belegung nachgebaut wird; das
  gehoert dem Nutzer.
- `260813-0420` Menue „Bearbeiten" — Datendomaene. Der Absatz im Dateikopf, den seine Behebung
  braucht, steht seit heute; die Verschiebung der zwei Bloecke gehoert dem `ontocoder`.
  Zwischenstand angehaengt.
- `260813-0416` cmd+a — die zwei Saetze, die der `ontorev` fuer die Belegungsdatei empfiehlt,
  sind bewusst nicht geschrieben: welche Funktion das Menuekuerzel behaelt, entscheidet die offene
  Frage `decisions/260813-0430`. Faellt sie anders aus, beschrieben die Saetze den falschen
  Mechanismus. Zwischenstand angehaengt.
- `260813-0311` Klick in die Bereichsleiste — unveraendert. Die Runde faehrt auf Weg 1, der
  Verlust steht auf der Abnahmeliste des Laufs am Buendel.

## Geaenderte Dateien

`crates/krk-core/src/ablage/mod.rs`, `.../ablage/sitzung.rs`, `.../ablage/sperre.rs`,
`crates/krk-core/tests/ablage.rs`, `.../tests/baum.rs`, `.../tests/belegung.rs`,
`.../tests/gemeinsam/mod.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`.../appkit/belegungsansicht.rs`, `.../appkit/blaetter/mod.rs`, `.../appkit/ereignisse.rs`,
`.../appkit/menue.rs`, `.../appkit/teilen.rs`, `.../appkit/weitereinstanz.rs`,
`crates/krk-ui/src/belegungsmodell.rs`, `.../kommandos/zulaessigkeit.rs`, `.../menuemodell.rs`,
`.../messmodus.rs`, `.../quellbaum.rs`, `resources/default-keymap.toml`.

Nicht committet; der Orchestrator commitet.
