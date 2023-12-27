#!/bin/bash

sudo apt-get install -y imagemagick

if [ $# -eq 0 ]; then
    echo "Specify image file path"
    exit 1
fi

for i in {1..10..1}; do
    cargo run $1 $i $i
done

for i in {20..100..10}; do
    cargo run $1 $i $i
done

convert -reverse -delay 25 -loop 0 pixie_*.jpg pixie.gif
