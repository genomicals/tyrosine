# Tyrosine
![Tyrosine Mascot](https://github.com/genomicals/tyrosine/blob/master/assets/mascot.png)
Mascot by [@Nauxe](https://github.com/nauxe)

## Overview
NEAT machine learning library for Rust.

NeuroEvolution of Augmenting Topologies (NEAT) is a genetic algorithm which
evolves a population of neural networks. The population is split into
several genetically-distinct species. NEAT aims to find an optimal set of
weights in addition to an optimal neural network structure.

Link to original paper <a href="https://nn.cs.utexas.edu/downloads/papers/stanley.cec02.pdf" target="_blank">here</a>.

## Implementation Specifics
There are several instances where specific features are up to the implementer's choice.
Here are some choices we made:
- A bias node is included in the inputs
- Cyclic connections will be disallowed
- Species won't be removed for becoming stale, only when allocated reproductive slots reach 0

## Trivia
Tyrosine is an amino acid. The word comes from the Greek "tyros", which means
cheese.

## License
This work is dual-licensed under GPL Version 2 and GPL Version 3. You may
choose between these licenses on a case-by-case basis.

