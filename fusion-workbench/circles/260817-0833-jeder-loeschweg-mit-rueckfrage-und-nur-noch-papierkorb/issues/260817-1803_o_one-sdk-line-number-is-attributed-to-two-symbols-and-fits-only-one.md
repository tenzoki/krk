One SDK line number is attributed to two symbols and fits only one

---
`crates/krk-ui/src/appkit/volumes.rs:130-132` reads "`NSURLVolumeLocalizedNameKey` und
`NSURLVolumeIsLocalKey` (`NSURL.h:338`, `API_AVAILABLE(macos(10.7), …)`) seit 10.7". In the local
SDK `NSURLVolumeIsLocalKey` stands at `NSURL.h:338` and `NSURLVolumeLocalizedNameKey` at
`NSURL.h:344`. The availability is right for both; the line number fits only the second name.

---

**Severity:** Low. Behaviour is unaffected and the version floor is correct — both keys really are
`macos(10.7)` and neither is above 15. The defect is in the one thing this section exists for: a
line number that can be re-read at the SDK. Whoever re-reads `NSURL.h:338` for
`NSURLVolumeLocalizedNameKey` finds a different symbol and has to decide whether the number or the
name is wrong.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-ui/src/appkit/volumes.rs:130-132`
**Tree state:** `792995a`
**Domain:** code

## Measured

```
$ SDK=$(xcrun --show-sdk-path)
$ grep -n "NSURLVolumeLocalizedNameKey\|NSURLVolumeIsLocalKey" \
    "$SDK/System/Library/Frameworks/Foundation.framework/Headers/NSURL.h"
338:FOUNDATION_EXPORT NSURLResourceKey const NSURLVolumeIsLocalKey        API_AVAILABLE(macos(10.7), …)
344:FOUNDATION_EXPORT NSURLResourceKey const NSURLVolumeLocalizedNameKey  API_AVAILABLE(macos(10.7), …)
```

The other four line numbers this section gained or carries were re-read at the same SDK and all
four are right: `fileURLWithPath:` at `NSURL.h:52` with no annotation, `resourceValuesForKeys:error:`
at `NSURL.h:183` with `macos(10.6)`, `NSURLResourceKey` at `NSURL.h:17` with no annotation, and
`boolValue` at `NSValue.h:73` with no annotation. The count "Sieben Beruehrungen sind juenger" is
also right: five at 10.6 and two at 10.7.

## How it arose

`749a4f3` rewrote the sentence. Before it, `NSURLVolumeLocalizedNameKey` carried no line number at
all (`git show 1a57418:crates/krk-ui/src/appkit/volumes.rs`, line 53: "`NSURLVolumeLocalizedNameKey`
seit 10.7"). Adding the new key merged the two names into one parenthesis and gave the pair the new
key's line number.

## Direction

Give each name its own line number, `:344` and `:338`. If the pair is to stay in one clause, put the
line number only on the symbol it belongs to. The section is not machine-checked — whether it should
be is the open question
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`, three stages with
costs — so a hand-read number is all this surface has, and a merged citation is the one form that
survives a re-read while being wrong.
