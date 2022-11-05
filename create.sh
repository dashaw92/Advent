#!/bin/bash
if [ "$#" -lt 2 ] 
then
    echo "Usage: ./create <year> <day>"
    exit
fi

year="${1}"
day="${2}"

if [ ! -d "$year" ]
then
    echo "Year $year does not exist yet."
    if [ "$#" -ge 3 ] && [ "$3" = "force" ]
    then
        echo "...but force flag was found. Creating!"
        mkdir "$year"
    else
        exit
    fi
fi

if [ -d "$year/day$day" ]
then
    echo "$year/day$day project already exists!"
    exit
fi

cd "$year"
echo "Creating day$day cargo --bin project..."
cargo new --bin "day$day"
cd ..
echo "Writing template to main.rs"
cat "template.rs" > "$year/day$day/src/main.rs"
cd "$year/day$day"
echo "Adding dependency to aoc in Cargo.toml..."
cargo add --path ../../aoc
echo "Done."
