`README.md` reicht verwaiste Tags mit `git push origin --tags` öffentlich nach, der Marke, die die Aufsicht des Werkzeugs verbietet
---
Ein gescheiterter Lauf lässt Eintrag und Tag lokal stehen. Wählt der Nutzer danach eine andere Zahl, bleibt der alte Tag verwaist liegen, und der Handgriff der README schiebt ihn mit jedem anderen zur Gegenseite.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `README.md`, `xtask/src/version.rs`, `xtask/src/git.rs`

## Befund

**Der Tag entsteht vor dem Bau.** `cargo xtask version` setzt ihn als letzten Schritt (`version.rs:224`, `:231-257`); `release` liest ihn nur (`release.rs:279-281`). Scheitert eine Station, bleiben Eintrag und Tag stehen — gewollt, `version.rs:53-63`.

**Er blockiert nicht, er bleibt liegen.** Ein zweiter Lauf mit derselben Zahl fällt in `NichtsZuTun` (`version.rs:148-154`). Ein Lauf mit einer **anderen** Zahl fragt allein `Tagliste(v<neu>)` (`:134`) und sieht den alten nicht; `Schub` schiebt je Lauf genau einen Tag (`git.rs:345-348`, `veroeffentlichung.rs:517-522`). Der alte Tag benennt danach einen lokal eingetragenen, nie ausgelieferten Stand.

**Der Handgriff.** `README.md:369-381` („Einmal vor dem ersten Lauf: die alten Tags nachschieben") empfiehlt `git push origin --tags`. Er ist für die Tags der Runden vor Station 8 geschrieben, schiebt aber jeden lokalen Tag, auch den verwaisten. Die `comm`-Zeile davor zeigt ihn an, sagt aber nicht, dass ein angezeigter Tag zu prüfen ist. Dieselbe Marke steht in `git.rs:466-477` (`MARKEN`) mit dem Doc-Kommentar „erweitert die Reichweite" und käme aus dem Werkzeug nie hinaus; die README empfiehlt sie von Hand.

**Stand am 260826:** `git tag -l` und `git ls-remote --tags origin` tragen dieselben 18 Namen; ein verwaister Tag liegt heute nicht vor.

## Abhilfe

Den Handgriff je Tag schreiben — `git push origin refs/tags/v<zahl>` für jeden Namen, den `comm` ausgibt — und den verwaisten Tag als Fall benennen: „ein Tag, zu dem keine Releaseseite gehört, wird nicht nachgeschoben, sondern gelöscht". Ob `version` einen verwaisten Vorgänger selbst erkennt, ist eine Entscheidung und nicht abzuleiten.

**Schwere:** Medium — eine Handanweisung, die eine Wirkung über das Gerät hinaus hat und die das Werkzeug sich selbst untersagt.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, M4
