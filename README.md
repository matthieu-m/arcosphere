A solver for Arcosphere conversion path for Factorio: Space Exploration extension.

#   Goals & Non-Goals

##  Goals

-   Generic: a generic API to compute the transformation path from one set of arcospheres to another.
-   Minimization of catalysts: the API privileges minimizing the number of catalysts (see nomenclature) over computing
    a shorter path, so as to minimize the overall number of arcospheres necessary.


#   Solutions

If you do not care for running the calculations yourself, please find the recipe per recipe minimal transformation paths
in the `etc/` directory.

If you do wish to run the calculations yourself, please check the `Run` section below.

You'll want to read the nomenclature below to understand them.


#   Nomenclature

##  Arcospheres

The arcospheres are named after greeks letters. This is cute, and very physic-y, but not easy to type or read for the
uninitiated. This library documentation instead uses the first letter of the greek letter name to identify the
arcospheres:

-   ε -> [E]psilon.
-   γ -> [G]amma.
-   λ -> [L]ambda.
-   ω -> [O]mega.
-   φ -> [P]hi.
-   θ -> [T]heta.
-   ξ -> [X]i.
-   ζ -> [Z]eta.


##   Polarity

One property of the arcospheres is their polarity. That is, when looking at the transformations offered, namely
_inversion_ and _folding_ one can see a property emerging which we'll call polarity:

-   Inversion: ELPX into GOTZ or GOTZ into ELPX.
-   Folding: 8 different transformations taking one each of ELPX & GOTZ and returning one each of ELPX and GOTZ.

Thus, we divide the arcospheres into 2 groups:

-   Negative polarity: ELPX.
-   Positive polarity: GOTZ.

and note that inversion transform 4 negative into 4 positive (or vice-versa) while folding is always polarity
preserving.

Based on polarity, we can thus quickly confirm that only folding is insufficient for a given transformation path, as
well as quickly infer the minimum number of inversions required.


##  Catalysts

Due the limited number of transformations, it is not always possible to find a transformation path with just the
arcospheres mentioned. For example, going from EP to LX starting with only EP is simply not possible:

-   The polarity is good, and inversions require 4 arcospheres anyway.
-   There is no folding recipe taking EP.

On the other hand, it is possible to go from EP+O to LX+O via folding:

1.  Executing EO -> LG moves us from EP+O to LP+G.
2.  Executing PG -> XO moves us from LP+G to LX+O.

Note that the additional [O]mega arcosphere is returned by the end of the transformation chain. This is an essential
property of a catalyst: it is not consumed by the transformation.


#   Run

You can run the binary with `cargo run [ARGS]`, for example.

The binary has 3 subcommands:

-   `solve`: prints the shortest paths which allow transforming SOURCE into TARGET.
-   `verify`: verifies that a given path is valid, that is, can actually be executed, or point where the problem is.
-   `plan`: prints the plan for how to execute a path.

See the sub-sections for more.


##  Solve

### Raw solve

By default, the solve subcommand takes two arguments -- SOURCE and TARGET -- each a set of arcospheres, and returns the
list of paths which transform SOURCE into TARGET, or an error if no such path can be found.

There are options to sort the output, they can be found in the help.

Example, recovering the spheres from one of the output of Macroscale Entanglement Data:

```sh
$ cargo run solve LGZ LOT
GLZ -> LOT + P  =>  GP -> OX |  XZ -> PT
GLZ -> LOT + X  =>  XZ -> PT |  GP -> OX
```

This outputs two paths, let's focus on the first:

```text
GLZ -> LOT + P  =>              GP -> OX |  XZ -> PT
^          ^~~ catalysts        ^        ^  ^~~~~~~~ second recipe to execute
\~~~~~~~~~ source & target      \        \ stage separator, the first recipe must be executed before the second
                                 \~~~~~~~ first recipe to execute
```

The second output is a wee bit more complicated, let's focus on the first recipe, and the elements of the path that
were not present in the above.

```text
OTX -> LOT x4 + EG  =>  EO -> GL // GX -> LZ |  GX -> LZ // XZ -> PT |  PZ -> EG |  ET -> OP // GX -> LZ |  PZ -> EG
           ^                     ^~ parallel separator, the recipes on either side can be executed concurrently
           \
            \~ the SOURCE to TARGET transformation can only be executed by transforming x4 the SOURCE into x4 the target
```


### Planning solve

On larger solutions, picking the path to use and laying down the gravimetrics can be a tad challenging. To help, the
solver subcommand can be passed the `--plan` option (`-p` for short) and output the execution plan for each path.

Coming back to the previous examples:

```sh
$ cargo run solve -p LGZ LOT
GLZ -> LOT + P  =>  GP -> OX |  XZ -> PT
 1.  [Z] + [GP] + [L] | GP -> OX
 2.  [] + [XZ] + [LO] | XZ -> PT

GLZ -> LOT + X  =>  XZ -> PT |  GP -> OX
 1.  [G] + [XZ] + [L] | XZ -> PT
 2.  [] + [GP] + [LT] | GP -> OX
```

The paths are printed as before, however below each path its execution plan is also printed. See the `Plan` subcommand
for an explanation of how to read this plan.


##  Verify

The verify subcommand takes one argument: a PATH, in the same format that the solve subcommand returns.

The PATH must be a single argument -- ie, it must be quoted.


##  Plan

The plan subcommand takes one argument: a PATH, in the same format that the solve subcommand returns.

The PATH must a single argument -- ie, it must be quoted.

```sh
$ cargo run plan "GLZ -> LOT + P  =>  GP -> OX |  XZ -> PT"
 1.  [Z] + [GP] + [L] | GP -> OX
 2.  [] + [XZ] + [LO] | XZ -> PT
```

The output is _one_ possible serie of stages which allows executing this plan. Each line is composed of:

```text
 1.  [Z] + [GP] + [L] | GP -> OX
 ^   ^     ^      ^   ^ the separator, followed by the recipes of the stage
  \   \     \      \
   \   \     \      \ the arcospheres which will no longer be necessary, and can already be returned
    \   \     \
     \   \     \  the arcospheres which will be used by this stage
      \   \
       \   \ the arcospheres which will be used by later stages
        \
         \ the index of the stage
```

The recipes of any given stage can be executed concurrently, and thus will be separated by `//`.

For example, on a more complex path:

```sh
$ cargo run plan 'OTX -> LOT x4 + EG  =>  EO -> GL // GX -> LZ |  GX -> LZ // XZ -> PT |  PZ -> EG |  ET -> OP // GX -> LZ |  PZ -> EG'`
 1.  [XXX] + [EGOX] + [OOOTTTT] | EO -> GL // GX -> LZ
 2.  [X] + [GXXZ] + [LLOOOTTTT] | GX -> LZ // XZ -> PT
 3.  [TX] + [PZ] + [LLLOOOTTTT] | PZ -> EG
 4.  [] + [EGTX] + [LLLOOOTTTT] | ET -> OP // GX -> LZ
 5.  [] + [PZ] + [LLLLOOOOTTTT] | PZ -> EG
```
