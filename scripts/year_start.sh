#!/bin/bash
NEW_YEAR=$1
echo "Starting a new year of AOC : ${NEW_YEAR}"

cargo new aoc-${NEW_YEAR}

mkdir -p aoc-${NEW_YEAR}/src/bin
mkdir -p aoc-${NEW_YEAR}/data/inputs

touch aoc-${NEW_YEAR}/src/bin/.nothing
touch aoc-${NEW_YEAR}/data/inputs/.nothing

cd aoc-${NEW_YEAR} && cargo add --path ../aoc-utils
