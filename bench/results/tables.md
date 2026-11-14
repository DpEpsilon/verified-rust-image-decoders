
### BMP corpus

files: 84, verified decodes 50; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): bmp-image: both-ok-equal 45, both-error 27, both-ok-DIFFERENT 3, incumbent-only-ok 7, verified-only-ok 2; gdk:bmp: both-ok-equal 40, both-error 24, both-ok-DIFFERENT 9, incumbent-only-ok 10, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp-image (45 files) | 8.62 | 8.38 | 8.85 / 8.31 | 464 | 9% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-strict | bmp-image (45 files) | 8.46 | 8.11 | 8.57 / 8.14 | 473 | 10% / 15% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-recommended-beta | bmp-image (45 files) | 8.46 | 7.75 | 8.63 / 7.64 | 473 | 14% / 19% NOISY | 26 (+0) | 1 |
| bmp-image | bmp-image (45 files) | 4.09 (0.47x) | 3.81 | 4.08 / 3.67 | 979 | 10% / 19% NOISY | 26 (+0) | 1 |
| bmp-verified | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 10.57 | 8.46 | 10.72 / 8.30 | 378 | 29% / 29% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-gdk | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 13.65 | 12.69 | 13.84 / 12.75 | 293 | 12% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-imagecompat | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 10.96 | 10.81 | 11.32 / 10.78 | 365 | 12% / 21% NOISY | 26 (+0) | 0 |
| bmp-verified | gdk:bmp (40 files) | 7.80 | 7.57 | 8.02 / 7.65 | 513 | 14% / 18% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-strict | gdk:bmp (40 files) | 7.91 | 7.34 | 8.03 / 7.28 | 505 | 12% / 17% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-recommended-beta | gdk:bmp (40 files) | 8.03 | 8.03 | 8.24 / 7.96 | 498 | 4% / 20% | 26 (+0) | 1 |
| gdk:bmp | gdk:bmp (40 files) | 2.89 (0.37x) | 2.77 | 2.89 / 2.75 | 1384 | 7% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 8.22 | 7.85 | 8.47 / 7.51 | 487 | 9% / 33% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-gdk | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 15.01 | 14.38 | 15.27 / 13.96 | 266 | 6% / 12% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-imagecompat | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 11.28 | 10.49 | 11.27 / 10.54 | 355 | 9% / 15% NOISY | 26 (+0) | 0 |

### BMP large

