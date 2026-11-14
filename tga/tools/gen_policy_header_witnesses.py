#!/usr/bin/env python3
"""Append small named-profile header witnesses without duplicating core vectors."""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'tga_verus/src/model_vectors.rs'
MARK = '// ---- policy header acceptance witnesses (generated) ----'
choices = [
    ('Strict', 'strict_policy', 'depth-true15', None),
    ('RecommendedBeta', 'recommended_beta_policy', 'attr-32-count4', None),
    ('RecommendedBeta', 'recommended_beta_policy', 'map-absent-stray-depth', None),
    ('GdkCompat-2.42.12', 'gdk_compat_policy', 'map-flag2-truecolour', None),
    ('GdkCompat-2.42.12', 'gdk_compat_policy', 'map-present-length0-baddepth', None),
    ('GdkCompat-2.42.12', 'gdk_compat_policy', 'depth-true15', 'UnsupportedPixelDepth'),
    ('ImageCrateCompat-0.25.10', 'image_crate_compat_policy', 'map-flag2-truecolour', None),
    ('ImageCrateCompat-0.25.10', 'image_crate_compat_policy', 'attr-grey8-count8', None),
    ('ImageCrateCompat-0.25.10', 'image_crate_compat_policy', 'depth-index16-map24', None),
    ('ImageCrateCompat-0.25.10', 'image_crate_compat_policy', 'depth-index16-map15', 'UnsupportedPixelDepth'),
]
vectors = json.loads((ROOT / 'policy/POLICY-VECTORS.json').read_text())['profiles']
parts = [MARK]
for profile, constructor, name, error in choices:
    row = next(row for row in vectors[profile] if row['name'] == name)
    assert row['expected'].get('error') == error
    raw = bytes.fromhex(row['input_hex'])
    ident = constructor.removesuffix('_policy') + '_' + name.replace('-', '_')
    expr = f'crate::policy_model::mh_policy(policy_bytes_{ident}(), crate::policy::{constructor}())'
    claim = f'{expr} == Err(Error::{error})' if error else f'{expr} is Ok'
    parts.append(f'// {profile}: frozen vector {name}.\npub open spec fn policy_bytes_{ident}() -> Seq<u8> {{\n    seq![' + ', '.join(f'{b}u8' for b in raw) + ']\n}\n' + f'pub proof fn policy_header_{ident}()\n    ensures {claim},\n{{ assert({claim}) by (compute); }}\n')
s = SOURCE.read_text()
base = s[:s.index(MARK)] if MARK in s else s[:s.rindex('} // verus!')]
SOURCE.write_text(base + '\n'.join(parts) + '\n} // verus!\n')
