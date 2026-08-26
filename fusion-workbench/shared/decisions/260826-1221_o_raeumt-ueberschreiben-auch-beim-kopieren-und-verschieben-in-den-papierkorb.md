# Raeumt „Ueberschreiben“ auch beim Kopieren und Verschieben in den Papierkorb?

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0942_*_ueberschreiben-loescht-beim-packen-endgueltig-und-beim-entpacken-in-den-papierkorb.md` (dort wird die Frage benannt und ausdruecklich offen gelassen); `shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md` (ueberholt); `crates/krk-core/src/operation/mod.rs:419-455`; `crates/krk-core/src/operation/loeschen.rs:91-110`

---

## Question

Seit der Runde 12 gibt es in KRK genau einen Loeschweg, und der fuehrt in den Papierkorb. Am
260825 hat der Nutzer diese Bindung ausdruecklich auf den Packlauf ausgedehnt: `zippen` raeumt
sein vorhandenes Ziel seither ueber die `Papierkorb`-Schnittstelle statt ueber
`loeschen::baum_entfernen`, und die Antwort schloss mit dem Satz, „Ueberschreiben" bedeute danach
im ganzen Kontextmenue dasselbe.

**Fuer das Kopieren und das Verschieben gilt das nicht.** `ziel_klaeren`
(`crates/krk-core/src/operation/mod.rs:431-441`) ruft im Zweig
`Konfliktantwort::Ueberschreiben` weiterhin `loeschen::baum_entfernen(ziel)`, also ein rekursives,
endgueltiges Loeschen ohne Papierkorb. Dieselbe Schaltflaeche desselben Konfliktblattes bedeutet
damit im Kontextmenue „in den Papierkorb" und beim Abwurf oder bei `F5`/`F6` „unwiederbringlich
weg".

Die Frage muss jetzt beantwortet werden, weil der geschlossene Datensatz von 260825 sie selbst
als Nutzerfrage benannt („Ob die Bindung der Runde 12 ueber den Konflikt-Zweig des Kopierens
mitentschieden ist, ist eine Nutzerfrage") und ihre Antwort dann nirgends festgehalten hat.
Solange kein Datensatz sie traegt, faellt sie aus jeder Suche nach aktiver Grundlage heraus, und
der Zustand am Code entscheidet sie stillschweigend.

## Options

1. **Auch Kopieren und Verschieben raeumen in den Papierkorb.** `ziel_klaeren` bekommt die
   `Papierkorb`-Schnittstelle gereicht — sie liegt in `ausfuehren` bereits an und wird bis
   `einen_abarbeiten` durchgereicht — und nimmt sie statt `baum_entfernen`.
   - Pro: „Ueberschreiben" bedeutet danach im ganzen Vorhaben dasselbe. Nichts, was der Nutzer
     mit einem Klick ersetzt, ist unwiederbringlich weg. Das ist dieselbe Antwort, die der Nutzer
     am 260825 fuer den Packlauf schon gegeben hat, und die Leitung dafuer liegt seither.
   - Contra: Das Ueberschreiben wird langsamer und kann an einem Datentraeger ohne Papierkorb
     scheitern, wo `remove_file` glueckte — dann steht der Eintrag mit seinem Grund in der
     Abschlussliste statt uebertragen zu werden. Bei einer Kopie von vielen Dateien in einen
     vollen Zielordner faellt das je Konflikt an.
2. **Der Zustand bleibt, und die Ungleichheit wird als bewusste Wahl festgehalten.** Dann gehoert
   in den Doc-Kommentar von `ziel_klaeren`, warum die Bindung der Runde 12 fuer diesen Zweig
   nicht gilt — heute steht dort nichts dazu, waehrend `zippen.rs` und `entpacken.rs` sie beide
   ausschreiben.
   - Pro: Kein Eingriff, keine neue Fehlerquelle im am haeufigsten gelaufenen Weg der Maschine.
     Ein Ordner auf einem gleichnamigen Ordner verschmilzt vorher (`mod.rs:427-429`), der
     Baumloescher kommt also nur bei ungleichen Typen ueberhaupt dran.
   - Contra: Zwei Bedeutungen fuer eine Schaltflaeche, und die Unterscheidung ist dem Nutzer im
     Blatt nicht anzusehen. Der Fall „Datei ueberschreibt gleichnamigen Ordner" trifft weiterhin
     einen ganzen Baum.
3. **Das Konfliktblatt sagt, was es tut.** Der Zustand bleibt, aber die Schaltflaeche heisst im
   einen Fall „Ersetzen" und im anderen „In den Papierkorb und ersetzen".
   - Pro: Loest den Widerspruch dort, wo der Nutzer ihn sieht, ohne die Maschine anzufassen.
   - Contra: Zwei Bedeutungen bleiben; der Nutzer muss beim Lesen unterscheiden, statt sich auf
     eine Regel verlassen zu koennen. Der Text des Blattes liegt in `krk-ui` und damit weit weg
     von der Stelle, die entscheidet.

## Constraints

- `krk-core` kennt AppKit nicht. Der Papierkorb kommt als Schnittstelle herein
  (`operation/loeschen.rs:39-51`); jede Antwort, die ihn in `ziel_klaeren` braucht, muss ihn
  durch `einen_abarbeiten`, `eintrag_kopieren` und `eintrag_verschieben` durchreichen. Diese
  Leitung existiert bis `einen_abarbeiten` bereits.
- `OhnePapierkorb` (`loeschen.rs:53-68`) scheitert absichtlich, statt still endgueltig zu
  loeschen. Jede Antwort mit Papierkorb muss deshalb den Fehlschlag als uebersprungenen Eintrag
  melden koennen — `zielarchiv_klaeren` (`zippen.rs:283-295`) zeigt die Form.
- `baum_entfernen` hat einen **zweiten** Rufer, `verschieben::ueber_datentraeger`
  (`verschieben.rs:123`). Der ist von dieser Frage nicht beruehrt: dort wird die Quelle
  weggeraeumt, nachdem sie kopiert wurde, und das ist die zweite Haelfte eines Verschiebens und
  kein Ersetzen. (Dass dieser Rufer heute auch dann loescht, wenn das Kopieren gescheitert ist,
  ist ein eigener Defekt:
  `shared/issues/260826-1221_*_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-loescht-die-quelle-trotzdem.md`.)

## Recommendation

Moeglichkeit 1. Sie ist die einzige, die ohne einen zweiten Begriff von „Ueberschreiben"
auskommt, und sie ist dieselbe Wahl, die der Nutzer am 260825 fuer denselben Wortlaut derselben
Schaltflaeche schon getroffen hat. Der genannte Nachteil — ein Datentraeger ohne Papierkorb —
trifft heute schon jeden Unzip- und Zip-Lauf und ist dort hingenommen.
