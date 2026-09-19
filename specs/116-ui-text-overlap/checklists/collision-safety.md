# Collision-Safety Checklist

## Allocation

- [x] Group label width is measured from the actual title font
- [x] Label width is bounded against minimum value and interaction reserves
- [x] Row, label, value, and interaction allocations are ordered and contained
- [x] Label, value, and interaction painters are clipped to their cells

## Accessibility

- [x] Truncated titles keep the exact complete AccessKit name
- [x] Truncated titles expose complete hover tooltip text without adding focus stops
- [x] Values keep their exact accessible text and tooltip behavior
- [x] Theme and enlarged logical-text fixtures pass

## Regression Gate

- [x] Pre-fix implementation demonstrates the expected failing collision test
- [x] Shipping title inventory is complete and centrally exercised
- [x] Semantic rectangles do not intersect beyond 0.5 points
- [x] Visible painted text remains inside the owning cell clip
- [x] System and State integration fixtures pass
- [x] ESO Weave Data Details integration fixtures pass
- [x] Existing UI sizing and accessibility tests remain green
