# Serpens Mortiferum
## What does the name mean?
Latin for deadly snake.
## What is it?
A Rust [Battlesnake](https://play.battlesnake.org) built on the [warpy_snake](https://github.com/Osrepnay/warpy-snake) template made by nosrep.

The battlesnake itself uses a hand backed algorithm based on the minimizing and maximizing of minimax, and the uneven tree search of Monte Carlo Tree Search.

1) Follow the highest score nodes until you reach a node that doesnt have explored children
2) Then run an eval function on each of those nodes' states
3) based on whether your minimizing or maxing, then choose the one with the highest or lowest score respectively. 
4) Back propogate that score by using a numerically stable running average

This allows for good exploration and exploitation, which is key when you look at algorithms like MCTS.
The static evaluation function maximizes:
 - Mobility / Area Control (Using the vornoi pattern to determine area control.)
 - 10/Number of Snakes
 - \# of owned food pieces.
