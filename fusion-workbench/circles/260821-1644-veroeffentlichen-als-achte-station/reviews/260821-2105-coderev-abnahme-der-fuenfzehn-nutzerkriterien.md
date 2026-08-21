# Abnahmeprüfung: die fünfzehn Kriterien, die auf den Nutzer warteten

**Reviewed-range:** `94855a7..26212b1`
**Not-opened:** `Cargo.lock`, `Makefile`, `README.md`, `fusion-workbench/shared/history/260821-1532-reconciliation.md`, `fusion-workbench/shared/history/260821-1740-coder-aufsicht-liest-plaetze-statt-freier-woerter.md`, `fusion-workbench/shared/history/260820-2200-orchestrator-session.md`, `fusion-workbench/shared/reviews/260821-1432-coderev-auftrag-statt-nackter-wortliste.md`, `fusion-workbench/circles/260821-1644-veroeffentlichen-als-achte-station/history/260821-1644-shaper-veroeffentlichen-als-achte-station.md`, `fusion-workbench/circles/260821-1644-veroeffentlichen-als-achte-station/history/260821-2044-coder-readme-umbau-nutzer-vor-entwickler.md`

**Art:** Abnahmeprüfung, kein Durchsichtslauf über den Code. Gemessen wird gegen den
Wortlaut des Specs `shared/planning/260821-1115_o_spec-artefakt-und-release.md`, nicht
gegen eine Zusammenfassung. Die neun nicht geöffneten Dateien liegen außerhalb der
fünfzehn Kriterien; der Code, an dem sie hängen — `xtask/src/veroeffentlichung.rs`,
`xtask/src/git.rs`, `xtask/src/release.rs` — ist geöffnet.

**Gegenstand:** das Release <https://github.com/tenzoki/krk/releases/tag/v0.5.6>, sein
Anhang `KRK-0.5.6.zip`, der Stand `71a9920` auf der Gegenseite, und der Baum `26212b1`.

**Datum:** 260821-2105

---

## Verdikt

**Fünfzehn geprüft. Vierzehn halten. Eines ist auf diesem Gerät nicht prüfbar.**
Keines hält nicht.

Das nicht prüfbare ist C2.2, „ein zweiter Mac ohne Netzverbindung". Ein zweiter Mac steht
nicht zur Verfügung, und die Netzverbindung dieses Geräts lässt sich für die Messung nicht
trennen. Was daran ohne zweiten Mac prüfbar ist, ist geprüft und steht unten; was
ungeprüft bleibt, steht ebenfalls dort und wird nicht als Abnahme ausgegeben.

Für den Abschluss des Circles heißt das: **alle prüfbaren Kriterien halten.**

---

## Die Tabelle über alle fünfzehn

