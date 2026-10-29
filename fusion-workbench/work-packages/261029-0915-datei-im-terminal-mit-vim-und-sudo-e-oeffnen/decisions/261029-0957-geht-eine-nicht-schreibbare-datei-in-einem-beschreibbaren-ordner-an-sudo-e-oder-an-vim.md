# Geht eine Datei, die der Nutzer nicht schreiben darf, in einem Ordner, den er beschreiben darf, an `sudo -e` oder an vim?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md

---

## Question

Der Nutzer hat am 261029 entschieden, dass `opt+cmd+e` die Datei unter dem Cursor im Terminal öffnet: mit `vim`, wenn er sie schreiben darf, sonst mit `sudo -e`. KRK prüft das mit `access(2)` und `W_OK` (`krk_core::verzeichnis::sys::schreiben_erlaubt`). Beim Planen fällt ein Fall auf, in dem diese Regel immer in eine Weigerung führt. `sudo -e` (sudoedit) bearbeitet keine Datei in einem Ordner, den der aufrufende Nutzer beschreiben darf, und sagt das im Terminal. `inference:` Das ist die Vorgabe `sudoedit_checkdir` von sudo ab 1.8.16, und das sudo von macOS 15 liegt darüber. Am Gerät gemessen ist es nicht.

Der Fall betrifft zwei Arten von Dateien:

- eine Datei, die root gehört, in einem Ordner des Nutzers. Diesen Fall hat der Nutzer schon als bekannte Grenze angenommen.
- **eine eigene Datei ohne Schreibbit** (`chmod 444`) im eigenen Ordner. Sie ist vermutlich der häufigere Fall, etwa eine schreibgeschützte Konfiguration oder eine Datei, die ein Werkzeug so abgelegt hat. Nach der Regel geht sie an `sudo -e`, sudo fragt nach dem Passwort oder weigert sich vorher, und die Datei wird nicht geöffnet.

Die Frage ist, ob die Wahl außer der Datei auch ihren Ordner fragt. Sie muss vor Schritt 1 des Plans beantwortet sein, weil sie die Eingabe der reinen Wahlfunktion festlegt.

## Options

1. **Die Regel des Nutzers ohne Zusatz.** Schreibbar heißt vim, sonst `sudo -e`, und der Ordner wird nicht gefragt.
   - Pros: genau der Wortlaut des Nutzerentscheids. Eine Eingabe und eine Frage an das System. Eine Datei, die root gehört und in einem geschützten Ordner liegt (`/etc`), geht wie gewollt an `sudo -e`.
   - Cons: in einem beschreibbaren Ordner führt jede nicht schreibbare Datei in eine Weigerung im Terminal, auch die eigene. Die Anleitung muss das als Grenze nennen.
2. **Auch den Ordner fragen.** Ist die Datei schreibbar, vim. Ist sie es nicht, aber ihr Ordner, ebenfalls vim, denn `sudo -e` würde dort sicher ablehnen. Sonst `sudo -e`.
   - Pros: kein Weg führt mehr in eine sichere Weigerung. Eine eigene schreibgeschützte Datei öffnet in vim, das sie als schreibgeschützt anzeigt, und `:w!` schreibt sie, weil der Ordner es erlaubt. Die bekannte Grenze aus der Anleitung fällt weg. Gefragt wird mit derselben Hülle `schreiben_erlaubt` am Elternordner, ohne neue Bindung.
   - Cons: weicht vom Wortlaut des Nutzerentscheids ab. Eine Datei, die root gehört und im eigenen Ordner liegt, öffnet in vim statt als root. `:w!` ersetzt sie dort durch eine Datei, die dem Nutzer gehört, und das ist eine Wirkung, die der Nutzer kennen muss. Zwei Fragen an das System statt einer, und die Wahlfunktion bekommt zwei Eingaben.

## Constraints

- KRK hält, speichert und übergibt kein Passwort.
- Keine dritte `unsafe`-Datei und keine neue Bindung. Beide Möglichkeiten kommen mit `schreiben_erlaubt` aus.
- Bewusstes Öffnen als root bei einer schreibbaren Datei gibt es in keiner der beiden Möglichkeiten (Nutzerentscheid vom 261029).

## Recommendation

Möglichkeit 2. Sie hält die Absicht des Nutzerentscheids, dass KRK den Weg nimmt, der zum Bearbeiten führt, an einer Stelle, an der der Wortlaut in eine sichere Weigerung führt. Ihr Preis ist eine zweite Frage an dieselbe Hülle. Der Plan ist trotzdem auf Möglichkeit 1 gebaut, weil sie der Entscheid ist, wie er heute steht. Möglichkeit 2 ändert allein `terminalskript::befehl_fuer`, ihre Proben in Schritt 1 und den Abschnitt in `HowTo.md` (Schritt 6). Beantwortet der Nutzer die Frage nicht vor dem Bau, gilt Möglichkeit 1.

---
Answered: Nutzer im Chat, 261029, Antwort „1, Aa Ba Cb“ zur Planfreigabe — Möglichkeit 2: zusätzlich den Ordner fragen; ist er schreibbar, vim; ruled by user, Kai Stalmann <ks@qantr.com>
Implemented: 36efb7f3 (befehl_fuer: vim wenn Datei oder Ordner schreibbar, sonst sudo -e); Nutzerabnahme 261029 bestanden
