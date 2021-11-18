#!/bin/bash

git add .
git commit -m "fixed potential trackid error"
git push

cd AUR
makepkg --printsrcinfo > .SRCINFO
git add .
git commit -m "Update AUR"
git push 