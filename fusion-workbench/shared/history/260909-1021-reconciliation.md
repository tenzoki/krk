# Abgleich 260909-1021 — die Zustandsregel greift auf den Bestand durch

**Filed by:** reconciler, Domäne `code`
**Baumstand:** `59e9910`, Sitzungsbeginn `28c4a47`, 37 Commits dazwischen
**Auftrag:** die Antwort des Nutzers zu
`shared/decisions/260907-2340_*_wie-weit-reicht-die-neue-regel-fuer-den-zustand-eines-anforderungsdokuments-in-den-bestand-zurueck.md`
(Möglichkeit 3) an vierzehn benannten Anforderungsdokumenten und Plänen ausführen
**Vorlage:** `shared/planning/260825-1725_c_plan-vorschau-vertieft-und-zwei-fehler.md` vom
260908-1539, Kopf und Nachsatz

## Was gefahren ist

Vierzehn Dateien geprüft, vierzehn geändert. Je Datei drei Eingriffe: die Kopfzeile
`**Status:**` neu gefasst auf den Stand der **Bauarbeit**, eine neue Kopfzeile `**Abnahme:**`
direkt darunter, und am Dateiende ein Abschnitt `## Nachsatz vom 260909-1021: Zustand und
Abnahme sind getrennt`, der den ursprünglichen Wortlaut der ersetzten Kopfzeile als Blockzitat
festhält. Danach die Umbenennung des Markers im Dateinamen.

**Am Sachtext oberhalb der Kopfzeilen ist an keiner der vierzehn Dateien etwas geändert.** Keine
Schrittmarke ist bewegt, kein Abnahmekästchen angehakt, kein Kriterium umformuliert. Die vier
Dateien, die schon einen `## Nachsatz vom 260906-0448` tragen, behalten ihn unangetastet; der
neue steht darunter.

**Kein Code und keine strukturierten Daten sind angefasst.** `git status` weist außerhalb von
`fusion-workbench/` keine Änderung aus.

## Die vierzehn Dateien, je mit Marker, Abnahmestand und Beleg

