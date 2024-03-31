from distutils.core import setup
import py2exe
from random import choice
import sys

setup(
    options = {'py2exe': {'bundle_files': 1, 'compressed': True}},
    console = [{'script': "soysay.py"}],
    zipfile = None,
)
