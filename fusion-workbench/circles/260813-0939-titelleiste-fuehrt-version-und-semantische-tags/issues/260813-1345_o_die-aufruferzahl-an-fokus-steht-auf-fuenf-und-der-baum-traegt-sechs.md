Die Aufruferzahl an `fokus` steht auf fünf, und der Baum trägt sechs

---

`crates/krk-ui/src/appkit/anwendung.rs:4163-4165` sagt nach A2:

```
/// [`Self::fokus`] bleibt fuer die fuenf uebrigen Aufrufer stehen, die den
/// Wert nicht schon in der Hand haben.
```

Es sind sechs: `anwendung.rs:1084`, `:1168`, `:1713`, `:3454`, `:4822`, `:5286`. Die Treffer
bei `:2656` und `:2713` sind Prosa und zählen nicht mit.

---

**Schwere:** niedrig. Kein Verhalten, keine Probe hängt daran, der Bau läuft grün.

**Warum die Zahl falsch in den Baum gekommen ist, und warum sie zweimal bestätigt wurde**

Der Plan zählt in A2 fünf Stellen auf und nennt sie einzeln (`:1112`, `:1657`, `:3346`, `:4702`,
`:5166`, Zeilennummern vor der Änderung). Der Aufruf im Rückruf des Ordnerwechsels heißt
`selbst.fokus()` und nicht `self.fokus()` (`anwendung.rs:1084`) und entgeht damit dem Muster,
das die übrigen fünf gefunden hat. Der Ausführer hat die Zahl aus dem Plan übernommen, und die
Durchsicht von Turn 1 hat sie nachgezählt und bestätigt
(`reviews/260813-1258-coderev-turn-1-titelleiste-version-und-tags.md`, Abschnitt „Was sonst
geprüft und in Ordnung ist": „nachgezählt: `:1168`, `:1713`, `:3454`, `:4822`, `:5286`") — mit
derselben Blindheit gegen die Empfängerform.

**Derselbe Fehler ist in diesem Projekt schon einmal aufgetreten.** Die Runde 7 führt ihn als
`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0540_c_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`
(geschlossen). Dort traf er zwei Zählproben; hier trifft er nur eine Prosazahl, weil keine Probe
die Aufrufer von `fokus` zählt.

**Was zu tun ist**

Die Zahl auf sechs setzen. Wer sie prüfbar machen will, braucht eine Zählprobe, die nach der
Sache statt nach der Schreibweise sucht — die Runde 7 hat für genau diese Lehre einen
geschlossenen Datensatz.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Die Aussage, um die es sachlich geht, hält: `lage` erhebt das Schlüsselfenster genau einmal
  (`anwendung.rs:2664`) und reicht es an `fokus_bei` weiter (`:2672`). Falsch ist allein die Zahl
  der Aufrufer, die die Hülle noch braucht.