| # | Datei (neuer Name) | Marker | Abnahme | Woran die Bauarbeit belegt ist |
|---|---|---|---|---|
| 1 | `circles/260807-2116-…/planning/260807-2147_c_spec-eingebauter-editor-mit-textmarken.md` | `_o_`→`_c_` | offen | Runde 2 beschränkt geschlossen, Plan `260808-0140_c_`; `krk-core/src/text/` mit fünf Modulen, `editormodell.rs`, `hervorhebung.rs`, `appkit/nummernspalte.rs`, `fenstertitel.rs` am 260909-1021 nachgesehen |
| 2 | `circles/260809-2040-…/planning/260811-0753_c_spec-tastenbelegung-als-markdown-in-downloads.md` | `_o_`→`_c_` | offen | Runde 3 beschränkt geschlossen, Plan `260811-0838_c_`; `belegungsausgabe::markdown` und der Selektor `tastenbelegungSichern:` in `appkit/menue.rs` |
| 3 | `circles/260809-2040-…/planning/260811-1130_c_abnahmeanleitung-tastenbelegung-als-markdown.md` | `_o_`→`_c_` | offen | die Anleitung selbst ist vollständig geschrieben, acht Blöcke über die 41 Kriterien des Specs |
| 4 | `circles/260811-1257-…/planning/260811-1552_c_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` | `_o_`→`_c_` | offen | Runde 4 beschränkt geschlossen, Plan `260811-1648_c_`; Abgleich 260811-2157 mit 23 von 62 Kriterien einzeln am Baum belegt; `Kommando::MitStandardprogrammOeffnen`, `Kommando::TabSchliessen`, `pfadzeilen` |
| 5 | `circles/260813-2332-…/planning/260813-2348_c_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` | `_o_`→`_c_` | **teilweise gefahren** 260814-1115, 8 von 29 Bündelkriterien gedeckt | Runde 9 beschränkt geschlossen, Plan `260814-0656_c_`; Abgleich 260814-1247 mit 46 Kriterien am Baum; `appkit/blaetter/zettel.rs` |
| 6 | `circles/260814-1551-…/planning/260814-1830_c_spec-tippen-filtert-dateiliste-flach-und-tief.md` | `_o_`→`_c_` | offen | Runde 10 beschränkt geschlossen, Plan `260814-2102_c_`; Abgleich 260815-1216 zu C1.9 und C1.10; `verzeichnis/filter.rs`, `verzeichnis/durchlauf.rs` |
| 7 | `circles/260823-2208-…/planning/260824-0613_c_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` | `_o_`→`_c_` | offen | Runde 16 beschränkt geschlossen; Abgleich 260824-1852: 38 von 56 Kriterien ohne Fenster belegt; `krk-core/src/leseprofil/` mit fünf Modulen |
| 8 | `circles/260823-2208-…/planning/260824-0640_c_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` | `_p_`→`_c_` | offen | alle vierzehn Schritte `[DONE]`, am 260824-1852 einzeln gegen den Baum gelesen, je mit Commit und Fundstelle; `make check` grün, 1520 Proben in 22 Zielen |
| 9 | `circles/260830-1045-…/planning/260830-1251_c_spec-git-bereich-liest-status-branch-verlauf.md` | `_p_`→`_c_` | offen | Runde 23 beschränkt geschlossen; 65 von 90 Kriterien mit Stelle im Baum oder Probe belegt; `krk-core/src/git/` mit vier Modulen, `appkit/git.rs` |
| 10 | `circles/260830-1045-…/planning/260830-1317_c_plan-git-bereich-liest-status-branch-verlauf.md` | `_p_`→`_c_` | offen | sechzehn Bauschritte `[DONE]`, am 260831-1417 einzeln belegt; Schritt 17 ist der Abnahmelauf und trägt keine Bauarbeit |
| 11 | `shared/planning/260813-0053_c_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` | `_o_`→`_c_` | offen | Runde 7 beschränkt geschlossen, Plan `260813-0205_c_`; `Kommando::WeitereInstanz`, `kommandos/zulaessigkeit.rs` |
| 12 | `shared/planning/260816-1310_c_spec-inhaltsfilter-der-dateiliste.md` | `_o_`→`_c_` | offen | Runde 11 beschränkt geschlossen, Plan `260816-1359_c_`; Abgleich 260820-2056, alle ohne Bündel entscheidbaren Kriterien halten; `verzeichnis/inhalt.rs` |
| 13 | `shared/planning/260819-2216_c_spec-auswahl-und-kopieren-in-der-vorschau.md` | `_p_`→`_c_` | **gefahren** 260820-1030 an `KRK.app` 0.5.4 | acht von acht Planschritten gebaut und einzeln gelesen, Plan `260819-2245_c_`, Runde 14 kohärent geschlossen; `text_auf_ablage_schreiben` in `appkit/zwischenablage.rs` |
| 14 | `shared/planning/260821-1115_c_spec-artefakt-und-release.md` | `_o_`→`_c_` | **gefahren** 260821-2105, ein Kriterium mit Indiz | elf von elf Planschritten am 260821-1532 belegt, vier Durchsichten gefahren; `xtask/src/veroeffentlichung.rs` mit `RELEASETEXT` an einer Stelle |

Alte und neue Pfade beider Hälften stehen vollständig in `git status --porcelain`; jede Zeile
trägt `RM` und nennt Quelle und Ziel.

## Die zwei Dateien, die stehen bleiben

`circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_o_plan-befehle-absetzen-und-makros-speichern.md`
und `shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md` sind
unangetastet. Die Runde ist zurückgestellt und nie gefahren, es gibt keine belegte Bauarbeit,
und `_o_` ist dort die richtige Auskunft.

## Zwei Abnahmen sind wirklich gefahren, und das ändert die Auskunft der Werkbank

Zwölf der vierzehn Dokumente tragen `**Abnahme:** offen`, und der Grund ist an jedem derselbe:
der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit. **Zwei tragen einen anderen
Stand, und er ist belegt.** Die Runde 14 hat der Nutzer am 260820-1030 an `KRK.app` 0.5.4
abgenommen; vierzehn der fünfzehn Kriterien mit Bündelanteil sind damit gefahren, C2.12 fehlt
und ist am Baum zur Hälfte widerlegt. Die Runde 15 hat er am 260821-2105 abgenommen, und die
Auslieferung von `KRK 0.5.6` über die achte Station ist der Beleg; für C2.2 fehlt ein zweiter Mac
ohne Netz, geprüft ist stattdessen der Mechanismus, und das steht in der Kopfzeile als Indiz und
nicht als Abnahme. Ein dritter Stand steht an der Runde 9: dort ist die Abnahme am 260814-1115
**teilweise** gefahren, mit 8 von 29 gedeckten Bündelkriterien.

