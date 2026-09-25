# Coder: README umbauen — erst der Nutzer, dann der Entwickler

**Datum:** 2026-08-21 20:44
**Status:** Complete
**Agent:** coder
**Baumstand:** `71a9920`

## Auftrag

Die `README.md` umbauen und kürzen. Zwei Festlegungen des Nutzers bestimmten den Umbau:
die Datei hat seit der Auslieferung von v0.5.6 einen zweiten Leser, der das Zip von der
Releaseseite lädt, und der soll seine Antwort in den ersten dreißig Zeilen finden; die
Signierung wird auf das Nötige gekürzt und bleibt in der Datei. Ausdrücklich verlangt
waren daneben zwei auffindbare Gegenstände: wie man ein Release macht, und Download und
Installation.

## Was entstanden ist

Die Datei ist von **639 auf 407 Zeilen** gekürzt und neu geordnet. Die Reihenfolge lautet
jetzt: Kopf, Herunterladen und installieren, Voraussetzungen, Bauen, Signierung, Ein
Release machen, Versionspflege.

| Abschnitt | vorher | nachher |
|---|---|---|
| Kopf | 10 | 12 |
| Herunterladen und installieren | 39 | 35 |
| Voraussetzungen | 23 | 19 |
| Bauen (mit „Das Bündel bauen") | 53 | 50 |
| Signierung (mit den drei Unterabschnitten) | 134 | 53 |
| Auslieferung → „Ein Release machen" | 295 | 182 |
| Versionspflege (mit „Versionsstufen") | 85 | 56 |

Der Nutzerabschnitt steht ab Zeile 13, die Releaseseite ab Zeile 16, die drei
Installationsschritte ab Zeile 18.

## Die Kürzungsregel und ihre Anwendung

Der Auftrag gab die Regel vor: eine Begründung bleibt, wenn ihr Fehlen jemanden eine
Sitzung kostet, und fällt, wenn sie nur erklärt, was ohnehin geschieht. Was danach
gefallen ist und keine bloße Wiederholung war:

- **Die Herleitung, warum Stufe 2 der Identitätssuche ohne `-v` fragt und Stufe 3 mit**
  (13 Zeilen) samt dem Absatz zur Vertrauenseinstellung. Der Unterschied steht jetzt als
  ein Halbsatz an den zwei Abfragen. Wer ihn genauer braucht, findet ihn im Modulkopf von
  `xtask/src/sign.rs`.
- **Der Kommandozeilenweg zur Entwicklungsidentität** (25 Zeilen: zwei `openssl`-Aufrufe,
  `security import`, die Begründung der drei Algorithmus-Angaben gegen
  `MAC verification failed`, das `rm`). Der Weg über die Schlüsselbundverwaltung führt
  zum selben Ergebnis in vier Zeilen und braucht ebenfalls kein Xcode. Aus dem gefallenen
  Block ist die eine Falle mitgenommen, die auch den GUI-Weg trifft: der
  Zertifikatsassistent schlägt einen anderen Zertifikatstyp als `Codesignatur` vor.
- **Die Herleitung der abgelaufenen Zertifikatskette** (G3-Instanz, `issuer=`-Zeile, Apple
  Root CA, `TeamIdentifier`, das Nachprüfen mit `codesign -dvv`). Geblieben sind das
  Symptom, der Satz, dass die Meldung in die falsche Richtung deutet, die zwei Kommandos
  und die Warnung, das alte Zwischenzertifikat nicht mit erhöhten Rechten zu entfernen.
- **Der Absatz „`release.sh` ist kein drittes Bauwerkzeug"**. Das Schichtenbild darüber
  sagt dasselbe.
- **Die Vorläufe a, b und c** als eigene Aufzählungsglieder im Stationenweg.
- **Die Versionsgeschichte des Taggens**: wer bis zum 260813-1534 taggte, warum der Nutzer
  die Festlegung zurücknahm, wofür `v0.1.0` steht und warum es keine rückwirkenden Tags
  gibt (17 Zeilen). Geblieben ist der Satz, dass das Werkzeug den Tag setzt, mit dem
  Verweis auf den Entscheidungsdatensatz.
- **Die Zahl `1.97.1` in der Voraussetzungstabelle.** Sie steht in
  `rust-toolchain.toml`, und die Tabelle nennt jetzt nur noch die Datei — kein
  Zahlenstand in Prosa, der altert.

Als Wiederholung gefallen sind die Überschneidungen zwischen `## Auslieferung`,
`### Das Paket bauen`, `### Nur beglaubigen` und `### Nur veröffentlichen`: die
Voraussetzungsprosa stand dreifach (Kopf der Datei, Fließtext der Auslieferung, Nachsatz
zu den Stationen) und steht jetzt einmal; die acht Stationen standen als
neunzig Zeilen Fließtext und stehen jetzt als Tabelle mit drei nachgestellten Absätzen für
das, was an ihnen nicht offensichtlich ist.

## Was nicht gekürzt wurde

**Die Betriebsregel gegen den Datenverlust.** Sie ist gestrafft und inhaltlich
unangetastet: Überkopieren statt Löschen, der Ordner
`~/Library/Application Support/KRK/`, die vier Dinge, die darin liegen, der App Deleter von
ForkLift als Beispiel, und der Ausweg für den, der doch löschen will. Der Herkunftsnachweis
auf die Untersuchung `shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`
steht weiter dabei, mit dem Satz, dass die Regel auch ohne diese Datei vollständig ist.
Gefallen ist allein die Beweisführung am Gerät (Neuanlage des Ablageordners am 17.08. um
19:13:48, vier unangetastete Auslieferungen, `bookmarks.toml` nach dem 20.08.): sie belegt
die Regel, ändert aber nichts an dem, was zu tun ist, und steht vollständig in der
Untersuchung.

## Abgleich mit dem Releasetext

Der Kopf der neuen `README.md` und `RELEASETEXT` in `xtask/src/veroeffentlichung.rs` sind
Aussage für Aussage gegeneinander gelesen. Sie decken sich jetzt in allen zehn Aussagen,
die die Probe `der_releasetext_traegt_jede_seiner_aussagen` einzeln behauptet. Vorher taten
sie es nicht: zwei Aussagen fehlten der README (macOS 15 als Voraussetzung für den Nutzer,
und dass das Bündel beglaubigt ist und ohne Rückfrage startet), und eine dritte stand ihr
entgegen. Beides ist im Defektdatensatz
`issues/260821-2044_c_der-readme-kopf-sagte-der-editor-folge-noch-in-spaeteren-runden.md`
festgehalten und mit diesem Umbau behoben.

## Was die Proben binden

- `der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` (`xtask/src/release.rs`): die
  neue Fassung schreibt die alte Stationszahl nirgends; der Weg heißt durchgehend
  achtstationig, und der Abschnitt trägt die Zahl in seiner Überschrift.
- Die zwei Abschnittsnamen, auf die Abbruchmeldungen aus `xtask/src/sign.rs` zeigen —
  `Entwicklungsidentität anlegen` und `Abgelaufene Zertifikatskette` — stehen unverändert
  als Überschriften, ebenso `Versionsstufen`, auf das `xtask/src/version.rs` zeigt. Keine
  Meldung war mitzuziehen, und an `xtask/` ist nichts geändert.

## Abnahme

`make check` — Exit 0. 155 Proben in `xtask` grün, `fmt --check` und
`clippy -D warnings` grün.
