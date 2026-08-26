Die sechs fachlichen `assert!` der Kindstarter-Rufer sind seit dem Gate unerreichbar

---

`kind_mit_deskriptorgrenze` (`crates/krk-core/tests/gemeinsam/mod.rs:513-540`) hält seit `17e5e4e` selbst `status.success() && stdout.contains("test result: ok. 1 passed;")` und bricht sonst mit Panik ab. Die sechs Rufer prüfen danach genau dasselbe: `assert!(ergebnis.status.success(), <fachliche Meldung>, stdout, stderr)`. Ihre Bedingung kann nie falsch sein, wenn der Aufruf zurückkehrt — die sechs Zusicherungen sind toter Code, und die Meldung, die die eigentliche Zusage benennt, erscheint bei keinem Fehlschlag mehr.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-core/tests/gemeinsam/mod.rs:64-66`, `:510-512`, `:513-540`; `crates/krk-core/tests/umfang.rs:264-270`, `:352-358`; `crates/krk-core/tests/verzeichnis.rs:2573-2579`, `:2779-2785`, `:2873-2879`; `crates/krk-core/tests/leseprofil.rs:3489-3495`
**Cross-references:** `shared/issues/260826-1302_c_sechs-elternproben-am-gemeinsamen-kindstarter-bleiben-gruen-wenn-der-kindname-nicht-trifft.md` (behoben, dieser Befund ist die Folge der Behebung); `shared/planning/260826-1811_c_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`, Schritt 3

## Der Befund

Das Gate im Starter (`gemeinsam/mod.rs:528-537`):

```rust
assert!(
    ergebnis.status.success() && stdout.contains(EIN_KIND_GELAUFEN),
    "die Kindprobe `{name}` ist nicht als genau ein Kind gelaufen …"
);
ergebnis
```

Der Rumpf jedes der sechs Rufer, hier `umfang.rs:264-270`:

```rust
assert!(
    ergebnis.status.success(),
    "ein Deskriptormangel des Prozesses wird zu einer Zahl ueber eine Auswahl\n\
     --- stdout ---\n{}\n--- stderr ---\n{}",
    String::from_utf8_lossy(&ergebnis.stdout),
    String::from_utf8_lossy(&ergebnis.stderr)
);
```

`ergebnis` existiert nur, wenn das Gate schon durchgelassen hat, und das Gate verlangt `status.success()` als eine seiner zwei Bedingungen. Die Bedingung der sechs Rufer ist damit eine Tautologie. Am Baum nachgelesen: alle sechs Rufer benutzen `ergebnis` **ausschließlich** in dieser einen Zusicherung, für nichts sonst.

## Was die Prosa an drei Stellen behauptet

1. **Modulkopf** (`gemeinsam/mod.rs:64-66`): „Die Rufer behalten ihr eigenes `assert!` als die fachliche Zeile; das Gate hier sagt nur, dass genau ein Kind gelaufen ist."
2. **Doc-Kommentar des Starters** (`:510-512`): „Die Ausgabe kommt zurueck, damit der Rufer seine fachliche Zusicherung mit derselben Meldung halten kann."
3. **Plan, Schritt 3**: „Der Starter liefert weiter `Output`, damit die Rufer ihre eigene Meldung behalten können; ihre sechs `assert!` bleiben als die fachliche Zeile stehen."

Keine der drei trägt: die fachliche Zeile ist nicht mehr erreichbar. Wer eine der sechs Zusagen bricht, liest die generische Meldung „trifft der Name nicht, oder fehlt dem Kind sein `#[ignore]`?" statt „ein Deskriptormangel des Prozesses wird zu einer Zahl über eine Auswahl". Das ist genau die Auskunft, die die sechs Meldungen einzeln geschrieben haben, um sie zu geben.

Der Rückgabewert `Output` ist damit an keiner Stelle mehr benutzt.

## Zwei Wege, einer davon der schmalere

- **Die sechs `assert!` streichen** und `kind_mit_deskriptorgrenze` `()` liefern lassen. Ehrlich, aber die sechs fachlichen Sätze fallen mit.
- **Den fachlichen Satz in den Starter hineinreichen**: ein Parameter `was: &str`, den das Gate seiner Meldung voranstellt, wie `mit_zeitschranke` (`gemeinsam/mod.rs:276-282`) es mit seinem `was` schon vorführt — dieselbe Bauform, dieselbe Begründung („damit ein Fehlschlag sagt, welche hängt"). Dann steht die fachliche Zeile weiter da, an der einen Stelle, an der sie auch ausgelöst wird.

Der zweite Weg ist der schmalere und hat die Vorlage im selben Modul.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.