## Der Preis, benannt und nicht verschwiegen

Fünf lebende Verweise nennen zwei der umbenannten Dateien mit ausgeschriebenem Marker und zeigen
seit dieser Umbenennung ins Leere. Sie stehen außerhalb der vierzehn Dateien und sind deshalb
nicht nachgezogen; der Nachsatz der jeweiligen Datei nennt sie einzeln.

| Verweisende Stelle | Ziel |
|---|---|
| `circles/260823-2208-…/_b_circle.md:6` (Feld `**Active spec/plan:**`) | Plan der Runde 16 |
| `circles/260816-1321-…/_b_circle.md:7`, `:38`, `:64` | Spec der Runde 11 |
| `circles/260816-1321-…/planning/260816-1359_c_plan-inhaltsfilter-der-dateiliste.md:5` | Spec der Runde 11 |

Der sechste Fund, `messungen/260816-abnahme-inhaltsfilter.md:8`, fällt unter die Ortsregel in
`CLAUDE.md` und behält seinen damaligen Marker. Alle übrigen Zitate der vierzehn Dateien stehen
in eingefrorenen Speichern (`history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`,
`archive/`) und sind dort Aufzeichnung eines Standes. Erhoben am 260909-1021 über den ganzen
Baum.

## Zwei Entscheidungsdatensätze, deren Antwort dieser Zug einlöst

`shared/decisions/260819-1440_a_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`
und `shared/decisions/260907-2340_a_wie-weit-reicht-die-neue-regel-…-zurueck.md` tragen beide
`_a_`. Ihre Antworten stehen mit diesem Zug im Baum. **Der Marker ist trotzdem nicht auf `_i_`
gezogen, und zwar aus zwei Gründen:** der Übergang `_a_`→`_i_` verlangt einen Commit, den er
zitieren kann, und diese Arbeit ist noch nicht committet; und der Auftrag dieses Laufs begrenzt
die Schreibarbeit ausdrücklich auf die vierzehn Dateien. **Der Nachzug gehört in denselben
Commit, der diese Änderung trägt** — wer ihn versäumt, lässt zwei umgesetzte Antworten dauerhaft
als „beantwortet, nicht gebaut" stehen.

## Kein neuer Defekt aus diesem Lauf

Der Befund, der beim Vorlegen der Frage entstanden ist — die Erhebung im Datensatz nennt
vierzehn Dokumente, der Baum trägt zweiundvierzig —, liegt schon als
`shared/issues/260909-0938_*_die-erhebung-im-datensatz-zur-reichweite-nennt-vierzehn-dokumente-der-baum-traegt-zweiundvierzig.md`
vor. Ein zweiter Datensatz derselben Sache entsteht hier nicht.

## Was die Regel noch offen hat, nachgezählt

Der Bestand führt 42 Anforderungsdokumente und Pläne. Nach diesem Lauf stehen 40 auf `_c_` und
zwei auf `_o_`, und die zwei sind die zurückgestellte Runde, die keine Bauarbeit hat. **Die
Markerhälfte der Regel ist damit über den ganzen Bestand durchgezogen.** Die Kopfzeilenhälfte ist
es nicht: von den 40 Dokumenten auf `_c_` tragen 15 eine Zeile `**Abnahme:**` und 25 keine,
gezählt am 260909-1021 mit `grep -L '^\*\*Abnahme:\*\*'` über beide Planungsspeicher. Die 25
sind die Runden, deren Marker schon vor der Regel auf `_c_` stand; ihr Abnahmestand steht heute
nirgends in ihrem Kopf. **Wer die Regel zu Ende führen will, fährt einen zweiten Zug gegen diese
25** — er ist derselbe Eingriff wie dieser hier, nur ohne Umbenennung.
