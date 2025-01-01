In the context of default Space Exploration, here are the shortest paths to "undo" a recipe.

#   Nomenclature

Please refer to the README for the nomenclature.

For ease of reading, the recipes present the negative & positive polarities in separate groups, while the solve paths
present the catalysts in a separate group.

In some cases, some recipes may be executed in any order. Such recipes are grouped with [].


#   Minimum number of Arcospheres

Attempting to isolate each recipe requires an absolute minimum of 98 arcospheres, with a very specific distribution.


#   Non-Science

##  Naquium Tesseract

The recipe itself:

    LX + Z -> EP + T | P + GO

Of note: the second alternative exhibits flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    //  39 other variants, see naquium_tesseract_ept.txt
    EPT x4 + E  =>  ET -> PO | EO -> LG | PG -> XO | EO -> LG | PG -> XO | EO -> LG | PG -> XO | EO -> LG | LT -> EZ | LT -> EZ | PG -> XO | EO -> LG | PG -> XO | LT -> EZ | EO -> LG | XG -> LZ

    //  438 other variants, see naquium_tesseract_gop.txt
    GOP x4 + E  =>  EO -> LG | LO -> XT | XG -> LZ | LO -> XT | GOTZ -> ELPX | LT -> EZ | PG -> XO | PG -> XO | EO -> LG | EO -> LG | XG -> LZ | XG -> LZ | PZ -> EG | PG -> XO | EO -> LG | XG -> LZ | PG -> XO | LO -> XT | LT -> EZ

A minimum of 13 arcospheres is required (any catalyst will do), or 14 if picking different catalysts.


##  Naquium Processor

The recipe itself:

    EP + GOTZ -> LLLLLX | LXXXXX

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    //  92 other variants, see naquium_processor_lllllx.txt
    LLLLLX x2 + O   =>  LO -> XT | LT -> EZ | XZ -> PT | ELPX -> GOTZ | LT -> EZ | LO -> XT | ET -> PO | LO -> XT | PG -> XO | LT -> EZ | LT -> EZ | XZ -> PT | XZ -> PT | XZ -> PT | PZ -> EG | ELPX -> GOTZ | ET -> PO | LT -> EZ

    //  92 other variants, see naquium_processor_lxxxxx.txt
    LXXXXX x2 + G   =>  XG -> LZ | XZ -> PT | LT -> EZ | ELPX -> GOTZ | LT -> EZ | XG -> LZ | EO -> LG | XG -> LZ | XZ -> PT | LT -> EZ | XZ -> PT | LT -> EZ | XZ -> PT | XZ -> PT | ET -> PO | ELPX -> GOTZ | PZ -> EG | PZ -> EG

A minimum of 13 arcospheres is required, with T or Z as catalyst.


#   Advanced Science II

##  Macroscale Entanglement Data

The recipe itself:

    L + OT -> L + GZ | X + OT