files: 7, verified decodes 5; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): bmp-image: both-ok-equal 5, both-error 1, incumbent-only-ok 1; gdk:bmp: both-ok-equal 5, incumbent-only-ok 2

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp24_4096x4096.bmp | 6.79 | 6.56 | 6.76 / 6.53 | 589 | 13% / 30% NOISY | 116 (+64) | 1 |
| bmp-verified-policy-strict | bmp24_4096x4096.bmp | 7.03 | 6.56 | 6.99 / 6.48 | 569 | 13% / 31% NOISY | 116 (+64) | 1 |
| bmp-verified-policy-recommended-beta | bmp24_4096x4096.bmp | 6.68 | 6.32 | 6.65 / 6.34 | 599 | 11% / 28% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-gdk | bmp24_4096x4096.bmp | 9.78 | 9.66 | 9.81 / 9.49 | 409 | 2% / 14% | 116 (+64) | 1 |
| bmp-verified-policy-imagecompat | bmp24_4096x4096.bmp | 8.94 | 8.70 | 9.07 / 8.66 | 447 | 8% / 18% NOISY | 116 (+64) | 1 |
| bmp-image | bmp24_4096x4096.bmp | 3.22 (0.47x) | 3.16 | 3.25 / 3.06 | 1243 | 8% / 38% NOISY | 164 (+112) | 0 |
| gdk:bmp | bmp24_4096x4096.bmp | 1.61 (0.24x) | 1.58 | 1.63 / 1.52 | 2486 | 8% / 16% NOISY | 106 (+49) | 0 |
| bmp-verified | bmp24_8192x2048.bmp | 6.79 | 6.56 | 6.90 / 6.50 | 589 | 20% / 31% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-strict | bmp24_8192x2048.bmp | 6.91 | 6.68 | 6.81 / 6.47 | 579 | 9% / 31% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-recommended-beta | bmp24_8192x2048.bmp | 6.91 | 6.91 | 6.96 / 6.51 | 579 | 3% / 28% | 116 (+64) | 0 |
| bmp-verified-policy-gdk | bmp24_8192x2048.bmp | 9.66 | 9.42 | 9.78 / 9.24 | 414 | 8% / 27% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-imagecompat | bmp24_8192x2048.bmp | 8.94 | 8.82 | 9.02 / 8.72 | 447 | 1% / 11% | 117 (+64) | 1 |
| bmp-image | bmp24_8192x2048.bmp | 3.22 (0.47x) | 3.10 | 3.21 / 3.04 | 1243 | 8% / 19% NOISY | 164 (+112) | 1 |
| gdk:bmp | bmp24_8192x2048.bmp | 1.61 (0.24x) | 1.49 | 1.62 / 1.49 | 2486 | 22% / 30% NOISY | 105 (+49) | 0 |
| bmp-verified | bmp32_4096x3840.bmp | 6.74 | 6.61 | 6.82 / 6.55 | 594 | 17% / 46% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-strict | bmp32_4096x3840.bmp | 6.74 | 6.61 | 6.82 / 6.55 | 594 | 6% / 21% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-recommended-beta | bmp32_4096x3840.bmp | 6.99 | 6.87 | 6.95 / 6.65 | 572 | 7% / 28% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-gdk | bmp32_4096x3840.bmp | 9.03 | 8.90 | 9.19 / 8.78 | 443 | 10% / 22% NOISY | 125 (+60) | 0 |
| bmp-verified-policy-imagecompat | bmp32_4096x3840.bmp | 9.28 | 9.16 | 9.39 / 8.96 | 431 | 3% / 14% | 124 (+60) | 0 |
| bmp-image | bmp32_4096x3840.bmp | 3.43 (0.51x) | 3.12 | 3.41 / 3.10 | 1165 | 57% / 65% NOISY | 169 (+105) | 1 |
| gdk:bmp | bmp32_4096x3840.bmp | 1.53 (0.23x) | 1.49 | 1.62 / 1.48 | 2621 | 32% / 41% NOISY | 130 (+61) | 0 |
| bmp-verified | bmp8_4096x4096.bmp | 7.99 | 7.87 | 8.05 / 7.50 | 501 | 11% / 35% NOISY | 84 (+57) | 1 |
| bmp-verified-policy-strict | bmp8_4096x4096.bmp | 7.87 | 7.75 | 7.90 / 7.59 | 508 | 6% / 27% NOISY | 84 (+57) | 1 |
| bmp-verified-policy-recommended-beta | bmp8_4096x4096.bmp | 7.99 | 7.75 | 8.06 / 7.62 | 501 | 15% / 42% NOISY | 84 (+57) | 0 |
| bmp-verified-policy-gdk | bmp8_4096x4096.bmp | 11.09 | 10.85 | 11.21 / 10.78 | 361 | 3% / 9% | 84 (+57) | 1 |
| bmp-verified-policy-imagecompat | bmp8_4096x4096.bmp | 10.25 | 10.01 | 10.25 / 9.75 | 390 | 6% / 18% NOISY | 84 (+57) | 0 |
| bmp-image | bmp8_4096x4096.bmp | 2.38 (0.30x) | 1.55 | 2.37 / 1.49 | 1678 | 74% / 70% NOISY | 132 (+105) | 0 |
| gdk:bmp | bmp8_4096x4096.bmp | 2.19 (0.27x) | 2.15 | 2.23 / 2.11 | 1830 | 7% / 19% NOISY | 74 (+47) | 1 |
| bmp-verified | bmp8rle_4096x4096.bmp | 5.48 | 2.86 | 5.54 / 2.73 | 729 | 98% / 59% NOISY | 74 (+47) | 0 |
| bmp-verified-policy-strict | bmp8rle_4096x4096.bmp | 5.42 | 5.25 | 5.42 / 5.15 | 737 | 11% / 25% NOISY | 74 (+47) | 1 |
| bmp-verified-policy-recommended-beta | bmp8rle_4096x4096.bmp | 5.36 | 3.04 | 5.42 / 2.81 | 746 | 86% / 62% NOISY | 74 (+47) | 1 |
| bmp-verified-policy-gdk | bmp8rle_4096x4096.bmp | 5.66 | 2.98 | 5.72 / 2.88 | 706 | 96% / 56% NOISY | 74 (+47) | 0 |
| bmp-verified-policy-imagecompat | bmp8rle_4096x4096.bmp | 6.68 | 6.56 | 6.67 / 6.31 | 599 | 2% / 15% | 74 (+47) | 1 |
| bmp-image | bmp8rle_4096x4096.bmp | 2.30 (0.42x) | 2.26 | 2.40 / 2.20 | 1736 | 11% / 22% NOISY | 122 (+95) | 0 |
| gdk:bmp | bmp8rle_4096x4096.bmp | 4.17 (0.76x) | 4.05 | 4.27 / 4.02 | 959 | 6% / 12% NOISY | 79 (+52) | 0 |
| probe_bmp24_16384x16384.bmp | - | probe, not timed (FileTooLarge) | | | | |
| bmp-image | probe_bmp24_8192x8192.bmp | 3.34 | 3.22 | 3.42 / 3.05 | 1198 | 8% / 26% NOISY | 644 (+448) | 0 |
| gdk:bmp | probe_bmp24_8192x8192.bmp | 1.67 | 1.64 | 1.71 / 1.59 | 2397 | 5% / 19% NOISY | 394 (+193) | 1 |