| Kriterium | Befund | Beleg |
|---|---|---|
| **C1.1** eigenständiger Lauf, ein Argument, keine Umgebungsvariable | **hält**, mit benannter Restlücke | Läufe vom 260821-2100 und -2101 in einem Wegwerfklon: `cargo xtask veroeffentlichen 0.5.6`, ein Argument, keine Umgebungsvariable, keine Rückfrage. Fünf der sechs Schritte laufen durch (gh-Prüfung, Tagfrage, Bündelfrage, Ticketprüfung, Packen, Schieben). Der sechste, das Anlegen, kann für `0.5.6` nicht gelingen, weil das Release steht — genau das, was C4.10 verlangt. Er ist am Lauf vom 20:24:46 desselben Tages belegt, durch denselben Rumpf `veroeffentlichung::veroeffentlichen`. |
| **C1.4** `release` fährt die achte Station ohne zweites Kommando | **hält** | Zeitkette am ausgelieferten Baum, auf die Sekunde: Commit `71a9920` 20:22:12 → x86-Ziel 20:23:01 → arm-Ziel und `lipo` 20:23:44 → Signatur 20:23:44 → **Ticket angeheftet 20:24:11** → **Zip gepackt 20:24:14** → Anhang hochgeladen 20:24:42 → Release veröffentlicht 20:24:46. Drei Sekunden zwischen Heften und Packen: kein zweites Kommando ist dazwischen getippt worden. Dazu `xtask/src/release.rs:242`, der Aufruf ist die letzte Anweisung von `release::ausfuehren` und hängt an keiner Bedingung. |
| **C1.5** der Befehl baut nichts, gemessen an den Änderungszeiten | **hält** | Manifest aller elf Einträge unter `target/KRK.app` vor und nach dem Lauf vom 260821-2100, `stat -f %m`: **kein Eintrag verändert**. In `target/` kommt allein `KRK-0.5.6.zip` hinzu — das ist C2.1 und keine Bauarbeit. Der Plan hat zurecht darauf bestanden, das an Zeiten und nicht an der Quelltextprobe zu messen. |
| **C2.1** `target/KRK-<zahl>.zip` liegt da, die Zahl ist die des Arguments | **hält** | Nach dem Lauf: `target/KRK-0.5.6.zip`, 6 908 618 Bytes. Argument war `0.5.6`. |
| **C2.2** zweiter Mac ohne Netz, keine Gatekeeper-Rückfrage | **nicht prüfbar** | Kein zweiter Mac vorhanden, die Netzverbindung nicht trennbar. Geprüft wurde stattdessen der Mechanismus, auf dem die Zusage beruht (siehe unten „Was an C2.2 gemessen ist und was nicht"). |
| **C2.3** das aus dem Zip entpackte Bündel trägt das angeheftete Ticket | **hält**, an zwei unabhängigen Mitteln | Das Zip **von der Releaseseite**, ohne Anmeldung geladen, entpackt: `Contents/CodeResources` beginnt mit `73 38 63 68` (`s8ch`), 1 674 Bytes. Zweites Mittel, vollständig ohne Netz: der CDHash des Bündels ist `f4f788ae…e611b24`, und **diese Bytefolge steht roh im angehefteten Ticket**. Das Ticket gehört also diesem Bündel und keinem anderen, und das ist an der Datei allein entscheidbar. |
| **C2.4** zweiter Lauf mit derselben Zahl schreibt die Datei neu | **hält** | Dritter Lauf im Klon, 260821-2100: die Datei stand da (mtime 21:00:13), nach dem Lauf mtime 21:00:49, gleiche Größe, gleiche SHA-256. Kein Abbruch an ihrer Existenz. Die Prüfsumme ist dabei identisch mit der des Anhangs von der Releaseseite: `bf683f4a…aad70`. |
| **C3.1** Tag und Zweig stehen auf der Gegenseite | **hält** | Am echten `origin`: `refs/tags/v0.5.6` = `71a9920` und `refs/heads/main` = `71a9920`, also der Stand, den der Lauf geschoben hat. Kontrollmessung gegen eine Ersatzgegenseite, deren `main` auf `5d363de` stand: nach dem Lauf steht sie auf `71a9920`, dem lokalen HEAD. Beide Hälften des Kriteriums gemessen. |
| **C3.2** ohne Tag auf HEAD bricht er ab und schiebt nichts | **hält** | Lauf im Nutzerbaum am 260821-2059, HEAD `26212b1` trägt keinen Tag. Rückgabewert 1, Meldung nennt den erwarteten Namen `v0.5.6` und daneben, dass auf HEAD überhaupt kein Tag steht. `git ls-remote origin` vor und nach dem Lauf **Byte für Byte gleich**, 17 Referenzen. |
| **C3.3** die Zahl der Tags auf der Gegenseite wächst um genau eins | **hält**, an einer Kontrollmessung | Ersatzgegenseite mit 14 Tags bestückt (alle außer `v0.5.6`), Lauf gefahren, danach 15. Der Unterschied der Referenzlisten ist **genau zwei Zeilen**: `main` gerückt, `refs/tags/v0.5.6` neu. Dazu die Quelle: `git::Auftrag::Schub` (`xtask/src/git.rs:345`) trägt vier Wörter, `push origin HEAD <verweis>`, mit einem einzigen Tagverweis. Mehr als ein Tag kann daraus nicht hinausgehen. |
| **C4.1** `releases/latest` zeigt `v<zahl>` | **hält** | `curl` ohne Token auf `https://github.com/tenzoki/krk/releases/latest`: 302 auf `…/releases/tag/v0.5.6`, Endstatus 200. |
| **C4.2** genau eine Datei, ohne Anmeldung ladbar | **hält** | Anonyme API: ein Anhang, `KRK-0.5.6.zip`, 6 908 618 Bytes. Anonym geladen (`curl` ohne jeden Token, `GH_TOKEN` geleert): Status 200, `content-length: 6908618`, SHA-256 identisch mit dem lokalen `target/KRK-0.5.6.zip`. |
| **C4.3** kein Entwurf, keine Vorabfassung, ohne Anmeldung sichtbar | **hält** | Anonyme API: `draft: false`, `prerelease: false`. Die HTML-Seite antwortet ohne Anmeldung mit 200, führt den Anhang und den Abschnitt „Die alte Fassung vorher nicht löschen"; kein Merkmal „draft" und keins für eine Vorabfassung steht darauf. |
| **C4.10** ein zweiter Lauf bricht ab und überschreibt nichts | **hält** | Zweiter Lauf am 260821-2100 gegen das echte GitHub. Rückgabewert 1, Meldung: „Auf der Gegenseite steht bereits ein Release v0.5.6. Es wird nicht ueberschrieben…". Vergleich des Releasestands vorher/nachher, Feld für Feld: **Kennung des Anhangs gleich** (`RA_kwDOTqZEp84fPQkS`), **Erstellungszeit gleich** (`2026-08-21T18:24:42Z`), **Änderungszeit gleich**, **Zahl der Anhänge gleich** (1), Größe, Titel, Text der Seite, `publishedAt` — alles gleich. Der Abbruch fällt an der getrennten Existenzfrage, nicht erst am Anlegeversuch. |
| **C5.2** `gh` vorhanden, nicht angemeldet | **hält** | `GH_CONFIG_DIR` auf ein leeres Wegwerfverzeichnis gesetzt, `GH_TOKEN` und `GITHUB_TOKEN` geleert; `gh auth status` gibt 1 zurück. Der Lauf bricht mit Rückgabewert 1 ab und nennt `gh auth login`. Nichts gepackt (`target/KRK-0.5.6.zip` behält seine Zeit 20:24), nichts geschoben. **Die Anmeldung des Nutzers ist dabei unangetastet geblieben.** |

**Zählung: 15 geprüft, 14 halten, 1 nicht prüfbar, 0 halten nicht.**

---

## Was an C2.2 gemessen ist und was nicht

Das Kriterium lautet: „Entpackt man das Zip auf einem zweiten Mac ohne Netzverbindung und
startet die App, erscheint keine Gatekeeper-Rückfrage." Zwei Bedingungen, und beide fehlen
hier: es gibt keinen zweiten Mac, und die Netzverbindung dieses Geräts lässt sich für die
Messung nicht abtrennen.

**Was geprüft ist**, am Zip von der Releaseseite, anonym geladen und entpackt:

- Das angeheftete Ticket ist da und gehört diesem Bündel. Der CDHash `f4f788ae…e611b24`
  steht roh im Ticket. Diese Prüfung braucht kein Netz — sie liest eine Datei.
- Die Signatur trägt `flags=0x10000(runtime)`, also die gehärtete Laufzeitumgebung, und
  die Developer-ID `Kai Stalmann (QYMPYB7MWM)`.
- `codesign --verify --deep --strict`: gültig, erfüllt seine Designated Requirement.
- Mit gesetztem Quarantäne-Merkmal — der Zustand, in dem eine geladene Datei ankommt —
  sagt `spctl -a -vvv -t exec`: `accepted, source=Notarized Developer ID`.

**Was ungeprüft bleibt:** ob das auf einem Gerät gilt, das den Signierschlüssel nicht im
Schlüsselbund hat, und ob es ohne Netz gilt. Die zweite Frage ist die inhaltlich
wichtigere, und der geprüfte Mechanismus ist genau der, aus dem die Antwort folgen soll:
Gatekeeper liest ohne Netz das angeheftete Ticket und vergleicht dessen CDHash mit dem des
Bündels. Beides liegt vor und passt zusammen. **Das ist ein starkes Indiz und keine
Abnahme**, und es wird hier nicht als eine ausgegeben. Für die Abnahme braucht es den
zweiten Mac.

---

## Berichtigung an zwei Messungen des Auftrags

Der Auftrag hat darum gebeten, seine Zahlen nachzumessen statt zu übernehmen. Eine ist zu
berichtigen.

**„Auf `origin` stehen 14 Tags" — es sind 15.** Nachgemessen auf drei Wegen, alle drei
sagen 15: `git ls-remote --tags origin` ohne die `^{}`-Zeilen, `gh api repos/tenzoki/krk/tags`
mit Seitendurchlauf, `gh api repos/tenzoki/krk/git/refs/tags` mit Seitendurchlauf. Lokal
stehen ebenfalls 15, und die zwei Listen sind deckungsgleich. Die Zahl trägt für C3.3:
wer von 14 auf der Gegenseite und 15 lokal ausgeht, schließt daraus, dass ein Tag noch
fehlt und der Lauf ihn nicht geschoben hat. Er fehlt nicht.

Die übrigen Angaben des Auftrags sind nachgemessen und stimmen: `s8ch` am Anfang von
`Contents/CodeResources`, `accepted, source=Notarized Developer ID`, `flags=0x10000(runtime)`,
`CFBundleShortVersionString` `0.5.6`, `x86_64 arm64`, `refs/heads/main` und
`refs/tags/v0.5.6` beide auf `71a9920`, `releases/latest` auf `v0.5.6`, das Zip
6 908 618 Bytes.

---

## Was der Wortlaut von C3.3 nicht mehr hergibt

C3.3 ist als Kontrollmessung abgenommen und nicht am echten Lauf vom 20:24. Der Grund
gehört dazu: **die Zahl der Tags auf der Gegenseite vor jenem Lauf ist nirgends
festgehalten.** Der Spec zählt sie am 260821 als eins, danach hat der Nutzer den einmaligen
Handgriff `git push origin --tags` gefahren, und ob `v0.5.6` zu diesem Zeitpunkt schon
lokal stand, sagt kein Datensatz und keine Terminalaufzeichnung. Aus dem heutigen Stand
folgt die Wachstumszahl jenes Laufs nicht.

Die Kontrollmessung ersetzt sie: eine Gegenseite mit 14 Tags, ein echter Lauf des
gebauten Befehls, danach 15 und genau zwei geänderte Referenzen. Zusammen mit dem
Wortlaut von `Auftrag::Schub`, der einen einzigen Tagverweis trägt, ist die Zusage
gemessen. Das ist kein Mangel des Befehls, sondern eine Lücke in der Aufzeichnung des
einen historischen Laufs, und sie wird hier benannt statt überspielt.

---

## Ein Befund, der neben den fünfzehn liegt

Ein eigener Datensatz ist abgelegt:
`circles/260821-1644-veroeffentlichen-als-achte-station/issues/260821-2105_o_ein-angemeldetes-gh-das-das-vorhaben-nicht-erreicht-schiebt-erst-und-nennt-dann-die-falsche-abhilfe.md`.

Kurz: `gh_pruefen` fragt nach dem Konto und nicht nach dem Vorhaben. Ein `gh`, das
angemeldet ist, dessen Gegenstelle aber nicht adressierbar ist, kommt durch die
Vorprüfung; der Lauf packt und schiebt, und erst das Anlegen scheitert. Die Meldung sagt
dann „derselbe Aufruf noch einmal holt sie nach", und das ist für einen Zeitüberlauf
richtig und für ein unerreichbares Vorhaben falsch. Gemessen im Wegwerfklon gegen eine
Ersatzgegenseite am 260821-2101.

**Kein Abnahmekriterium fällt daran.** C5.1 und C5.2 nennen genau zwei Lagen, und beide
halten. Was nicht hält, ist der Begründungssatz des Specs zu C5, „eine fehlende
Voraussetzung soll auffallen, solange noch nichts geschehen ist".

---

## Wie gemessen wurde, und was dabei am Baum geschehen ist

Der Arbeitsbaum des Nutzers ist vor und nach der Prüfung sauber, HEAD steht unverändert
auf `26212b1`, `target/KRK.app` und `target/KRK-0.5.6.zip` tragen ihre Zeiten vom Lauf um
20:23 und 20:24. Gelöscht oder zurückgenommen ist nichts.

Die zwei Läufe im Nutzerbaum (C5.2, C3.2) brechen beide vor dem ersten Wirken ab. Die drei
Läufe, die packen und schieben, sind in einem Wegwerfklon unter `/tmp` gefahren, auf einen
eigenen Zweig `main` bei `71a9920` gesetzt, mit dem hineinkopierten beglaubigten Bündel.
Zwei davon gingen gegen das echte `origin` — beide Schübe waren dort wirkungslos, weil
`main` und der Tag schon auf `71a9920` standen; die Referenzliste ist vor und nach den
Läufen Byte für Byte gleich. Der dritte ging gegen eine Ersatzgegenseite unter `/tmp`.

Für C5.2 ist `GH_CONFIG_DIR` auf ein leeres Verzeichnis gesetzt worden, nicht die
Anmeldung des Nutzers entfernt. Sie steht unverändert.

**Ein Anhang ist heruntergeladen worden, und der Zähler der Releaseseite steht danach auf
2 statt 1.** Das ist die einzige Spur, die diese Prüfung auf der Gegenseite hinterlässt.

---

## Empfehlung für den Abschluss des Circles

**Kohärent.** Alle prüfbaren der fünfzehn halten, und keines hält nicht. Das eine nicht
prüfbare, C2.2, verlangt ein Gerät, das dieses Projekt nicht hat; sein Mechanismus ist
geprüft, seine Abnahme steht aus.

Wer den Marker `_b_` vorzieht, hat mit C2.2 einen Grund dafür, und er ist ein anderer als
der übliche Grund dieses Projekts: nicht „der Abnahmelauf ist Nutzerarbeit", sondern „das
Gerät fehlt". Die Wahl liegt beim Nutzer. Die Zahlen für die Schließungsnotiz stehen oben:
15 geprüft, 14 haltend, 1 nicht prüfbar.
