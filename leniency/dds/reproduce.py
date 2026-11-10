#!/usr/bin/env python3
"""Run FILE(s) through every dds decoder column and print one JSON line each: python3 -I reproduce.py FILE..."""
import sys
from pathlib import Path
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
import harness, leniency_core as core
core.reproduce(harness.DdsConfig(), sys.argv[1:])
