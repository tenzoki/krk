# KRKs fusion-Leseprofile auf fusion 11 nachziehen

---
**Status:** claimed
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260912-2043
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

KRKs Vorschau zeigt für eine erkannte fusion-Werkbank eine Profil-Zusammenfassung statt der
Metadaten. Die Profile in `resources/default-readers.toml` beschreiben die Werkbank in der Form,
die fusion bis Fassung 10 angelegt hat; fusion 11 hat die Form geändert, und seitdem greifen neun
Stellen ins Leere oder zählen etwas, das es nicht mehr gibt. Erledigt ist die Arbeit, wenn jedes
Profil, das eine fusion-Werkbank beschreibt, an einer Werkbank der Fassung 11 dieselbe Auskunft
liefert wie früher an einer der Fassung 10, und wenn keine Zeile mehr eine Zahl zeigt, die aus
einer abgeschafften Form stammt.

## Die eine Ursache hinter fast allem

Eine Arbeitseinheit trug ihren Zustand früher als Buchstaben im Dateinamen ihres Datensatzes
(`_t_circle.md` für laufend, `_b_` für beschränkt geschlossen, und vier weitere). Heute heißt der
Datensatz wie sein Verzeichnis, ohne Buchstaben, und der Zustand steht als Kopfzeile `**Status:**`
darin, mit vier Werten statt sechs. Alles, was Dateinamen zählt, zählt damit ins Leere; die
Auskunft ist nicht verloren, sondern nur noch durch Öffnen der Datei zu holen.

## Der Angelpunkt

`default-readers.toml:739` erkennt ein Rundenverzeichnis daran, dass ein Eintrag `_X_circle.md`
heißt. Ersetzt durch `pfad = 'fusion-workbench/circles/[^/]+$'` trifft das Profil beide Formen,
alte wie neue. Ohne diesen Schritt sind vier weitere Befunde nicht sinnvoll zu beheben, weil die
Auskunft, die sie verloren haben, nur je Container erreichbar ist.

## Was daran hängt, in Abhängigkeitsreihenfolge

1. Ortsangabe des Rundenprofils umstellen (Zeile 739), Kommentar 728-729 mitziehen.
2. Im selben Profil die vier Zustandszeilen (743-755) durch eine Zeile ersetzen, die `**Status:**`
   aus dem Datensatz liest. Sie liest an alten Datensätzen das alte Vokabular und an neuen das
   neue, was für eine gemischte Werkbank die ehrlichere Auskunft ist. Das Directive-Muster in 759
   braucht nur ein anderes `datei`-Muster.
3. Die sechs Zustandszeilen im Profil „alle Runden" (554-574) fallen; mit Dateinamen ist die
   Auskunft nicht mehr zu haben, und der Kommentar 535-543, der ihre Summe als Prüfung begründet,
   ist damit überholt.
4. Zwei tote Zeilen streichen, je zweimal, weil sie doppelt stehen: „Aktive Runde" (389, 819) liest
   `.active-circle`, das fusion inzwischen aktiv löscht; „Sitzung" (393, 823) liest
   `orchestrator-live.md`, das abgeschafft ist. Für beide gibt es mit den vier Bausteinen keinen
   Nachfolger.
5. `backlog` gegen `checkouts` tauschen (Aufzählung 426, Zeilenpaar 653-659). `shared/backlog` ist
   kein Speicher mehr, `shared/checkouts` ist neu und führt eine Zeile je Arbeitskopie. Der
   Leselauf-Haushalt des Profils steht bei zehn von zwölf und bleibt bei zehn.
6. Optional: eine Zeile für das `checks`-Objekt in `.fusion-setup`, neu seit 11.0.1, zehn
   Installationsprüfungen mit Datum. Kostet eine der vierundzwanzig Öffnungen, Platz ist da.

## Eine Nutzerfrage steckt darin und ist nicht abzuleiten

Die Zeilen für Verläufe und Untersuchungen (673-687, 773-775) zählen Speicher, die fusion für
schreibgesperrt erklärt hat — in dieser Werkbank aber schreibst du weiter hinein. Wer sie
streicht, verliert eine Auskunft, die dieses Projekt pflegt; wer sie behält, hat auf einer
regelkonformen Werkbank eine Zeile, die wie ein Puls aussieht und keiner ist.

## Nicht anfassen

Die Speicherpfade, die Marker der Defekte und der Entscheidungen, das Ablageprofil, `setup_at` und
`plugin_version` sind unverändert richtig. Das `stilwerk`-Verzeichnis bekommt bewusst keine Zeile,
aus demselben Grund wie beim flight-Profil.

---

## Stand 260913-0900 — die Profile sind nachgezogen, die Arbeit ist es nicht

**Gelandet in `4815f0a`:** alle sechs Schritte der Liste oben, bis auf den als freiwillig
ausgewiesenen sechsten. Die Ortsangabe des Rundenprofils hängt am Pfad und trifft beide
Formen; der Zustand steht als eine Feldzeile statt vier Ja/Nein-Zeilen; die sechs
Zustandszeilen in „alle Runden" sind ohne Ersatz gefallen; „Aktive Runde" und „Sitzung" sind
in beiden Wurzelprofilen weg; `backlog` ist gegen `checkouts` getauscht. Die Nutzerfrage zu
Verläufen und Untersuchungen ist am 260912 entschieden: Zählzeilen bleiben, Datumszeilen
gehen. Die Proben sind mitgezogen (`260912-2154_*`, geschlossen).

**Nicht gebaut, mit Begründung:** die Zeile für das `checks`-Objekt in `.fusion-setup`. Ein
Feldbaustein fängt genau eine Fanggruppe, also ein Datum, und welches der zehn es ist,
entscheidet die Reihenfolge im JSON und nicht der Kalender. Eine Zeile „Prüfungen, zuletzt",
die das erste statt das jüngste zeigt, ist die irreführende Sorte, die diese Datei anderswo
schon abgelehnt hat.

**Offen, und deshalb bleibt dieses Arbeitspaket beansprucht:** die Auslieferungsfassung bricht
das Abnahmekriterium C6.7 der Runde 16 um genau eine Dateiöffnung, zwölf statt elf, weil
„Zustand" und „Directive" dieselbe Datei nennen und sie zweimal öffnen. Der Nutzer hat am
260913 entschieden, die Ursache zu beheben statt die Zahl:
`260913-0851_*_merkt-sich-der-feldbaustein-seine-dateioeffnungen-wie-der-leselauf-seine-lesungen.md`.
Bis dahin geht keine Auslieferung hinaus.

**Daneben offen, aus dieser Arbeit gefunden:**
`260912-2158_*_zwei-rundendatensaetze-tragen-eine-status-kopfzeile-die-ihrem-eigenen-dateinamen-widerspricht.md`
— zwei Datensätze der Werkbank widersprechen sich selbst, und KRKs Vorschau zeigt dort jetzt
den falschen Zustand.
