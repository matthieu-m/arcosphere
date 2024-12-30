A solver for Arcosphere conversion path for Factorio: Space Exploration extension.

#   Goals & Non-Goals

##  Goals

-   Generic: a generic API to compute the transformation path from one set of arcospheres to another.
-   Minimization of catalysts: the API privileges minimizing the number of catalysts (see nomenclature) over computing
    a shorter path, so as to minimize the overall number of arcospheres necessary.


#   Solutions

If you do not care for running the calculations yourself, please find the recipe per recipe minimal transformation paths
in the `etc/` directory.

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