### TGA corpus

files: 294, verified decodes 252; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): tga-image: both-ok-equal 230, both-error 35, incumbent-only-ok 7, both-ok-DIFFERENT 22; tga-tinytga: verified-only-ok 101, both-error 31, incumbent-only-ok 11, both-ok-DIFFERENT 22, both-ok-equal 129; gdk:tga: both-ok-equal 230, both-error 27, incumbent-only-ok 15, verified-only-ok 3, both-ok-DIFFERENT 19

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-verified | tga-image (230 files) | 9.23 | 9.16 | 9.47 / 8.97 | 433 | 2% / 11% | 69 (+4) | 1 |
| tga-verified-policy-strict | tga-image (230 files) | 10.02 | 9.66 | 10.14 / 9.44 | 399 | 7% / 16% NOISY | 70 (+4) | 1 |
| tga-verified-policy-recommended-beta | tga-image (230 files) | 9.87 | 9.80 | 10.07 / 9.45 | 405 | 4% / 20% | 69 (+4) | 1 |
| tga-image | tga-image (230 files) | 3.27 (0.35x) | 2.20 | 3.25 / 2.13 | 1224 | 50% / 53% NOISY | 73 (+7) | 0 |
| tga-verified | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.33 | 8.90 | 9.43 / 8.86 | 429 | 14% / 27% NOISY | 69 (+4) | 0 |
| tga-verified-policy-gdk | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.97 | 9.54 | 10.15 / 9.45 | 401 | 13% / 18% NOISY | 70 (+4) | 1 |
| tga-verified-policy-imagecompat | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.97 | 9.54 | 10.11 / 9.57 | 401 | 6% / 12% NOISY | 69 (+4) | 1 |
| tga-verified | tga-tinytga (129 files) | 8.27 | 4.35 | 8.41 / 4.22 | 483 | 94% / 57% NOISY | 33 (+4) | 0 |
| tga-verified-policy-strict | tga-tinytga (129 files) | 9.33 | 4.63 | 9.44 / 4.67 | 429 | 112% / 67% NOISY | 33 (+4) | 0 |
| tga-verified-policy-recommended-beta | tga-tinytga (129 files) | 9.96 | 9.54 | 10.08 / 9.54 | 402 | 96% / 98% NOISY | 33 (+4) | 0 |
| tga-tinytga | tga-tinytga (129 files) | 18.09 (2.19x) | 17.53 | 18.52 / 17.58 | 221 | 9% / 15% NOISY | 34 (+4) | 0 |
| tga-verified | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 9.02 | 8.67 | 9.08 / 8.56 | 444 | 12% / 17% NOISY | 34 (+4) | 0 |
| tga-verified-policy-gdk | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 9.72 | 9.58 | 10.06 / 9.53 | 411 | 97% / 102% NOISY | 33 (+4) | 0 |
| tga-verified-policy-imagecompat | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 18.60 | 9.72 | 18.60 / 9.63 | 215 | 104% / 60% NOISY | 33 (+4) | 0 |
| tga-verified | gdk:tga (230 files) | 9.38 | 9.18 | 9.52 / 8.91 | 426 | 15% / 29% NOISY | 72 (+4) | 1 |
| tga-verified-policy-strict | gdk:tga (230 files) | 9.99 | 9.65 | 10.08 / 9.24 | 400 | 8% / 18% NOISY | 72 (+4) | 0 |
| tga-verified-policy-recommended-beta | gdk:tga (230 files) | 9.79 | 9.52 | 9.91 / 9.46 | 409 | 4% / 10% | 71 (+4) | 0 |
| gdk:tga | gdk:tga (230 files) | 21.20 (2.26x) | 20.39 | 21.07 / 20.33 | 189 | 7% / 26% NOISY | 81 (+9) | 1 |
| tga-verified | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 8.69 | 8.55 | 8.81 / 8.50 | 460 | 3% / 8% | 70 (+4) | 0 |
| tga-verified-policy-gdk | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 9.47 | 9.47 | 9.65 / 9.28 | 422 | 5% / 29% NOISY | 69 (+4) | 1 |
| tga-verified-policy-imagecompat | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 9.61 | 9.26 | 9.73 / 9.34 | 416 | 14% / 22% NOISY | 69 (+4) | 0 |

