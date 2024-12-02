# Advent Of Code 2024

## Overview
This repository contains my solutions for the Advent Of Code 2024 challenges

## Structure

- `./inputs/` is a directory for puzzle inputs. It is intentionally not versioned to
    comply with the challenge runners requests to not publish the puzzle inputs

- `./src/input_fetcher` contains a little helper code for pulling down puzzle inputs
    if they aren't already in the inputs directory. 

    If they are already in the directory the runner will just use the data in the file

- `./src/main` contains some CLI argument parsing code and logic for solution orchesration

- `./solutions/*` contains solutions for each day of the challenge. 

    They all implement the `Solution` trait to provide a common interface for the driver code

## Priorities

This year I'm mostly trying to prioritize maintaining my free time :)
We'll see how that goes.
