# Bugfix: PDF-Betrachter stuerzt beim Zoomen mit Stapelueberlauf ab

**Date:** 2026-08-28 09:17
**Status:** Complete
**Trigger:** Orchestrator (Absturzbericht des Nutzers, `analyses/260828-0912-absturzbericht-pdfview-delegate-rekursion.txt`)

## Error

KRK 1.2.1 auf `03af590`: beim Anzeigen/Zoomen eines PDFs `EXC_BAD_ACCESS` im Stack Guard, Thread 0 rekursiert unbegrenzt in `PDFKit -[PDFView PDFViewWillChangeScaleFactor:toScale:]` ueber `-[PDFView delegate]`.

## Root Cause

`crates/krk-ui/src/appkit/betrachter.rs:384` (Stand `03af590`) setzt die Ansicht als ihren eigenen Delegierten, und `Pdfbetrachter` implementiert `PDFViewDelegate` (`:273`). `PDFView` beantwortet selbst Selektoren mit den Namen seiner Delegiertenmethoden und reicht darin an den Delegierten weiter, sobald der `respondsToSelector:` mit ja beantwortet. Beleg: `swift`-Probe `PDFView(frame:.zero).responds(to: "PDFViewWillChangeScaleFactor:toScale:")` liefert `true` auf diesem SDK (`PDFView.h:375` fuehrt die Methode im Protokoll; die Klasse selbst antwortet ebenfalls). Eine Unterklasse erbt diese Antwort, also fragt die Ansicht beim ersten Zoom ihren Delegierten (sich selbst), der dieselbe weiterreichende Fassung ausfuehrt, bis der Stapel ueberlaeuft. Die einzige Methode, die KRK selbst beantwortet (`PDFViewWillClickOnLink:withURL:`), ist nicht betroffen; betroffen ist jede von `PDFView` selbst weitergereichte.

## Fix

Ein eigenes Delegatenobjekt statt der Ansicht: `Verweisdelegierter`, `NSObject`-Unterklasse per `define_class!`, `MainThreadOnly`, ohne ivars und ohne Rueckverweis (die eine Antwort geht an `zwischenablage::im_browser_oeffnen`, nicht an den Betrachter). Die Ansicht haelt es als `Retained` im ivar `delegierter`, weil `PDFView` seinen Delegierten schwach haelt. `Pdfbetrachter` implementiert `PDFViewDelegate` nicht mehr. Modulkopf: Absatz unter „Verweise" und Untergrenzen-Abschnitt (`NSObject`, `init`) nachgezogen.

| File | Change |
|------|--------|
| `crates/krk-ui/src/appkit/betrachter.rs` | neue Klasse `Verweisdelegierter` mit der verschobenen `PDFViewWillClickOnLink:withURL:`; ivar `delegierter`; `neu` baut und setzt es; `PDFViewDelegate`-Impl am Betrachter entfernt; Modulkopf |

## Verification

- [x] Original error resolved: Ursache belegt und beseitigt; der Absturz selbst braucht KRK im Vordergrund und ist vom Nutzer zu pruefen (Zoom in einem PDF)
- [x] Full test suite passes: `make check` exit 0 (alle vier gruen)
- [x] No regressions introduced: `cargo xtask bundle` exit 0

## Unrelated Issues Found

None
