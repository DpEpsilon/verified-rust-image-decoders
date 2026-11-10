#!/usr/bin/env python3
"""Run every TIFF column on one existing file and print statuses, tolerance groups and pair metrics."""
import sys
from pathlib import Path
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.append(str(Path(__file__).resolve().parents[1] / 'jpeg'))
import harness
import jt_harness
jt_harness.reproduce(harness.AD, sys.argv)
