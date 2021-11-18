#!/bin/bash

git add .
git commit -m "Use trackid for detecting ads, code restructure"
git push

cd AUR
makepkg --printsrcinfo > .SRCINFO
git add .
git commit -m "Update AUR"
git push 