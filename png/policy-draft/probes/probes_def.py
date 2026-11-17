"""Probe list used by validate.py, run_probes.py, heldout_def.py and icc_probes.py.

PROBES = the tracked list in probes_public.py, plus the entries of EXTRA from the optional,
git-ignored module probes_crash_private.py (inputs that crash the png crate). Each EXTRA entry is
(insert_before_name, name, bytes, question); it is inserted before the probe named insert_before_name.
Without the private module, PROBES is the public list.
"""
from probes_public import *  # noqa: F401,F403
from probes_public import PROBES as _PUBLIC, RAMPB, RAMPD  # noqa: F401

try:
    from probes_crash_private import EXTRA
except ImportError:
    EXTRA = []

PROBES = list(_PUBLIC)
for _before, _name, _b, _why in EXTRA:
    PROBES.insert([n for n, _, _ in PROBES].index(_before), (_name, _b, _why))
