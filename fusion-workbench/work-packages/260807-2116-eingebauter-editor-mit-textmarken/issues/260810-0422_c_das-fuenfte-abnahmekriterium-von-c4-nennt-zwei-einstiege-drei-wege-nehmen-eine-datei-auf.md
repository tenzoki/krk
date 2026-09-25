# Das fünfte Abnahmekriterium von C4 nennt zwei Einstiege, drei Wege nehmen eine Datei auf

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md:288`
**Cross-references:** Spec C2 Abnahmekriterium bei Zeile 241, `crates/krk-ui/src/editormodell.rs` (`einziehen`), `crates/krk-ui/src/appkit/anwendung.rs` (`im_editor_oeffnen`, `editor_aus_vorschau`, `textmarke_anspringen`)

---

## Der Befund

Das fünfte Abnahmekriterium von C4 lautet:

> Dieselbe Nachfrage erscheint, wenn der Editor über einen der **beiden
> Einstiege aus C2** eine andere Datei aufnehmen soll.

Drei Wege nehmen eine andere Datei auf, und alle drei rufen
`Editorbereich::datei_oeffnen`:

```
  F4                 im_editor_oeffnen        anwendung.rs:3298
  cmd+e (Vorschau)   editor_aus_vorschau      anwendung.rs:3345
  Textmarke (C6)     textmarke_anspringen     anwendung.rs:1105
```

C2 sagt das an seiner eigenen Stelle bereits richtig: das Kriterium bei Zeile
241 nennt neben den beiden Einstiegen ausdrücklich „dasselbe gilt für den
Sprung auf eine Textmarke aus C6". Der Doc-Kommentar von `Editormodell::einziehen`
sagt es ebenso: „F4, der Uebergang aus der Vorschau und der Sprung auf eine
Textmarke aus C6 erben die Regel". Allein C4 zählt zwei.

## Wirkung

Keine im Code: gefragt wird an allen drei Wegen, weil die Regel im Modell
sitzt und nicht bei den Einstiegen. Der Befund ist ein Zählfehler im Spec, der
eine Abnahme in die Irre führt — wer das fünfte Kriterium abhakt, prüft zwei
von drei Wegen.

## Was zu tun ist

Das Kriterium auf „über einen der Wege, die eine Datei in den Editor bringen"
umstellen oder den Sprung aus C6 namentlich dazunehmen, so wie C2 es an seiner
Stelle tut. Der Befund ist älter als diese Sitzung und fällt hier auf, weil
D5 die Aufzählung der Anlässe ohnehin durchgezählt hat.

---
Resolved: Das fünfte Abnahmekriterium von C4 nennt seit dem 260810-0714 alle drei
Wege. Es lautet jetzt: "Dieselbe Nachfrage erscheint, wenn der Editor eine andere
Datei aufnehmen soll, gleich auf welchem der drei Wege sie kommt: über F4, über
den Übergang aus der Vorschau oder über den Sprung auf eine Textmarke aus C6."
Der Zusatz "und der Abnahmelauf prüft alle drei" steht dabei, weil genau daran der
Befund hing.

Der Vermerk bei S28, der das Kriterium mit seinem alten Wortlaut zitierte, ist
mitgezogen und hält den Befund fest: der Bau war nie enger als die drei Wege,
weil die Regel im Editormodell sitzt und nicht bei den Einstiegen.

Am Code ist nichts geändert; der Befund war ein Zählfehler im Spec.
