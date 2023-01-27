@echo off

set filename=wallpaper

g++ "%filename%".cpp -luuid -lole32 -municode -O2 -s -o "%filename%".exe
