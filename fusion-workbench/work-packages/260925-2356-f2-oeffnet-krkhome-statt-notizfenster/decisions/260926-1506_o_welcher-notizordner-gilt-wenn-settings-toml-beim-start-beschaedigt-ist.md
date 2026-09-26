# Welcher Notizordner gilt, wenn `settings.toml` beim Start beschädigt ist?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-1447_*_bekommt-krkhome-ein-eigenes-menue-und-einen-einstellbaren-ort.md, 260926-1451_*_spec-home-menue-und-einstellbarer-ort.md, 260926-1506_*_plan-home-menue-und-einstellbarer-ort.md

---

## Question

Mit Stufe 2 des Spec kommt der Ort des Notizordners aus `settings.toml`. Für eine beschädigte Datei (kein gültiges TOML, ein unbekannter Schlüssel, ein Wert vom falschen Typ an `terminal`, oder eine Datei, die sich nicht lesen lässt) gilt heute eine Regel für die ganze Datei: KRK meldet den Schaden, legt die Datei beiseite und arbeitet mit der Auslieferungsfassung jedes Werts weiter (`ablage/einstellungen.rs`, `laden`). Der Spec setzt für den Ort eine zweite Regel daneben: ein unzulässiger **Wert** ergibt keinen Ersatzort, weil ein stiller Rückfall auf `~/krkhome` bei F2 Dateien an einem Ort anlegte, den der Nutzer gerade verlassen wollte. Für die beschädigte **Datei** sagt er nichts, und die zwei Regeln geben dort verschiedene Antworten. Der Plan braucht die Antwort in Schritt 2.4, an genau einem Zweig; er baut auf die Empfehlung und hält nicht an.

## Options

1. **Kein Ort.** Ist `settings.toml` beschädigt oder nicht lesbar (`Grund::Beschaedigt`, `Grund::NichtLesbar`), gilt kein Notizordner. F2 nennt den Grund und den Weg hinaus („`settings.toml` ist beschädigt: berichtigen und KRK neu starten“, ab Stufe 3 zusätzlich „oder den Ort über ‚Home‘ → ‚Ort wählen…‘ setzen“), legt nichts an und öffnet keinen Tab. „Ort wählen…“ schreibt nicht in eine Datei, die beim Wählen noch beschädigt ist; ist sie inzwischen berichtigt, schreibt es und setzt den Ort ohne Neustart, weil es die Datei unter der Sperre neu liest. **Geschärft nach der Zweitlesung `260926-1520-zweitlesung-home-menue-und-einstellbarer-ort.md` (Frage 7, M2 und M3):** Fehlte die Datei und ließ sich nicht anlegen (`Grund::NichtAnlegbar`), gilt der Vorgabeort, weil dann nachweislich niemand einen anderen Ort eingestellt hat. Kommt der Start gar nicht bis zum Lesen (Ablageordner nicht zu öffnen, Schreibsperre nicht zu nehmen), gilt ebenfalls kein Ort, mit dem Neustart als Weg; allein der Messmodus bleibt beim Vorgabeort.
   - Folgen: Dieselbe Regel wie beim unzulässigen Wert, also eine Regel statt zweier. Die Startmeldung zum Schaden steht schon heute in der Statuszeile. Die Sitzung behält den zuletzt gemerkten Ort, so dass nach dem Berichtigen keine Meldung über einen Ortswechsel kommt, wenn der Ort derselbe geblieben ist. An der Antwort hängt im Plan genau ein Zweig, `Some(Grund::Beschaedigt(_) | Grund::NichtLesbar(_))` in `notizort` (Schritt 2.4); die Fälle `NichtAnlegbar` und „nicht gelesen“ folgen aus dem Spec (kein stiller Rückfall) und nicht aus dieser Wahl.
   - Preis: F2 wirkt nicht mehr, auch wenn der Schaden an einer ganz anderen Zeile liegt (etwa ein Tippfehler an `terminal`) und der Nutzer den Ort nie eingestellt hat. Wer die Startmeldung übersieht, merkt es erst bei F2, dann aber mit Grund.
2. **Der Vorgabeort `~/krkhome`**, wie jeder andere Wert der Auslieferungsfassung.
   - Folgen: F2 wirkt weiter und legt bei Bedarf unter `~/krkhome` an. Hat der Nutzer seinen Ort woanders eingestellt, entstehen dort leere Dateien, und notiert er dort weiter, liegen seine Notizen nach dem Berichtigen an zwei Orten. Der Vergleich mit der Sitzung meldet beim Start einen Wechsel nach `~/krkhome` und merkt ihn; nach dem Berichtigen folgt die zweite Wechselmeldung zurück.
   - Preis: Genau der Schaden, den der Spec für den unzulässigen Wert ausschließt, nur über einen anderen Auslöser.
3. **Der zuletzt geltende Ort aus `session.toml`.**
   - Folgen: Im häufigen Fall (Schaden an einer anderen Zeile) führt F2 dorthin, wo der Nutzer zuletzt war. Ohne gemerkten Ort, etwa beim ersten Start nach dieser Arbeit, fiele die Wahl auf Möglichkeit 1 oder 2 zurück.
   - Preis: Eine zweite Quelle für den Ort neben `settings.toml`, und damit eine Regel, die der Nutzer aus der Datei allein nicht ablesen kann. Hat er Ort und Schaden in derselben Bearbeitung geändert, gilt der alte Ort, den er verlassen wollte. Eine zweite Instanz ohne Sitzungsrecht liest dieselbe Sitzung, schreibt sie aber nie; das ändert hier nichts, macht die Regel aber nicht einfacher.

## Constraints

- Beim Start wird kein Ort über das Dateisystem befragt und nichts angelegt (Constraints des Spec).
- `~/krkhome` wird bei einem unzulässigen Wert nicht stillschweigend eingesetzt (H2 des Spec).
- Eine beschädigte `settings.toml` wird von KRK nicht überschrieben (Modulkopf von `ablage/einstellungen.rs`, H3 des Spec).

## Recommendation

Möglichkeit 1. Sie ist die Regel, die der Spec für den unzulässigen Wert schon gewählt hat, und sie hat dieselbe Begründung: ein Ersatzort ist nur dann harmlos, wenn der Nutzer nie einen anderen eingestellt hat, und genau das weiß KRK bei einer beschädigten Datei nicht. Der Preis ist ein F2, das mit Grund nichts tut, bis die Datei berichtigt ist. Die Zweitlesung `260926-1520-zweitlesung-home-menue-und-einstellbarer-ort.md` stimmt ihr in der oben geschärften Fassung zu. Der Plan baut auf diese Möglichkeit; eine andere Antwort ändert allein den markierten Zweig in `heimordner::ort::notizort` (Schritt 2.4), seine Probe und den Satz in `HowTo.md`.
