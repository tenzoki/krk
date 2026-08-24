# Räumung der Spec- und Planbuchführung der Runde 16

**Datum:** 2026-08-24 12:34
**Agent:** analyst
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Auftrag:** Sechs Defektdatensätze abarbeiten, die sämtlich die Buchführung an Spec und Plan betreffen. Ein Sicherungsschalter hatte angeschlagen: vierzehn offene Befunde gegen sieben erledigte Schritte, und der Nutzer hat die Räumung gewählt. Ein `coder` arbeitete parallel an den Code-Befunden.

---

## Ergebnis

**Alle sechs Datensätze sind geschlossen.** Vier Abnahmekriterien des freigegebenen Specs sind dabei inhaltlich berichtigt, ein fünftes hat einen Satz nachbekommen; die vier inhaltlichen Änderungen gehen dem Nutzer vor. Der Plan trägt zwei Nachträge und eine Regel, die in ihm bisher nirgends stand. Keine Datei unter `crates/` ist angefasst.

| Befund | Zustand | Was daran geändert ist |
|---|---|---|
| `260824-1124_*_zwei-feldmuster-…-koennen-nie-treffen` | geschlossen | C3.8 und C3.9 des Specs, Schritt 7 des Plans |
| `260824-1124_*_c4-3-sagt-eine-zeile-…` | geschlossen | C4.3 des Specs |
| `260824-1014_*_c3-14-nennt-bis-zur-grenze-lesen-…` | geschlossen | C3.14 und die Constraints des Specs, zwei Entscheidungsdatensätze |
| `260824-1042_*_schritt-3-zaehlt-vier-abweisungen-…` | geschlossen | Schritt 3 des Plans |
| `260824-0955_*_die-files-zeile-eines-planschritts-…` | geschlossen | Schritt 8 des Plans, `## Testing Strategy` |
| `260824-0634_*_c6-1-sagt-der-feldbaustein-…` | geschlossen | C6.1 des Specs, `## Open Questions` des Plans |

---

## Der folgenreichste Befund, und wie er nachgeprüft wurde

Zwei der sechs Feldmuster, die Schritt 7 für `resources/default-readers.toml` vorschreibt, konnten nie treffen. Der Grund ist bei beiden derselbe: die Kiste `regex` verankert `^` und `$` ohne die Angabe `m` an Anfang und Ende der ganzen Eingabe und nicht an denen einer Zeile. Der Feldbaustein ist der einzige der vier, der gegen einen Dateiinhalt läuft, und nur seine Ausdrücke sind davon betroffen.

Der Auftrag verlangte, die Messung des `coder` nicht zu übernehmen, sondern an einem eigenen Beleg nachzuprüfen. Wir haben dafür ein Wegwerfprogramm gegen `regex` 1.13.1 gebaut, außerhalb des Baumes und im Zwischenablageverzeichnis dieser Sitzung, und es gegen die **echten** Dateien dieser Werkbank laufen lassen statt gegen ausgedachte Zeichenketten. Der Unterschied ist nicht akademisch: der Befund hält ausdrücklich fest, dass eine Probe, die ihre Eingabe im Quelltext ohne abschließendes Zeilenende schreibt, das falsche Muster bestätigt.

```text
.active-circle, echte Datei:
  ^(.+)$        ->  kein Treffer
  ^([^\n]+)     ->  "260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten"
  (?m)^(.+)$    ->  derselbe Name

alle achtzehn Circle-Datensätze, echte Dateien:
  (?s)^## Directive\s*\n+(.+?)\n\n   ->  trifft 0 von 18
  (?sm)^## Directive\s*\n+(.+?)\n\n  ->  trifft 18 von 18
```

Die vier übrigen Ausdrücke sind im selben Lauf gegen `.fusion-setup` und `orchestrator-live.md` gehalten worden und liefern, was C3.7 und C5.1 ihnen zuschreiben. Die Kennzeichen- und Pfadmuster sind in einem zweiten Lauf mitgeprüft und treffen ebenfalls; sie laufen gegen einen Eintragsnamen oder einen Pfad, also gegen eine einzige Zeile ohne Zeilenende, und dort heißt „Anfang der Eingabe" dasselbe wie „Anfang der Zeile".

Ein Nebenbefund fällt aus derselben Messung: die Zeile `## Directive` liegt in **keinem** der achtzehn Circle-Datensätze jenseits der 64-KB-Grenze aus C6.6. Der größte Datensatz ist zwar 119.614 Bytes groß, seine Überschrift steht aber bei Byte 222.

---

