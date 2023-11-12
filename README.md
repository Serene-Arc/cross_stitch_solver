# cross_stitch_solver

This is a simple tool that solves a sequence of stitches on a grid, just
like regular cross stitch. It finds the most efficient way to do the
stitches, with efficiency being the amount of thread used. Basically, the
tool finds the lowest distance that connects all of the points.

Beware! At the moment, this tool uses a brute force find to get the
shortest path. This is very close to the 'travelling salesman problem' and
the number of options expands factorially e.g. ten points results in $10!$
possible combinations, each of which are tried. The number of possible
combinations is actually per half-stitch, so 15 stitches would result in
$30!$ possible sequences. The solver is multithreaded but this could still
take a LONG time, depending on how many stitches and cores your machine
has.

## Commands and Arguments

There are only two commands: `solve` and `visualise`. Both take their
input from STDIN and both have the option `-o` or `--output-file` to
specify where they will put the output.

`solve` will generate a sequence of stitches in a CSV file that shows the
solution. `visualise` takes this same sequence and uses it to generate
a gif of the solution, showing you how it's solved.

## Things to Implement

There are specific heuristics that can be implemented in the future. For
instance, there is only one most-efficient way to do a column from the
top, or from the bottom. These would make the solving a lot faster if it
was implemented to solve these sub-problems first.

## Solutions

There are a couple folders of solutions and the generated gifs that can be
looked at! The solutions are [here](./stitch_sequences/solved_sequences/).

