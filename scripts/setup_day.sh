#!/bin/bash
LIB_PATH="./src/lib.rs"
TMP_LIB_PATH="./src/tmp"

read -p "Which day do you want to setup? " DAY

echo "pub mod day$DAY;" >> $LIB_PATH
sort $LIB_PATH | uniq >> $TMP_LIB_PATH
mv $TMP_LIB_PATH $LIB_PATH

cp -n ./src/day1.rs ./src/day$DAY.rs

cp -n ./tests/day1_test.rs ./tests/day$DAY\_test.rs

# You need to have 1password cli installed on your computer
eval $(op signin my)
PASS_NAME="AoC Session Header"
SESSION=$(op get item "$PASS_NAME" | jq '.details.password')

wget --header="Cookie: session=${SESSION//\"}" \
  -O ./src/input_files/day$DAY.txt \
  https://adventofcode.com/2020/day/$DAY/input