## Die zwei Berichtigungen, die über den Wortlaut hinausgehen

**C3.14 nannte eine Funktion, die die Runde nicht benutzt.** Das Kriterium schrieb `krk_core::text::datei::bis_zur_grenze_lesen` als den Leseweg vor. Diese Funktion weist eine Datei über der Grenze ab, statt sie anzulesen, während C6.6 das Anlesen verlangt; beide waren in ihrem Wortlaut nicht zugleich erfüllbar. Schritt 4 hat deshalb `anlesen` als dritte Hülle an derselben Tür gebaut, und Schritt 6 ruft sie. C3.14 nennt jetzt die Zusage statt der Funktion: gelesen wird über die Hüllen in `krk_core::text::datei`, die sämtlich durch `verzeichnis::sys::ohne_warten_oeffnen` gehen und den Typ am offenen Deskriptor prüfen. Die zweite Hälfte und der Nachweis sind unverändert geblieben.

Derselbe überholte Constraint stand an drei weiteren Stellen. Der Befund nannte zwei davon, den Constraints-Abschnitt des Specs hat er nicht genannt und den Datensatz `260824-0600_a_der-titel-aus-der-ueberschriftenzeile-…` auch nicht. Beide sind mitberichtigt, in derselben Form wie die schon dort stehende `**Berichtigung 260824-0910**`: der ursprüngliche Wortlaut bleibt stehen, die Berichtigung tritt daneben, und kein Marker bewegt sich.

**Die `Files:`-Zeilen hatten eine Wurzel, die im Plan nirgends stand.** Der Befund hält fest, dass die Zeilen der Schritte 2 bis 6 fünfmal einzeln nachgezogen werden mussten, weil keine Stelle des Plans sagte, in welche Datei eine Probe gehört. Von den offenen Schritten trug genau einer dieselbe Lücke: Schritt 8, dessen Proben zu C1.1, C1.2 und C1.5 bis C1.8 einen Prüfordner brauchen, den in `krk-core` nur eine Datei unter `tests/` erreicht. Er ist um `crates/krk-core/tests/ablage.rs` nachgezogen, was zugleich dem Defekt `260824-0940` entgegenkommt, der dort ohnehin ein Nachziehen verlangt. Die übrigen fünf offenen Schritte tragen die Lücke nicht, und der Grund ist je Schritt notiert. Die Regel selbst steht jetzt unter `## Testing Strategy`.

---

## Was dem Nutzer vorzulegen ist

Vier Abnahmekriterien des am 260824-0625 freigegebenen Specs sind **inhaltlich** geändert: C3.8 (der Ausdruck), C3.14 (der Leseweg), C4.3 (mehrzeilige Werte) und C6.1 (der eigene Leselauf des Feldbausteins). C3.9 hat einen Satz nachbekommen und ist inhaltlich unberührt.

Jede Berichtigung steht als solche im Spec, unter der Kriterienliste ihres Abschnitts, mit dem ursprünglichen Wortlaut, dem Grund und der Messung. Der freigegebene Text ist an keiner Stelle spurlos ersetzt.

---

## Nebenbefunde, in derselben Buchführung mit erledigt

Drei Kopf- und Vorbehaltszeilen standen auf einem Stand von vor dem Spec-Tor und sind nachgezogen: die Statuszeile des Specs („Entwurf, noch nicht gebaut"), die des Plans („Entwurf, wartet am Tor", bei sieben erledigten Schritten) und zwei Punkte unter `## User Decisions Pending`, von denen einer die Festlegungen A1 bis A7 als unbestätigt führte und einer einen längst geschlossenen Defekt als offen. Fünf Verweise im Plan trugen den Marker `_o_` auf Dateien, die jetzt `_c_` heißen; sie stehen in der Sternform, wie es für ein Planungsdokument gilt.

---

## Grenzen dieser Sitzung

Wir haben keine Datei unter `crates/` angefasst und keinen der sieben Datensätze, an denen der `coder` parallel arbeitet. Die Auslieferungsfassung `resources/default-readers.toml` steht weiterhin aus; Schritt 7 schreibt jetzt die richtigen Ausdrücke vor, gebaut ist sie nicht. Ob die Berichtigungen an den vier Abnahmekriterien Bestand haben, entscheidet der Nutzer und nicht diese Sitzung.

**Werkzeug:** das Wegwerfprogramm liegt unter dem Zwischenablageverzeichnis dieser Sitzung und nicht im Baum. Es ist Beleg und kein Artefakt; wer die Messung wiederholen will, baut es in drei Minuten nach.
