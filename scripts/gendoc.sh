#!/bin/bash
#
# apt install plantuml pandoc pandoc-plantuml-filter texlive-xetex texlive-luatex

filename=$1

if [ ! "x$filename" == "x" ]; then
pandoc --filter pandoc-plantuml -f markdown --pdf-engine=xelatex \
    -V mainfont='JetBrainsMonoNL Nerd Font Propo' \
    -V geometry:portrait \
    -V geometry:margin=1in \
    $filename -o ${filename%.md}.pdf 
fi
    #--number-sections \
    #-V documentclass=report \
    #-V geometry:landscape \