### TGA large

files: 5, verified decodes 4; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): tga-image: incumbent-only-ok 1, both-ok-equal 4; tga-tinytga: both-error 1, both-ok-equal 2, verified-only-ok 2; gdk:tga: incumbent-only-ok 1, both-ok-equal 4

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-image | probe_tga24_8192x8192.tga | 2.47 | 2.32 | 2.47 / 2.30 | 1617 | 19% / 41% NOISY | 644 (+448) | 0 |
| gdk:tga | probe_tga24_8192x8192.tga | 18.54 | 18.30 | 18.67 / 18.23 | 216 | 6% / 16% NOISY | 585 (+385) | 0 |
| tga-verified | tga24_4096x4096.tga | 9.66 | 9.18 | 9.77 / 9.02 | 414 | 26% / 33% NOISY | 116 (+64) | 0 |
| tga-verified-policy-strict | tga24_4096x4096.tga | 10.01 | 9.89 | 10.26 / 9.96 | 399 | 6% / 11% NOISY | 117 (+64) | 0 |
| tga-verified-policy-recommended-beta | tga24_4096x4096.tga | 10.13 | 10.01 | 10.41 / 9.87 | 395 | 5% / 11% | 117 (+64) | 0 |
| tga-verified-policy-gdk | tga24_4096x4096.tga | 10.25 | 9.66 | 10.44 / 9.91 | 390 | 12% / 17% NOISY | 116 (+64) | 0 |
| tga-verified-policy-imagecompat | tga24_4096x4096.tga | 10.37 | 10.13 | 10.51 / 10.09 | 386 | 15% / 36% NOISY | 116 (+64) | 0 |
| tga-image | tga24_4096x4096.tga | 2.34 (0.24x) | 1.71 | 2.36 / 1.70 | 1706 | 42% / 42% NOISY | 164 (+112) | 0 |
| tga-tinytga | tga24_4096x4096.tga | 16.93 (1.75x) | 16.45 | 17.12 / 16.32 | 236 | 13% / 40% NOISY | 117 (+64) | 1 |
| gdk:tga | tga24_4096x4096.tga | 18.36 (1.90x) | 17.88 | 18.49 / 17.72 | 218 | 5% / 12% | 154 (+97) | 0 |
| tga-verified | tga24rle_4096x4096.tga | 6.44 | 6.20 | 6.55 / 6.12 | 621 | 33% / 62% NOISY | 84 (+57) | 0 |
| tga-verified-policy-strict | tga24rle_4096x4096.tga | 6.79 | 6.44 | 6.86 / 6.46 | 589 | 7% / 16% NOISY | 83 (+57) | 0 |
| tga-verified-policy-recommended-beta | tga24rle_4096x4096.tga | 6.44 | 6.44 | 6.65 / 6.43 | 621 | 9% / 31% NOISY | 84 (+57) | 1 |
| tga-verified-policy-gdk | tga24rle_4096x4096.tga | 6.79 | 6.32 | 6.87 / 6.32 | 589 | 28% / 42% NOISY | 83 (+57) | 1 |
| tga-verified-policy-imagecompat | tga24rle_4096x4096.tga | 6.56 | 6.44 | 6.65 / 6.39 | 610 | 6% / 14% NOISY | 84 (+58) | 0 |
| tga-image | tga24rle_4096x4096.tga | 4.95 (0.77x) | 4.89 | 5.02 / 4.80 | 809 | 16% / 34% NOISY | 132 (+105) | 0 |
| tga-tinytga | tga24rle_4096x4096.tga | 18.95 (2.94x) | 18.00 | 18.64 / 17.99 | 211 | 7% / 28% NOISY | 84 (+57) | 0 |
| gdk:tga | tga24rle_4096x4096.tga | 18.48 (2.87x) | 18.00 | 18.72 / 18.19 | 216 | 14% / 27% NOISY | 88 (+62) | 1 |
| tga-verified | tga32_4096x3840.tga | 9.79 | 9.54 | 9.72 / 9.53 | 409 | 24% / 30% NOISY | 124 (+60) | 0 |
| tga-verified-policy-strict | tga32_4096x3840.tga | 10.43 | 10.30 | 10.52 / 10.14 | 384 | 2% / 13% | 124 (+60) | 0 |
| tga-verified-policy-recommended-beta | tga32_4096x3840.tga | 10.94 | 10.68 | 11.27 / 10.32 | 366 | 18% / 28% NOISY | 125 (+60) | 0 |
| tga-verified-policy-gdk | tga32_4096x3840.tga | 10.55 | 10.05 | 10.61 / 10.18 | 379 | 32% / 40% NOISY | 124 (+60) | 0 |
| tga-verified-policy-imagecompat | tga32_4096x3840.tga | 10.30 | 10.17 | 10.39 / 10.19 | 388 | 10% / 15% NOISY | 125 (+60) | 0 |
| tga-image | tga32_4096x3840.tga | 1.53 (0.16x) | 1.20 | 1.51 / 1.21 | 2621 | 36% / 39% NOISY | 125 (+60) | 1 |
| gdk:tga | tga32_4096x3840.tga | 18.69 (1.91x) | 18.69 | 19.10 / 18.31 | 214 | 7% / 24% NOISY | 190 (+121) | 1 |
| tga-verified | tga32rle_4096x3840.tga | 6.36 | 6.10 | 6.40 / 6.08 | 629 | 104% / 111% NOISY | 84 (+57) | 1 |
| tga-verified-policy-strict | tga32rle_4096x3840.tga | 6.74 | 6.48 | 6.87 / 6.52 | 594 | 35% / 43% NOISY | 83 (+57) | 1 |
| tga-verified-policy-recommended-beta | tga32rle_4096x3840.tga | 6.61 | 6.48 | 6.78 / 6.43 | 605 | 18% / 24% NOISY | 83 (+57) | 1 |
| tga-verified-policy-gdk | tga32rle_4096x3840.tga | 6.61 | 6.48 | 6.73 / 6.49 | 605 | 6% / 12% NOISY | 83 (+57) | 1 |
| tga-verified-policy-imagecompat | tga32rle_4096x3840.tga | 6.74 | 6.61 | 6.86 / 6.52 | 594 | 15% / 32% NOISY | 83 (+57) | 1 |
| tga-image | tga32rle_4096x3840.tga | 4.20 (0.66x) | 4.01 | 4.17 / 3.94 | 953 | 33% / 45% NOISY | 84 (+57) | 1 |
| gdk:tga | tga32rle_4096x3840.tga | 18.95 (2.98x) | 18.82 | 19.18 / 18.73 | 211 | 3% / 11% | 108 (+80) | 0 |

