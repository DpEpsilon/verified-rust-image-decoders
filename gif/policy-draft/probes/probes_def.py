"""Probe list used by validate.py and run_probes.py.

PROBES = the tracked list in probes_public.py, plus the entries of EXTRA from the optional,
git-ignored module probes_crash_private.py (inputs that crash a third-party decoder). Each EXTRA entry is
(insert_before_name, name, bytes, question); it is inserted before the probe named insert_before_name.
Without the private module, PROBES is the public list.
"""
from probes_public import *  # noqa: F401,F403
from probes_public import PROBES as _PUBLIC  # noqa: F401

try:
    from probes_crash_private import EXTRA
except ImportError:
    EXTRA = []

PROBES = list(_PUBLIC)
for _before, _name, _b, _why in EXTRA:
    PROBES.insert([n for n, _, _ in PROBES].index(_before), (_name, _b, _why))
