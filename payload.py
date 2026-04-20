#!/usr/bin/env python3
"""
payload.py -- Generate OTA payload dumps for analysis
Usage: python3 payload.py target_files.zip
"""
import zipfile, os, sys

if len(sys.argv) < 2:
    print("Usage: python3 payload.py target_files.zip")
    sys.exit(1)

try:
    with zipfile.ZipFile(sys.argv[1], 'r') as z:
        print("Contents of OTA payload:")
        for name in z.namelist()[:20]:
            info = z.getinfo(name)
            print(f"  {name:<50} {info.file_size:>10} bytes")
        if len(z.namelist()) > 20:
            print(f"  ... and {len(z.namelist())-20} more files")
except Exception as e:
    print(f"Error: {e}")