### ICNS corpus

files: 15701, verified decodes 267; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): icns-crate: incumbent-only-ok 11519, both-error 3915, both-ok-DIFFERENT 174, verified-only-ok 50, both-ok-equal 43; gdk:icns: both-error 15247, incumbent-only-ok 187, both-ok-DIFFERENT 166, both-ok-equal 101

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns-crate (43 files) | 21.67 | 20.00 | 21.68 / 18.69 | 185 | 15% / 35% NOISY | 61 (+0) | 0 |
| icns-crate | icns-crate (43 files) | 11.62 (0.54x) | 9.57 | 11.92 / 9.32 | 344 | 30% / 35% NOISY | 61 (+0) | 1 |
| icns-verified | gdk:icns (101 files) | 17.08 | 16.36 | 17.21 / 16.11 | 234 | 22% / 27% NOISY | 61 (+0) | 0 |
| gdk:icns | gdk:icns (101 files) | 5.09 (0.30x) | 4.47 | 5.16 / 4.38 | 787 | 18% / 40% NOISY | 61 (+0) | 0 |

### ICNS large

files: 3, verified decodes 2; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): icns-crate: both-ok-equal 2, incumbent-only-ok 1; gdk:icns: both-ok-equal 2, both-error 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns_legacy128.icns | 23.30 | 22.29 | 23.54 / 22.06 | 172 | 18% / 23% NOISY | 27 (+0) | 0 |
| icns-crate | icns_legacy128.icns | 14.69 (0.63x) | 13.29 | 15.06 / 12.76 | 272 | 42% / 48% NOISY | 27 (+0) | 0 |
| gdk:icns | icns_legacy128.icns | 1.05 (0.04x) | 0.93 | 1.09 / 0.86 | 3827 | 62% / 70% NOISY | 27 (+0) | 0 |
| icns-verified | icns_legacy128_pad32M.icns | 19.82 | 17.44 | 19.58 / 16.37 | 202 | 18% / 33% NOISY | 37 (+0) | 0 |
| icns-crate | icns_legacy128_pad32M.icns | 373.39 (18.84x) | 315.95 | 347.45 / 304.33 | 11 | 43% / 57% NOISY | 69 (+32) | 0 |
| gdk:icns | icns_legacy128_pad32M.icns | 286.87 (14.48x) | 195.31 | 291.31 / 164.23 | 14 | 66% / 70% NOISY | 75 (+34) | 1 |
| icns-crate | icns_png1024.icns | 12.40 | 11.92 | 12.61 / 11.51 | 323 | 24% / 32% NOISY | 27 (+0) | 0 |

