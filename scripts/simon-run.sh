#!/usr/bin/env bash

{ time ./data/generated/mandelbrot.c.exe ; } 2>> ./data/logs/mandelbrot.c.time.out
{ time ./data/generated/mandelbrot.rs.exe ; } 2>> ./data/logs/mandelbrot.rs.time.out
