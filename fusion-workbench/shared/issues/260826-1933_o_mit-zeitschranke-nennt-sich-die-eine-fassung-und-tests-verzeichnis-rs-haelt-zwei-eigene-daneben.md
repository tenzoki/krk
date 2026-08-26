`mit_zeitschranke` nennt sich „die eine Fassung“, und `tests/verzeichnis.rs` haelt zwei eigene daneben

---

Der Commit `9c02863` zieht `mit_zeitschranke` nach `tests/gemeinsam/mod.rs` und laesst den Doc-Kommentar sagen, sie sei die eine Fassung fuer alle Huellen um dieselbe Tuer. Die Datei, die den vierten Rufer traegt, `tests/verzeichnis.rs`, haelt zwei Fassungen derselben Bauart weiter selbst: `inhalt_mit_zeitschranke` und die Handform in `eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an`. Die zweite begruendet ihre Sonderform mit dem Fehlen einer gemeinsamen Fassung, die es seit diesem Commit gibt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/tests/gemeinsam/mod.rs:249-277`; `crates/krk-core/tests/verzeichnis.rs:1706-1729`, `:3509-3525`
**Tree state:** `9c02863`
**Domain:** code

## Was dasteht

`tests/gemeinsam/mod.rs:252`: „**Die eine Fassung fuer alle Huellen um dieselbe Tuer.**“

`tests/verzeichnis.rs:1714-1729` (`inhalt_mit_zeitschranke`): Kanal, Faden, `recv_timeout`, Panik mit Namen; Zeile fuer Zeile die Bauart von `mit_zeitschranke`, mit vorgebundenem Aufruf `traegt_der_inhalt`.

`tests/verzeichnis.rs:3509-3512`: „sie ist dieselbe Bauart wie `oeffnen_mit_zeitschranke` in `tests/text.rs`, und eine gemeinsame Fassung gaebe es nur um den Preis, den Pruefling durch die Hilfsfunktion zu reichen.“ Genau das tut die neue Probe `eine_benannte_roehre_ohne_schreiber_haelt_den_schwungleser_nicht_an` (`:3547-3549`) fuenfzehn Zeilen darunter mit `move || Schwungleser::oeffnen(&roehre).map(|_| ())`, und der Preis ist eine Zeile.

Der Sitzungseintrag `shared/history/260826-1930-coder-der-schwungleser-nimmt-die-huelle-ohne-warten-oeffnen.md` nennt beide Stellen unter „Nicht angefasst“ und verweist auf den Plan, der allein den vierten Rufer nennt. Der Befund richtet sich deshalb nicht gegen die Behebung, sondern gegen den Zustand, den sie hinterlaesst: ein Doc-Kommentar, der „die eine Fassung“ sagt, neben zwei Fassungen in derselben Kiste, und ein zweiter, dessen Begruendung nicht mehr traegt.

## Was zu tun waere

Beide Stellen in `tests/verzeichnis.rs` auf `gemeinsam::mit_zeitschranke` ziehen; `inhalt_mit_zeitschranke` bleibt als duenne Huelle um den vorgebundenen Aufruf oder faellt. Den Satz an `:3509-3512` streichen. Die Fassung in `crates/krk-ui/src/vorschaumodell.rs:1077` liegt in einer anderen Kiste und ist von `tests/gemeinsam` aus nicht erreichbar; sie bleibt.

## Was geprueft ist

Gelesen am Baum `9c02863`; `git grep -n 'recv_timeout' 9c02863 -- crates` liefert die Stellen.