### ICO corpus

files: 433, verified decodes 286; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): ico-crate: both-ok-equal 283, incumbent-only-ok 105, both-error 42, verified-only-ok 2, both-ok-DIFFERENT 1; ico-image: both-ok-equal 250, both-ok-DIFFERENT 32, incumbent-only-ok 105, both-error 42, verified-only-ok 4; gdk:ico: both-ok-equal 275, incumbent-only-ok 51, both-error 96, verified-only-ok 3, both-ok-DIFFERENT 8

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico-crate (283 files) | 8.52 | 7.32 | 8.62 / 7.21 | 469 | 19% / 27% NOISY | 28 (+0) | 0 |
| ico-crate | ico-crate (283 files) | 9.97 (1.17x) | 8.88 | 10.53 / 8.89 | 401 | 31% / 33% NOISY | 28 (+0) | 1 |
| ico-verified | ico-image (250 files) | 7.14 | 6.27 | 7.28 / 6.22 | 560 | 17% / 24% NOISY | 28 (+0) | 0 |
| ico-image | ico-image (250 files) | 2.74 (0.38x) | 2.59 | 2.78 / 2.53 | 1461 | 10% / 20% NOISY | 28 (+0) | 1 |
| ico-verified | gdk:ico (275 files) | 7.40 | 7.13 | 7.38 / 7.13 | 541 | 5% / 18% NOISY | 28 (+0) | 0 |
| gdk:ico | gdk:ico (275 files) | 3.06 (0.41x) | 2.97 | 3.15 / 2.88 | 1307 | 18% / 26% NOISY | 28 (+0) | 1 |

