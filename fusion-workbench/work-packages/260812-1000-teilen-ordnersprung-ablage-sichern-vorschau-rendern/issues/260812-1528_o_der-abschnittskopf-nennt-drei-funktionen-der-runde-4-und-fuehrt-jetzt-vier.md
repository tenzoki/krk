Der Abschnittskopf über `teilen` nennt drei Funktionen der Runde 4 und führt jetzt vier

---

`teilen` ist in den Abschnitt „Pfade kopieren und mit dem Standardprogramm öffnen"
gesetzt worden (`resources/default-keymap.toml:607-626`). Dessen Kopf steht unverändert
(`:573-580`):

> Drei Funktionen der Runde 4. Sie stehen neben C11, weil sie dieselbe Sorte Handlung
> sind: KRK gibt den angezeigten Ordner oder die betroffenen Eintraege an etwas
> ausserhalb der Liste weiter und zeigt selbst nichts davon. **Die drei Kombinationen
> sind der Nutzerentscheid vom 260811-1505** (`decisions/260811-1300_*_welche-vier-kombinationen-gelten-ab-werk.md`, Moeglichkeit 1).

Der Abschnitt führt seit `95b2dfa` vier Blöcke: `ordnerpfad_kopieren` (`:582`),
`eintragspfad_kopieren` (`:593`), `teilen` (`:607`), `mit_standardprogramm_oeffnen`
(`:629`). Zwei Aussagen des Kopfes treffen den vierten nicht:

- **„Drei Funktionen der Runde 4."** `teilen` gehört zur Runde 6
  (`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`).
- **„Die drei Kombinationen sind der Nutzerentscheid vom 260811-1505."**
  `shift+cmd+s` stammt aus dem Entscheid vom 260812-1105
  (`.../decisions/260812-1000_i_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`).

Die Begründung dazwischen trägt `teilen` dagegen: es gibt die betroffenen Einträge an
etwas außerhalb der Liste weiter und zeigt selbst nichts davon. **Der Platz ist richtig,
der Kopf ist alt.** Dass er falsch geworden ist, hält keine Probe an: die Zählzeile im
Dateikopf prüft `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs:1513`), Abschnittsköpfe prüft nichts.

---

**Herkunft:** Directive dieser Runde, Commit `95b2dfa` (Schritt 4 des Plans
`.../planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`).

**Empfehlung:** den Kopf auf vier Funktionen ziehen und beide Herkünfte nennen, statt
die Zahl allein hochzusetzen — der Abschnitt trägt jetzt zwei Runden und zwei
Nutzerentscheide, und genau das gehört in den Kopf. Die Zeile über `Cmd+W`
(`:579-580`) bleibt davon unberührt.

**Zu prüfen bei der Berichtigung:** ob die Sachgruppe weiter nach Runden benannt sein
soll. Ein Kopf, der eine Runde nennt, veraltet mit der nächsten Funktion, die
dazukommt; ein Kopf, der die Sorte Handlung nennt, nicht. 13 der übrigen 14
Abschnittsköpfe der Datei nennen eine Fähigkeit (`C1` bis `C11`) und keine Runde; ohne
Fähigkeitskennung stehen außer diesem nur „Der eingebaute Editor" (`:642`), und der
nennt dafür keine Runde.
