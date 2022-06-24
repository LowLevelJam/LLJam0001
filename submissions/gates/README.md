# Gates
## Team
1. Nate Catelli

## Introduction
Gates is an attempt to build a digital-logic simulator out of highly impractical abstractions. Primarily, using docker and http as the individual gate and connection analogs.

To facilitate building these logical circuits, docker-compose is used to chart the relationship of all gates and their interconnection as well as manage the creation, lifecycle and cleanup of the simulation. Each gate being a small http service.

https://github.com/ncatelli/gates/tree/v0.1.0

The largest problem with gates is that the ratio of boilerplate to code is extremely high when using yaml and docker-compose. To assist in building a simulation, I've also included a Descriptor language compiler to generate the compose files using a minimal DSL.

https://github.com/ncatelli/gates-hdl/tree/v0.1.0

Additionally this includes a web frontend so users will not be required to install any additional tools to try the generator.

### Demo
https://ncatelli.github.io/agents/

## Running Locally
The instructions for building each project can be found at their corresponding links.

### Dependencies
To run the gates simulations:
- docker 19.03.0+
- docker-compose latest

#### HDL
To build the HDL:
- rust 1.56+

##### Web Fronted
To build the web frontend, you will additionally need
- wasm-pack
- npm 16.x

## Ideas and Inspirations
I wanted to play with an idea of elevating an extremely low-level concept to an absurd level of abstraction. I considered both beam or http services, mostly wanting to keep networked IPC as the "bus" between gates. Finally settling on docker as the medium to abstract each gate.

## Next Steps
If I had more time I would love to

- Add more safety to the go service
  - Retries and backoffs to the http client outputter
- Add more non-gate services
  - Signals
    - Clocks
  - Simpler user inputs
  - Additional outputs
- Spend more time refining the web front end
  - It's very rough and ugly
  - I'd improve the errors, which currently only log to console

## Example Programs
### Half Adder

```
DEFINE sum AS xor;
DEFINE carry AS and;
```

### Full Adder 

```
DEFINE first_sum AS xor;
DEFINE first_carry AS and;
DEFINE second_sum AS xor;
DEFINE second_carry AS and;
DEFINE output AS or;
LINK first_sum -> b OF second_sum;
LINK first_sum -> a OF second_carry;
LINK first_carry -> b OF output;
```

## Referenced Links and Credits

- Frontend and rendering code heavily referenced examples from [wasm_game_of_life](https://github.com/rustwasm/wasm_game_of_life/).
