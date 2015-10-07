@echo off

set filename=wallpaper

gcc "%filename%".c -municode -O2 -s -o "%filename%".exe
