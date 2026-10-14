Eine entpackte Verknuepfung bekommt ihr Aenderungsdatum nicht

---

Der Rundweg durch Zip und Unzip erhaelt seit dem 260825 das Aenderungsdatum jeder Datei und
jedes Ordners, aber nicht das einer symbolischen Verknuepfung. Der Archiveintrag traegt es
richtig; das Auspacken legt es nur nicht an.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was der Baum traegt

`operation::entpacken::zeit_setzen` geht ueber `File::open(pfad)?.set_times(zeiten)`, und
`File::open` **folgt** einer Verknuepfung. Ein Aufruf an einer Verknuepfung schriebe das Datum
also auf ihr Ziel, und das waere schlimmer als es liegen zu lassen: aus einem fehlenden Wert
wuerde ein falscher an einer Datei, ueber die niemand etwas gesagt hat. Der Zweig
`eintrag.is_symlink()` in `eintraege_entpacken` laesst das Datum deshalb bewusst aus, mit dem
Grund im Modulkopf.

Gemessen am 260825-1855 an einem von KRK gepackten Archiv mit `/usr/bin/unzip` und
`/usr/bin/ditto -x -k`: beide legen die Verknuepfung mit der Uhrzeit des Entpackens ab, obwohl
der Eintrag im Archiv das Quelldatum fuehrt. Die zwei fremden Werkzeuge tun hier also dasselbe
wie KRK.

## Warum das traegt

Der Wert steht im Archiv und kommt nicht an. Das ist ein kleinerer Verlust als der des Defekts
`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_*_jeder-gepackte-eintrag-traegt-den-1-januar-1980-*`,
den diese Runde behoben hat, aber es ist derselbe Verlust: eine Angabe, nach der der Nutzer in
der Dateiliste sortiert.

## Was fehlt

Die Zeit am Verweis selbst setzt allein `lutimes(2)`, hilfsweise `utimensat(2)` mit
`AT_SYMLINK_NOFOLLOW`. Die Standardbibliothek bietet keinen Weg dorthin: `FileTimes` und
`File::set_times` folgen dem Verweis, und `std::os::unix` kennt keine Ausnahme davon. Es waere
damit die **siebte** Schnittstelle der Systemschicht in `krk-core/src/verzeichnis/sys.rs`, und
ob sie diesen einen Wert wert ist, ist eine Frage und keine Zeile.

`operation::kopieren` hat dieselbe Luecke an derselben Stelle: `verknuepfung` legt den Verweis
ueber `std::os::unix::fs::symlink` an und setzt keine Zeit. Wer die Schnittstelle bindet,
schliesst beide Stellen und nicht eine.

**Schwere:** gering. Kein Datenverlust am Inhalt und keine falsche Angabe, sondern eine
fehlende, und zwei verbreitete Werkzeuge des Systems machen es genauso.

**Gefunden:** coder, bei der Umsetzung von Schritt 3, Strang 1 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`. Das Abnahmekriterium
jenes Schritts nennt die Verknuepfung im Rundweg mit; erfuellt ist es fuer Datei und Ordner und
fuer die Verknuepfung nicht.

**Betroffen:** `crates/krk-core/src/operation/entpacken.rs` (`zeit_setzen`, der
`is_symlink`-Zweig in `eintraege_entpacken`), `crates/krk-core/src/operation/kopieren.rs`
(`verknuepfung`), `crates/krk-core/src/verzeichnis/sys.rs` fuer die Bindung

**Domain:** code
