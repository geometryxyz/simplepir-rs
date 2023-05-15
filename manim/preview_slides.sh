#!/bin/bash

cd "$(dirname "$0")"

source venv/bin/activate
manim -ql simplepir.py SimplePIR && manim-slides SimplePIR
