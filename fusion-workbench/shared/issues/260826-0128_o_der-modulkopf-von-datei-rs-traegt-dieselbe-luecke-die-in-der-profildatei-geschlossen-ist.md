# Der Modulkopf von `datei.rs` trägt dieselbe Lücke, die in der Profildatei geschlossen ist

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/issues/260825-2233_c_der-satz-ueber-kennzeichnen-gilt-nur-fuer-ein-profil-ohne-pfad-daneben.md`; `shared/reviews/260825-2233-ontorev-nachdurchsicht-der-leseprofile-nach-der-behebungsrunde.md` (Befund N1, Beobachtung für `coderev`); `crates/krk-core/src/leseprofil/datei.rs` (Modulkopf, Abschnitt „Wo `deny_unknown_fields` steht und wo nicht"); `resources/default-readers.toml:61-67`

---

## Was ist

Ein verschriebener Schlüssel `kennzeichnen` statt `kennzeichen` nimmt einem
Profil sein Erkennungsmuster. Was dann geschieht, hängt davon ab, ob ein `pfad`
danebensteht:

- **ohne `pfad`:** das Profil nennt weder Pfadmuster noch Kennzeichendatei und
  fällt weg, mit Meldung. Gemessen: 11 Profile statt 12.
- **mit `pfad`:** der Tippfehler wird **still übergangen**, weil `Profilblock`
  als einziger Block kein `deny_unknown_fields` trägt. Das Profil greift weiter
  über den `pfad` allein. Gemessen: 12 Profile, keine Meldung.

Diese Fallunterscheidung ist seit dem 260826 im Kommentarkopf von
`resources/default-readers.toml` (`:61-67`) ausgeschrieben, also dort, wo der
Nutzer sie liest.

**Der Modulkopf von `crates/krk-core/src/leseprofil/datei.rs` sagt an derselben
Stelle weiter nur die erste Hälfte**, im Abschnitt „Wo `deny_unknown_fields`
steht und wo nicht".

## Warum das zählt

Es ist die Stelle, an der ein Entwickler nachliest, warum `Profilblock` als
einziger Block ohne `deny_unknown_fields` steht. Wer dort die halbe Antwort
findet, hält die Reichweite für größer, als sie ist — und das ist genau der
Fehlschluss, den die Behebung in der Profildatei gerade beseitigt hat.

Zwei normative Texte über denselben Mechanismus, von denen einer berichtigt ist
und der andere nicht, laufen ab jetzt auseinander.

## Was zu tun wäre

Einen Halbsatz im Modulkopf von `datei.rs` nachziehen, nach dem Wortlaut, den
`resources/default-readers.toml:61-67` bereits trägt. Kein Verhalten ändert
sich, keine Probe.

Zu prüfen wäre dabei, ob der Modulkopf die zwei Lagen auch **misst** — die
Behebung in der Profildatei hat beide gemessen, und eine Probe, die den stillen
Fall festhält, gibt es bisher nicht.

## Status

Offen. Gefunden vom Ontocoder der Aufgabe S-1 an einer Datei, die für ihn
gesperrt war, und vom Reviewer der Nachdurchsicht als Beobachtung ohne eigenen
Datensatz festgehalten. Hält keine Auslieferung auf.
