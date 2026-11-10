#!/usr/bin/env python3
"""Run one file through every pnm decoder: python3 -I reproduce.py FILE"""
import sys
sys.dont_write_bytecode = True
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
import harness
harness.common.reproduce(harness, sys.argv[1])