Solve paths:

    LGZ + P =>  PG -> XO | XZ -> PT
    LGZ + X =>  XZ -> PT | PG -> XO

    XOT x4 + EG =>  XG -> LZ   ET -> PO   PZ -> EG   EO -> LG   XG -> LZ   XZ -> PT   XG -> LZ   PZ -> EG
    XOT x4 + EG =>  XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ   XZ -> PT   EO -> LG   XG -> LZ   PZ -> EG
    XOT x4 + EG =>  XG -> LZ   XZ -> PT   EO -> LG   XG -> LZ   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG
    XOT x4 + EP =>  EO -> LG   XG -> LZ   PZ -> EG   ET -> PO   XG -> LZ   PZ -> EG   XG -> LZ   XZ -> PT
    XOT x4 + EP =>  EO -> LG   XG -> LZ   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ   XZ -> PT
    XOT x4 + EZ =>  EO -> LG   XG -> LZ   XZ -> PT   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ
    XOT x4 + EZ =>  ET -> PO   PZ -> EG   EO -> LG   XG -> LZ   XZ -> PT   XG -> LZ   PZ -> EG   XG -> LZ
    XOT x4 + EZ =>  ET -> PO   PZ -> EG   XG -> LZ   XZ -> PT   EO -> LG   XG -> LZ   PZ -> EG   XG -> LZ
    XOT x4 + GG =>  XG -> LZ   XG -> LZ   XZ -> PT   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   EO -> LG
    XOT x4 + PG =>  XG -> LZ   PZ -> EG   XG -> LZ   EO -> LG   XZ -> PT   XG -> LZ   PZ -> EG   ET -> PO
    XOT x4 + PG =>  XG -> LZ   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ   XZ -> PT   EO -> LG
    XOT x4 + PG =>  XG -> LZ   PZ -> EG   XG -> LZ   XZ -> PT   EO -> LG   XG -> LZ   PZ -> EG   ET -> PO
    XOT x4 + GZ =>  XG -> LZ   XZ -> PT   PZ -> EG   ET -> PO   XG -> LZ   PZ -> EG   XG -> LZ   EO -> LG
    XOT x4 + GZ =>  XG -> LZ   XZ -> PT   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ   EO -> LG
    XOT x4 + PZ =>  PZ -> EG   XG -> LZ   EO -> LG   XG -> LZ   XZ -> PT   PZ -> EG   XG -> LZ   ET -> PO
    XOT x4 + PZ =>  PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   EO -> LG   XG -> LZ   XZ -> PT   XG -> LZ
    XOT x4 + PZ =>  PZ -> EG   XG -> LZ   XZ -> PT   EO -> LG   XG -> LZ   PZ -> EG   XG -> LZ   ET -> PO
    XOT x4 + ZZ =>  XZ -> PT   PZ -> EG   XG -> LZ   ET -> PO   PZ -> EG   XG -> LZ   EO -> LG   XG -> LZ

A minimum of 14 arcospheres is required, with PG or PZ as catalysts.


##  Singularity Application Data

The recipe itself:

    E + OT -> E + GZ | P + OT

Solve paths:

    EGZ + P =>  PG -> XO | XZ -> PT
    EGZ + X =>  XZ -> PT | PG -> XO

    POT x4 + GG =>  PG -> XO | XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | LT -> EZ | PZ -> EG
    POT x4 + LG =>  LO -> XT | XG -> LZ | LT -> EZ | PZ -> EG | PG -> XO | PZ -> EG | XG -> LZ | PZ -> EG
    POT x4 + LG =>  LO -> XT | XG -> LZ | PZ -> EG | PG -> XO | LT -> EZ | PZ -> EG | XG -> LZ | PZ -> EG
    POT x4 + LG =>  LT -> EZ | PG -> XO | PZ -> EG | XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG
    POT x4 + XG =>  XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | LT -> EZ | PZ -> EG | PG -> XO
    POT x4 + XG =>  XG -> LZ | LT -> EZ | PZ -> EG | PZ -> EG | PG -> XO | XG -> LZ | PZ -> EG | LO -> XT
    POT x4 + XG =>  XG -> LZ | PZ -> EG | PG -> XO | LT -> EZ | PZ -> EG | XG -> LZ | PZ -> EG | LO -> XT
    POT x4 + GZ =>  PG -> XO | PZ -> EG | XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | LT -> EZ
    POT x4 + GZ =>  PG -> XO | PZ -> EG | XG -> LZ | PZ -> EG | LO -> XT | XG -> LZ | PZ -> EG | LT -> EZ
    POT x4 + LX =>  LT -> EZ | PZ -> EG | XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | PG -> XO
    POT x4 + LX =>  LT -> EZ | PZ -> EG | XG -> LZ | PZ -> EG | LO -> XT | XG -> LZ | PZ -> EG | PG -> XO
    POT x4 + LZ =>  LO -> XT | PZ -> EG | XG -> LZ | LT -> EZ | PZ -> EG | PZ -> EG | PG -> XO | XG -> LZ
    POT x4 + LZ =>  LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | LT -> EZ | PZ -> EG | PG -> XO | XG -> LZ
    POT x4 + LZ =>  PZ -> EG | PG -> XO | LT -> EZ | PZ -> EG | XG -> LZ | PZ -> EG | LO -> XT | XG -> LZ
    POT x4 + XZ =>  PZ -> EG | XG -> LZ | LO -> XT | PZ -> EG | XG -> LZ | PZ -> EG | PG -> XO | LT -> EZ
    POT x4 + XZ =>  PZ -> EG | XG -> LZ | LT -> EZ | PZ -> EG | PZ -> EG | PG -> XO | XG -> LZ | LO -> XT
    POT x4 + XZ =>  PZ -> EG | XG -> LZ | PZ -> EG | PG -> XO | LT -> EZ | PZ -> EG | XG -> LZ | LO -> XT
    POT x4 + ZZ =>  PZ -> EG | PG -> XO | PZ -> EG | XG -> LZ | PZ -> EG | LO -> XT | XG -> LZ | LT -> EZ

