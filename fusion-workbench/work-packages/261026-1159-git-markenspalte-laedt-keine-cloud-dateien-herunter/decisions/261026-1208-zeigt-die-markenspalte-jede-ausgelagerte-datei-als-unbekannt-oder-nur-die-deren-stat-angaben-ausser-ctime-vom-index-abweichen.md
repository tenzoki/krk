# Zeigt die Markenspalte jede ausgelagerte Datei, deren Stand sie nicht ohne Lesen kennt, als „unbekannt“, oder gilt eine ausgelagerte Datei als unverändert, wenn ihre stat-Angaben außer ctime zum Index passen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261026-1159-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`, `261026-1208-plan-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`, `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md`, `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`

---

## Question

`gix` vergleicht eine verfolgte Datei zuerst über ihre stat-Angaben mit dem Index und liest ihren Inhalt nur, wenn diese abweichen oder der Eintrag „racy“ ist (`gix-status-0.34.1/src/index_as_worktree/function.rs`, `compute_status`). Der Plan setzt an genau dieser Stelle an: eine Datei mit `SF_DATALESS` wird dort nie gelesen. Offen ist, was die Spalte dann für sie sagt.

**Gemessen am 261026-1208** an drei ausgelagerten Dateien der Wegwerfkopie auf Google Drive (`stat -f` gegen `git ls-files --debug`): nach dem Auslagern weicht allein `ctime` vom Index ab; `mtime` samt Nanosekunden, Größe, Gerät, Inode, `uid` und `gid` sind gleich. Das Auslagern selbst setzt also jede ausgelagerte Datei auf „stat weicht ab“, ohne ihren Inhalt zu ändern. Die Antwort entscheidet, ob in einem Repository auf Google Drive im Streaming-Modus fast jede Zeile ein „?“ trägt oder nur die, über die KRK wirklich nichts weiß. Sie bindet die Bedeutung der Marke über diesen Plan hinaus, und sie weicht in einer Möglichkeit vom Vorschlag im Auftrag ab („zeigt für sie ‚unbekannt‘“), deshalb entscheidet der Nutzer.

## Options

1. **Jede ausgelagerte Datei, deren Inhalt `gix` lesen wollte, trägt „unbekannt“.** Der Wortlaut des Vorschlags im Auftrag.
   - Pros: die Spalte behauptet über eine ausgelagerte Datei nichts, was nicht schon die stat-Angaben samt ctime belegen. Einfachste Regel, eine Zeile.
   - Cons: weil das Auslagern ctime hebt, trägt **jede** ausgelagerte verfolgte Datei „?“, solange sie ausgelagert ist; bei der Messung vom 261026 wären es 261 von 261 Dateien unter `crates/`. Nach der Rangregel des Plans trägt dann auch fast jeder Ordner „?“, wenn nicht ein bekanntes „M“, „U“ oder „K“ darüber steht. Auf einem Laufwerk im Streaming-Modus sagt die Spalte damit über die meisten Zeilen nichts mehr. Kehrt eine Datei lokal zurück, verschwindet ihr „?“, weil `gix` sie dann liest.
2. **Eine ausgelagerte Datei gilt als unverändert, wenn ihre stat-Angaben ohne ctime zum Index passen und der Eintrag nicht racy ist; sonst „M“, wenn die Größe abweicht, und „unbekannt“ in jedem anderen Fall.** KRK prüft das mit `gix`s eigenen Funktionen `Stat::matches` (mit `trust_ctime: false`, die übrigen Schalter wie im Repository) und `Stat::is_racy`, allein für Dateien mit `SF_DATALESS`; für lokale Dateien bleibt alles wie heute.
   - Pros: die Spalte zeigt für eine ausgelagerte Datei, was `git status` über eine lokale Datei aus denselben Angaben schlösse; „?“ steht nur dort, wo `mtime`, Inode oder die Racy-Prüfung wirklich eine mögliche Änderung anzeigen (etwa eine Fassung, die der Provider aus der Cloud als Platzhalter neu angelegt hat, oder eine Datei, die gleich nach dem Index geschrieben wurde). Die Größe als Beweis einer Änderung ist dieselbe Regel, die `gix` heute über `FastEq` anwendet, ohne zu lesen.
   - Cons: die Aussage „unverändert“ ruht für eine ausgelagerte Datei auf stat ohne ctime. Eine Änderung, die mtime, Größe und Inode unberührt lässt, sähe KRK nicht; dieselbe Lücke hat `git` mit `core.trustctime=false`. Dass das Auslagern allein ctime hebt, ist an drei Dateien eines Providers gemessen; der Abnahmeschritt des Plans misst es an allen ausgelagerten Dateien nach.
3. **Wie 2, aber global über `core.trustctime=false` für jedes Repository und jede Datei.**
   - Pros: kein eigener Prüfweg in KRK, `gix` tut es selbst.
   - Cons: ändert die Auskunft auch für lokale Dateien, wo kein Grund dafür besteht; verlangt eine Konfigurationsüberlagerung beim Öffnen, die das Repository des Nutzers anders liest als `git`. Verworfen.

## Constraints

- Keine ausgelagerte Datei wird geöffnet (Entscheid `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`, Möglichkeit 1: ein Hintergrundleser öffnet keine Platzhalterdatei und nennt sie ungelesen).
- „Unverändert“ darf nie eine Auskunft sein, die KRK aus einem halb gelesenen Stand schließt (Modulkopf von `crates/krk-core/src/git/mod.rs`, „`None` heißt unentschieden“).
- Keine Option schreibt den Index zurück.

## Recommendation

Möglichkeit 2. Unter Möglichkeit 1 macht der Mechanismus, der das Herunterladen verhindert, die Spalte auf genau den Laufwerken nutzlos, für die er gebaut wird, und zwar wegen einer Angabe, von der gemessen ist, dass das Auslagern sie ändert und der Inhalt nicht. Möglichkeit 2 nimmt diese eine Angabe allein für ausgelagerte Dateien aus und lässt jede andere Abweichung „?“ bleiben. Der Plan baut beide Möglichkeiten an derselben einen Stelle; zwischen ihnen liegt ein Zweig und eine Probe.

Answered: 261026, vom Nutzer im Chat: Möglichkeit 2. Bei einer ausgelagerten Datei zählt ctime nicht; „?“ steht nur, wo mtime, Größe, Inode oder die Racy-Prüfung eine mögliche Änderung anzeigen. Umgesetzt wird es in Schritt 2 des Plans `261026-1208-plan-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`.
