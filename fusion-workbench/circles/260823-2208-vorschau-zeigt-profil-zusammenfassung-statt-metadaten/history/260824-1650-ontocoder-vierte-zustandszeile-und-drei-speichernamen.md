# Ontocoder: die vierte Zustandszeile und die drei zusätzlichen Speichernamen

**Status:** Complete
**Datum:** 260824-1650
**Executor:** ontocoder
**Planschritt:** Schritt 14 des Plans `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel C
**Grundlage:** die zwei Antworten des Nutzers vom 260824-1505, beide Möglichkeit 2:
`decisions/260824-0634_a_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`
und `decisions/260824-1313_a_deckt-das-speicherprofil-auch-decisions-memos-und-investigations-ab.md`

---

## Was geändert ist

Eine Datei, `resources/default-readers.toml`, zwei Änderungen am Inhalt und zwei an den
Kommentaren, die sie beschreiben.

**Erstens die vierte Zustandszeile.** Das Profil „fusion-Werkbank: eine Runde" trägt hinter
„Geschlossen" und vor der Directive-Zeile ein weiteres Vorhandensein:

```toml
  [[profil.zeile]]
  beschriftung = "Abgelegt"
  vorhandensein = { muster = '^_[sd]_circle\.md$' }
```

**Zweitens das gewachsene Pfadmuster des Speicherprofils.** Aus sechs Alternativen sind neun
geworden; neu sind `decisions`, `investigations` und `memos`:

```toml
pfad = 'fusion-workbench/(shared|circles/[^/]+)/(analyses|backlog|consult|decisions|history|investigations|memos|planning|reviews)$'
```

**Die Kommentare ziehen mit.** Der Kopf des Speicherprofils sprach von „zwölf Orten" und
„sechs Speichern" und spricht jetzt von achtzehn und neun; daneben steht der Satz, der die
offen gebliebene Frage aus dem Entscheidungsdatensatz festhält, nämlich dass ein
Entscheidungsspeicher damit dieselben zwei Zeilen trägt wie ein Analysespeicher und wie er
sich später ohne Bruch herauslösen ließe. Der Kopf des Rundenprofils sprach von „drei Zeilen"
und schloss mit dem Satz, eine überholte oder zurückgestellte Runde antworte auf alle drei mit
„nein"; er spricht jetzt von vier Zeilen und zählt aus, welcher der sechs Marker in welche
Zeile fällt. Der alte Schlusssatz wäre nach dieser Änderung falsch gewesen und ist deshalb
ersetzt und nicht bloß ergänzt.

## Gegen den echten Bestand gehalten

Beide Ausdrücke sind vor dem Hinschreiben gegen die Dateinamen und Pfade dieser Werkbank
gemessen worden, wie es Schritt 7 vorgemacht hat und weil zwei Feldmuster dieser Datei an der
Verankerungsfrage schon einmal falsch gewesen sind
(`issues/260824-1124_c_zwei-feldmuster-der-auslieferungsfassung-verankern-mit-dach-und-koennen-nie-treffen.md`).

**Die vier Zustandszeilen gegen die achtzehn Circle-Verzeichnisse.** Der Bestand trägt zehn
`_b_`, fünf `_c_`, zwei `_d_` und ein `_t_`; `_a_` und `_s_` kommen heute nicht vor. Gemessen:

| Zeile | Muster | Treffer |
|---|---|---|
| Vorgesehen | `^_a_circle\.md$` | 0 |
| Aktiv | `^_t_circle\.md$` | 1 |
| Geschlossen | `^_[cb]_circle\.md$` | 15 |
| Abgelegt | `^_[sd]_circle\.md$` | **2** |

Die zwei sind `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` und
`260816-2255-befehle-absetzen-und-makros-speichern`, dieselben zwei, die der Datensatz nach
seiner Berichtigung vom 260824-1508 nennt. **Jedes der achtzehn Verzeichnisse bejaht jetzt
genau eine der vier Zeilen**, keines zwei und keines keine; vor der Änderung antworteten zwei
dreimal mit „nein".

**Das Pfadmuster gegen die Unterordner von `shared/` und der achtzehn Runden.** 118 Ordner
insgesamt:

| | vorher | nachher |
|---|---|---|
| Speicherprofil | 78 | **99** |
| Defektspeicher | 19 | 19 |
| ohne Profil | 21 | **0** |

Die 21 neu erfassten sind `shared/investigations`, `shared/memos` und die neunzehn
`decisions`-Ordner, also der gemeinsame und je einer in jeder der achtzehn Runden. Die Zahl
21 deckt sich mit der Messung im Entscheidungsdatensatz vom 260824-1313.

## Was sich nicht geändert hat

**Der Haushalt.** Das neue Vorhandensein trägt `muster` und keinen `ordner`, prüft also die
Liste des für die Erkennung ohnehin gelesenen Ordners: kein zusätzlicher Verzeichnisleselauf,
keine Dateiöffnung. Die zweite Änderung betrifft die Erkennung und keinen Baustein. Das
Rundenprofil steht unverändert bei fünf Leseläufen und elf Öffnungen, C6.7 ist unberührt.

**Der Bausteinsatz** bleibt bei vier, und die Datei führt weiter **fünf** Profile.

**Keine neue Probe.** `ablage::leseprofile::tests::die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
zerlegt den eingebetteten Text und schickt ihn durch dieselbe Prüfung wie eine Nutzerdatei;
sie deckt beide Zeilen mit ab und ist der Beleg dafür, dass die zwei neuen Ausdrücke sich in
`regex` übersetzen lassen.

## Prüfung

`make check` — Exit 0, alle vier Kommandos grün. Die zwei einschlägigen Proben laufen mit:
`die_eingebettete_fassung_besteht_ihre_eigene_pruefung` und
`die_auslieferungsfassung_nennt_jeden_bausteinnamen`.

Ein zweiter Agent hat währenddessen an `crates/krk-ui/src/appkit/vorschau.rs` gearbeitet
(Schritt 10); der Lauf war zum Prüfzeitpunkt grün und trug keinen Befund aus `krk-ui`.

## Offen für den Orchestrator

Die zwei Entscheidungsdatensätze gehen mit diesem Schritt von `_a_` auf `_i_` und brauchen
dafür die Zeile `Implemented:` mit dem Commit-Hash. Der Commit steht aus, also ist die
Umbenennung nicht ausgeführt.