A minimum of 14 arcospheres is required, with XG or XZ as catalysts.


##  Timespace Manipulation Data

The recipe itself:

    EL + O -> EL + G | PX + O

Solve paths:

    ELG x4 + PP =>  PG -> XO | PG -> XO | XG -> LZ | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | ET -> PO
    ELG x4 + PT =>  PG -> XO | ET -> PO | PG -> XO | XG -> LZ | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT
    ELG x4 + PT =>  LT -> EZ | PG -> XO | XZ -> PT | ET -> PO | PG -> XO | XG -> LZ | PG -> XO | XZ -> PT
    ELG x4 + PT =>  LT -> EZ | PG -> XO | XZ -> PT | PG -> XO | XG -> LZ | ET -> PO | PG -> XO | XZ -> PT
    ELG x4 + PX =>  PG -> XO | XG -> LZ | XZ -> PT | PG -> XO | LT -> EZ | XZ -> PT | PG -> XO | ET -> PO
    ELG x4 + PX =>  PG -> XO | XG -> LZ | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | PG -> XO | ET -> PO
    ELG x4 + PZ =>  PG -> XO | XZ -> PT | ET -> PO | PG -> XO | XG -> LZ | PG -> XO | XZ -> PT | LT -> EZ
    ELG x4 + PZ =>  PG -> XO | XZ -> PT | PG -> XO | XG -> LZ | ET -> PO | PG -> XO | XZ -> PT | LT -> EZ
    ELG x4 + PZ =>  PG -> XO | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | ET -> PO | PG -> XO | XG -> LZ
    ELG x4 + XT =>  XG -> LZ | ET -> PO | PG -> XO | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | PG -> XO
    ELG x4 + XT =>  LT -> EZ | XZ -> PT | PG -> XO | ET -> PO | XG -> LZ | PG -> XO | XZ -> PT | PG -> XO
    ELG x4 + XT =>  LT -> EZ | XZ -> PT | PG -> XO | XG -> LZ | ET -> PO | PG -> XO | XZ -> PT | PG -> XO
    ELG x4 + TZ =>  ET -> PO | PG -> XO | XZ -> PT | PG -> XO | LT -> EZ | XZ -> PT | PG -> XO | XG -> LZ
    ELG x4 + TZ =>  ET -> PO | PG -> XO | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | PG -> XO | XG -> LZ
    ELG x4 + XX =>  XG -> LZ | XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | ET -> PO | PG -> XO | PG -> XO
    ELG x4 + XZ =>  XZ -> PT | PG -> XO | ET -> PO | PG -> XO | XG -> LZ | XZ -> PT | LT -> EZ | PG -> XO
    ELG x4 + XZ =>  XZ -> PT | PG -> XO | ET -> PO | XG -> LZ | PG -> XO | XZ -> PT | LT -> EZ | PG -> XO
    ELG x4 + XZ =>  XZ -> PT | LT -> EZ | PG -> XO | XZ -> PT | ET -> PO | PG -> XO | XG -> LZ | PG -> XO

    PXO + G =>  XG -> LX | PZ -> EG
    PXO + Z =>  PZ -> EG | XG -> LZ

A minimum of 14 arcospheres is required, with PZ or XZ as catalysts.


#   Deep Space Science III

##  Space Dilation Data

