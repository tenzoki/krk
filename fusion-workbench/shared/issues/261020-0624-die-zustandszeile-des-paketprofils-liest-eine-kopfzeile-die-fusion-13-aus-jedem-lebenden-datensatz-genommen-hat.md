Die Zustandszeile des Paketprofils liest eine Kopfzeile, die fusion 13 aus jedem lebenden Datensatz genommen hat

---

Das Profil „fusion-Werkbank: ein Arbeitspaket“ in `resources/default-readers.toml` liest den Zustand eines Arbeitspakets aus der Kopfzeile `**Status:**` seines Datensatzes (Zeile `beschriftung = "Status"`, `feldmuster = '(?m)^\*\*Status:\*\* *(.+)$'`). Seit der Umstellung auf fusion 13 (`1fe34c4f`, 261014) steht der Zustand als `status` in der `package.json` neben dem Datensatz, und die Umstellung hat die Kopfzeile aus jedem lebenden Datensatz entfernt (`git show 1fe34c4f` zeigt am Datensatz `261006-1916-fensterlage-und-groesse-ueberstehen-den-neustart.md` die entfernte Zeile `**Status:** open`). Ein neu angelegtes Arbeitspaket trägt sie nie. Die Vorschau zeigt deshalb an genau den Arbeitspaketen, deren Zustand am meisten interessiert, den offenen, den Platzhalter.

---

**Filed by:** policy-curator, Kai Stalmann <kai@stalmann.org>
**Domain:** data
**Cross-references:** 261020-0617-curator-run.md

Gemessen am 261020 gegen HEAD `ac4d1f8c`:

```
$ grep -h '^\*\*Status:\*\*' fusion-workbench/work-packages/*/2*-*.md | sort | uniq -c
   7 **Status:** done
$ jq -r .status fusion-workbench/work-packages/*/package.json | sort | uniq -c
  22 done
  15 dropped
   1 open
```

Die einzige offene Arbeitseinheit, `261006-1916-fensterlage-und-groesse-ueberstehen-den-neustart`, trägt keine Kopfzeile. Der Modulkopf über dem Profil („TRÄGT EIN ÄLTERER DATENSATZ DIE KOPFZEILE NICHT, steht der Platzhalter da“) beschreibt den Platzhalter als Fall älterer Datensätze; heute ist er der Fall jedes lebenden.

Ein `feld`-Baustein liest Text aus einer Markdown-Datei; ob er die `package.json` lesen kann oder ein anderer Baustein dafür nötig ist, ist nicht geprüft.

**Abnahme:** an einem Arbeitspaket mit `status` `open` in seiner `package.json` und ohne Kopfzeile `**Status:**` im Datensatz zeigt die Vorschau `open` und nicht den Platzhalter; der Modulkopf über dem Profil nennt die Quelle des Zustands so, wie sie seit fusion 13 ist. Ausführender: `data-implementer` (Profil in `resources/default-readers.toml`), `code-implementer`, falls ein neuer Baustein nötig ist.
