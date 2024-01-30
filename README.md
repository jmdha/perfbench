# perfbench
A [perft](https://www.chessprogramming.org/Perft) benchmark tool.

## Table of Contents
1. [Purpose](#Purpose)
2. [Usage](#Usage)
3. [Example](#Example)

## Purpose
This tool is intended to measure the performance of move generation and state update in a chess engine or library. 
To do this it measures the average time to complete the perft command for various positions and depths. 

It can be used compare different chess engines/libraries, however, this comparison would be somewhat meaningless. 
Most chess engines calculate things doing state update needed for their evaluation and search, which simply adds overhead during Perft. 
Moreover, most engines aren't optimised for Perft, as it is simply a debugging tool.

As such, this tool is intended to measure performance gain/loss during optimization of a specific engine/library.

Inspired by the great tool [perftree](https://github.com/agausmann/perftree).

## Usage
perfbench requires a list of engines/libraries as input. Each has to invoke perft with these arguments:

```
./some_engine "depth" "fen"
```

Most engines do not have a specific executable for completing perft, as such they often require a custom implementation or script.
An example is shown [here](https://github.com/jamadaha/perfbench/tree/main/examples/engine_scripts) for how Stockfish perft speed can be measured.  

Seen below is perfbench --help
```
Usage: perfbench [OPTIONS] <ENGINES>...

Arguments:
  <ENGINES>...  

Options:
  -t, --test-suite <TEST_SUITE>  An optional custom test_suite
  -i, --iterations <ITERATIONS>  Default number of iterations to run for each test case [default: 10]
  -h, --help                     Print help
  -V, --version                  Print version

```
## Example
This shows a comparison of perft speed between [JankChess](https://github.com/jamadaha/JankChess) and [Stockfish](https://github.com/official-stockfish/Stockfish) v 14.1.
```
perfbench -i 50 JankChess/build/Perft perfbench/examples/engine_scripts/Stockfish.exp 
Loading test suite...
Loading engines...
	Loading "JankChess/build/Perft"...
	Loading "perfbench/examples/engine_scripts/Stockfish.exp"...
Running perft...
Perft     Stockfish fen
127       120       rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
129       113       r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 
24        91        8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 
465       182       r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1
67        96        rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8
111       104       r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 1

```