The recipe itself:

    OZ -> LL | PP

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    LL x2 + PG  =>  PG -> XO | LO -> XT | LT -> EZ | XZ -> PT | ELPX -> GOTZ | LT -> EZ | ET -> PO
    LL x2 + XO  =>  LO -> XT | LT -> EZ | XZ -> PT | ELPX -> GOTZ | LT -> EZ | EZ -> PO | PG -> XO
    LL x2 + XO  =>  LO -> XT | LT -> EZ | XZ -> PT | LT -> EZ | ELPX -> GOTZ | ET -> PO | PG -> XO
    LL x2 + XT  =>  LT -> EZ | XZ -> PT | ET -> PO | LO -> XT | LT -> EZ | ELPX -> GOTZ | PG -> XO

    PP x2 + EG  =>  PG -> XO | EO -> LG | XG -> LZ | PZ -> EG | PG -> XO | ELPX -> GOTZ | LT -> EZ
    PP x2 + EZ  =>  PZ -> EG | PG -> XO | EO -> LG | PG -> XO | ELPX -> GOTZ | XG -> LZ | LT -> EZ
    PP x2 + EZ  =>  PZ -> EG | PG -> XO | EO -> LG | ELPX -> GOTZ | PG -> XO | XG -> LZ | LT -> EZ
    PP x2 + LT  =>  LT -> EZ | PZ -> EG | PG -> XO | EO -> LG | PG -> XO | ELPX -> GOTZ | XG -> LZ

A minimum of 7 arcospheres is required, with EPG or LXT as catalysts.


##  Space Folding Data

The recipe itself:

    LX -> EP | TZ

Of note: the second alternative exhibits flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    EP + G  =>  PG -> XO | EO -> LG
    EP + O  =>  EO -> LG | PG -> XO

    TZ x2 + EG  =>  ET -> PO | GOTZ -> ELPX | PZ -> EG | PG -> XO | EO -> LG
    TZ x2 + EO  =>  ET -> PO | PZ -> EG | EO -> LG | GOTZ -> ELPX | PG -> XO
    TZ x2 + EO  =>  ET -> PO | PZ -> EG | GOTZ -> ELPX | EO -> LG | PG -> XO
    TZ x2 + EP  =>  ET -> PO | PZ -> EG | EO -> LG | PG -> XO | GOTZ -> ELPX
    TZ x2 + EP  =>  ET -> PO | PZ -> EG | PG -> XO | EO -> LG | GOTZ -> ELPX
    TZ x2 + GO  =>  GOTZ -> ELPX | PZ -> EG | ET -> PO | PG -> XO | EO -> LG
    TZ x2 + PG  =>  PZ -> EG | ET -> PO | GOTZ -> ELPX | PG -> XO | EO -> LG
    TZ x2 + PG  =>  PZ -> EG | ET -> PO | PG -> XO | GOTZ -> ELPX | EO -> LG
    TZ x2 + PO  =>  PZ -> EG | ET -> PO | GOTZ -> ELPX | EO -> LG | PG -> XO

A minimum of 6 arcospheres is required, with EP or any of [EP]+[GO] as catalysts.


##  Space Injection Data

The recipe itself:

    GT -> ZZ | EE

Of note: the second alternative exhibits flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    ZZ x2 + X   =>  XZ -> PT | PZ -> EG | ET -> PO | PG -> XO | XZ -> PT | PZ -> EG | EO -> LG | LO -> XT

    EE x2 + PO  =>  EO -> LG | PG -> XO | LO -> XT | ET -> PO | EO -> LG | ELPX -> GOTZ | XZ -> PT
    EE x2 + PT  =>  ET -> PO | EO -> LG | PG -> XO | ELPX -> GOTZ | EO -> LG | LO -> XT | XZ -> PT
    EE x2 + PT  =>  ET -> PO | EO -> LG | PG -> XO | EO -> LG | ELPX -> GOTZ | LO -> XT | XZ -> PT
    EE x2 + XZ  =>  XZ -> PT | ET -> PO | EO -> LG | PG -> XO | EO -> LG | ELPX -> GOTZ | LO -> XT

A minimum of 6 arcospheres is required, with XZ as catalysts.


##  Space Warping Data

