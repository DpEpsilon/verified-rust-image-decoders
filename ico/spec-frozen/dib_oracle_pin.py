"""Frozen DIB 1.1 oracle identity shared by the ICO checker and fixture builder (retaken at the DIB 1.1 freeze: unchanged).

Never pin against an independently changing working-copy oracle without checking it.
"""
import hashlib
from pathlib import Path

ORACLE_PATH = Path('/home/ws/projects/dib/spec-1.1/check_vectors.py')
ORACLE_SHA256 = '214dbf12cd63e8967e1af401297cff5ac16922653d969998e7addc0d2bdd5488'

def verified_path():
    actual = hashlib.sha256(ORACLE_PATH.read_bytes()).hexdigest()
    if actual != ORACLE_SHA256:
        raise RuntimeError(f'DIB oracle digest mismatch: {actual}, expected {ORACLE_SHA256}')
    return ORACLE_PATH
