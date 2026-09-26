Die alten Zettel werden bei jedem neuen Anlegen von `~/krkhome` erneut übernommen, obwohl die Übernahme einmalig sein soll

---

`heimordner::bereitstellen` übernimmt `note-1.txt` und `note-2.txt` aus dem Ablageordner in eine neu entstehende `notes.txt`, sobald das `mkdir(2)` des Notizordners gelingt. Die zwei alten Dateien bleiben danach liegen, wie `260926-0007_*_was-geschieht-mit-den-zwei-zetteln-des-bisherigen-notizblatts.md` es verlangt. Löscht der Nutzer `~/krkhome` und drückt später F2, entsteht der Ordner neu, und dieselben Zettel wandern ein zweites Mal hinein, samt Text, den er am alten Ort vielleicht absichtlich gelöscht hat. Der Datensatz hat aber eine **einmalige** Übernahme beantwortet („Möglichkeit 1, einmalige Übernahme, ausgelöst allein durch das Anlegen des Ordners“); gebaut ist eine Übernahme je Anlegen.

---
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-1506_*_plan-home-menue-und-einstellbarer-ort.md, 260926-1520-zweitlesung-home-menue-und-einstellbarer-ort.md

**Beleg.** Gefunden von der Zweitlesung (Frage 3, Punkt O4): auf diesem Gerät stehen `note-1.txt` und `note-2.txt` nach der Übernahme weiter im Ablageordner. Die Bedingung der Übernahme ist allein `ordner_angelegt` in `crates/krk-core/src/heimordner/bereitstellen.rs`; ein Merker, dass sie schon gelaufen ist, steht nirgends.

**Warum nicht im Plan `260926-1506_*_…`.** Der Plan beschränkt die Übernahme auf den Vorgabeort (`ordner_angelegt && heim.ist_vorgabeort()`) und macht die Lücke damit wahrscheinlicher (Umzug an einen anderen Ort, `~/krkhome` gelöscht, Rückkehr zum Vorgabeort), schafft sie aber nicht. Ein Merker in `session.toml`, den die Zweitlesung vorschlägt, hielte sie nicht dicht: eine Instanz ohne Sitzungsrecht schreibt die Sitzung nie, und eine beschädigte `session.toml` wird durch die Auslieferung ersetzt und verlöre den Merker. Ein dichter Merker ist eine weitere Ablagedatei (`Datei::ALLE` wächst, mit allen Prosastellen, die die Ablage zählen) oder ein Eingriff in die alten Zettel selbst (etwa Umbenennen nach der Übernahme), und der zweite widerspricht der Antwort „alte Dateien unangetastet“.

**Zu entscheiden.** Ob die Einmaligkeit hergestellt wird, und wenn ja, über welchen Merker; oder ob die wiederholte Übernahme als Lage angenommen wird und `HowTo.md` sie nennt, samt dem Rat, die alten Zettel nach der ersten Übernahme selbst zu entfernen.

**Abnahme.** Entweder übernimmt ein zweites Anlegen von `~/krkhome` nach einer gelungenen ersten Übernahme nichts mehr, gehalten von einer Probe in `crates/krk-core/tests/heimordner.rs`, und `make check` ist grün. Oder `HowTo.md` beschreibt die Wiederholung und wie der Nutzer sie vermeidet, und dieser Datensatz ist als Lage angenommen geschlossen.