The recipe itself:

    EP -> TZ | GO

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    TZ x2 + EG  =>  ET -> PO | GOTZ -> ELPX | XZ -> PT | LT -> EZ | PZ -> EG
    TZ x2 + ET  =>  ET -> PO | PZ -> EG | GOTZ -> ELPX | LT -> EZ | XZ -> PT
    TZ x2 + EZ  =>  ET -> PO | PZ -> EG | GOTZ -> ELPX | XZ -> PT | LT -> EZ
    TZ x2 + GO  =>  GOTZ -> ELPX | PZ -> EG | LT -> EZ | XZ -> PT | ET -> PO
    TZ x2 + GO  =>  GOTZ -> ELPX | XZ -> PT | ET -> PO | LT -> EZ | PZ -> EG
    TZ x2 + GO  =>  GOTZ -> ELPX | XZ -> PT | LT -> EZ | ET -> PO | PZ -> EG
    TZ x2 + LT  =>  LT -> EZ | ET -> PO | PZ -> EG | GOTZ -> ELPX | XZ -> PT
    TZ x2 + LX  =>  LT -> EZ | ET -> PO | PZ -> EG | XZ -> PT | GOTZ -> ELPX
    TZ x2 + LX  =>  LT -> EZ | XZ -> PT | ET -> PO | PZ -> EG | GOTZ -> ELPX
    TZ x2 + LX  =>  LT -> EZ | XZ -> PT | PZ -> EG | ET -> PO | GOTZ -> ELPX
    TZ x2 + LX  =>  XZ -> PT | PZ -> EG | ET -> PO | LT -> EZ | GOTZ -> ELPX
    TZ x2 + PO  =>  PZ -> EG | GOTZ -> ELPX | LT -> EZ | XZ -> PT | ET -> PO
    TZ x2 + PT  =>  PZ -> EG | ET -> PO | GOTZ -> ELPX | LT -> EZ | XZ -> PT
    TZ x2 + PZ  =>  PZ -> EG | ET -> PO | GOTZ -> ELPX | XZ -> PT | LT -> EZ
    TZ x2 + XZ  =>  XZ -> PT | PZ -> EG | ET -> PO | GOTZ -> ELPX | LT -> EZ

    GO x2 + LT  =>  LO -> XT | XG -> LZ | GOTZ -> ELPX | LT -> EZ | XZ -> PT
    GO x2 + LT  =>  LO -> XT | XG -> LZ | LT -> EZ | GOTZ -> ELPX | XZ -> PT
    GO x2 + LX  =>  LO -> XT | XG -> LZ | LT -> EZ | XZ -> PT | GOTZ -> ELPX
    GO x2 + LX  =>  LO -> XT | XG -> LZ | XZ -> PT | LT -> EZ | GOTZ -> ELPX
    GO x2 + LZ  =>  LO -> XT | GOTZ -> ELPX | XG -> LZ | XZ -> PT | LT -> EZ
    GO x2 + XT  =>  XG -> LZ | GOTZ -> ELPX | LO -> XT | LT -> EZ | XZ -> PT
    GO x2 + TZ  =>  GOTZ -> ELPX | LO -> XT | XG -> LZ | XZ -> PT | LT -> EZ
    GO x2 + XZ  =>  XG -> LZ | LO -> XT | GOTZ -> ELPX | XZ -> PT | LT -> EZ
    GO x2 + XZ  =>  XG -> LZ | LO -> XT | XZ -> PT | GOTZ -> ELPX | LT -> EZ

A minimum of 6 arcospheres is required, with LT, LX, or XZ as catalysts.


#   Deep Space Science IV

##  Wormhole Data

The recipe itself:

    EL + GZ -> PX + OT

Of note: no alternative in this one.

Solve paths:

    PXOT + E    =>  EO -> LG | LT -> EZ | XG -> LZ | PZ -> EG
    PXOT + E    =>  EO -> LG | XZ -> LZ | PZ -> EG | LT -> EZ
    PXOT + G    =>  XG -> LZ | PZ -> EG | EO -> LG | LT -> EZ
    PXOT + G    =>  XG -> LZ | LT -> EZ | EO -> LG | PZ -> EG
    PXOT + L    =>  LT -> EZ | EO -> LG | PZ -> EG | XG -> LZ
    PXOT + L    =>  LT -> EZ | PZ -> EG | EO -> LG | XG -> LZ
    PXOT + Z    =>  PZ -> EG | XG -> LZ | EO -> LG | LT -> EZ
    PXOT + Z    =>  PZ -> EG | EO -> LG | LT -> EZ | XG -> LZ

Exactly 5 arcospheres are required, with any of E, G, L, or Z as catalysts.