### ICO large

files: 3, verified decodes 3; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): ico-crate: both-ok-equal 3; ico-image: both-ok-equal 3; gdk:ico: both-ok-equal 2, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico_dib256.ico | 6.68 | 6.48 | 6.80 / 6.40 | 599 | 6% / 12% NOISY | 27 (+0) | 0 |
| ico-crate | ico_dib256.ico | 15.26 (2.29x) | 13.22 | 15.55 / 12.66 | 262 | 21% / 28% NOISY | 27 (+0) | 1 |
| ico-image | ico_dib256.ico | 2.54 (0.38x) | 2.54 | 2.64 / 2.50 | 1573 | 24% / 37% NOISY | 27 (+0) | 1 |
| gdk:ico | ico_dib256.ico | 1.85 (0.28x) | 1.66 | 1.95 / 1.70 | 2159 | 18% / 20% NOISY | 27 (+0) | 0 |
| ico-verified | ico_multi6.ico | 6.63 | 6.26 | 6.84 / 5.85 | 603 | 18% / 28% NOISY | 27 (+0) | 0 |
| ico-crate | ico_multi6.ico | 14.13 (2.13x) | 14.13 | 14.61 / 13.77 | 283 | 14% / 28% NOISY | 27 (+0) | 0 |
| ico-image | ico_multi6.ico | 2.74 (0.41x) | 1.57 | 2.79 / 1.50 | 1461 | 100% / 66% NOISY | 27 (+0) | 1 |
| gdk:ico | ico_multi6.ico | 1.82 (0.27x) | 1.59 | 1.81 / 1.58 | 2204 | 29% / 28% NOISY | 27 (+0) | 1 |
| ico-verified | probe_ico_dib1024_dir256.ico | 6.83 | 6.68 | 6.94 / 6.72 | 585 | 5% / 10% | 27 (+0) | 0 |
| ico-crate | probe_ico_dib1024_dir256.ico | 8.11 (1.19x) | 7.63 | 8.26 / 7.27 | 493 | 10% / 19% NOISY | 27 (+0) | 0 |
| ico-image | probe_ico_dib1024_dir256.ico | 2.54 (0.37x) | 2.46 | 2.57 / 2.51 | 1573 | 10% / 12% NOISY | 27 (+0) | 0 |
