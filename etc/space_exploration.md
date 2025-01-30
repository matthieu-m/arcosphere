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

    EPT -> LXZ x4 + G  =>  ET -> PO // PG -> XO |  EO -> LG // EO -> LG |  LT -> EZ // LT -> EZ // PG -> XO // PG -> XO |  EO -> LG // EO -> LG |  LT -> EZ // PG -> XO // PG -> XO |  EO -> LG // EO -> LG |  XG -> LZ
    EPT -> LXZ x4 + O  =>  EO -> LG // ET -> PO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // PG -> XO |  EO -> LG // XG -> LZ |  PG -> XO

    GOP -> LXZ x4 + E  =>  EO -> LG // PG -> XO // PG -> XO // PG -> XO |  LO -> XT // XG -> LZ // XG -> LZ |  LO -> XT // LT -> EZ // PZ -> EG |  GOTZ -> ELPX // EO -> LG // EO -> LG |  LO -> XT // PG -> XO // XG -> LZ |  EO -> LG // LT -> EZ |  XG -> LZ
    GOP -> LXZ x4 + G  =>  PG -> XO // PG -> XO |  XG -> LZ // XG -> LZ |  LO -> XT // LO -> XT // PZ -> EG |  GOTZ -> ELPX // EO -> LG // XG -> LZ |  EO -> LG // LO -> XT // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // XG -> LZ
    GOP -> LXZ x4 + X  =>  PG -> XO // PG -> XO // XG -> LZ |  LO -> XT // PZ -> EG // XG -> LZ |  GOTZ -> ELPX // EO -> LG // LO -> XT |  EO -> LG // LO -> XT // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // XG -> LZ |  XG -> LZ

A minimum of 13 arcospheres is required (any catalyst will do), or 14 if picking different catalysts.

Editor's picks:

    EPT -> LXZ x4 + O  =>  EO -> LG // ET -> PO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // PG -> XO |  EO -> LG // XG -> LZ |  PG -> XO
    1.  [EEPPPPTTT] + [EEOT] + [] | EO -> LG // ET -> PO
    2.  [EPPPPTT] + [EGLOPT] + [] | EO -> LG // LT -> EZ // PG -> XO
    3.  [EPPPT] + [EGLOPT] + [XZ] | EO -> LG // LT -> EZ // PG -> XO
    4.  [EPP] + [EGLOPT] + [XXZZ] | EO -> LG // LT -> EZ // PG -> XO
    5.  [EP] + [EGOP] + [LXXXZZZ] | EO -> LG // PG -> XO
    6.  [P] + [EGOX] + [LLXXXZZZ] | EO -> LG // XG -> LZ
    7.  [] + [PG] + [LLLLXXXZZZZ] | PG -> XO

    GOP -> LXZ x4 + E  =>  EO -> LG // PG -> XO // PG -> XO // PG -> XO |  LO -> XT // XG -> LZ // XG -> LZ |  LO -> XT // LT -> EZ // PZ -> EG |  GOTZ -> ELPX // EO -> LG // EO -> LG |  LO -> XT // PG -> XO // XG -> LZ |  EO -> LG // LT -> EZ |  XG -> LZ
    1.  [GOOOP] + [EGGGOPPP] + [] | EO -> LG // PG -> XO // PG -> XO // PG -> XO
    2.  [OOOOOP] + [GGLOXX] + [X] | LO -> XT // XG -> LZ // XG -> LZ
    3.  [OOOO] + [LLOPTZ] + [XXZ] | LO -> XT // LT -> EZ // PZ -> EG
    4.  [O] + [EEGOOOTZ] + [XXXZ] | GOTZ -> ELPX // EO -> LG // EO -> LG
    5.  [E] + [GGLOPX] + [LLXXXZ] | LO -> XT // PG -> XO // XG -> LZ
    6.  [X] + [ELOT] + [LLXXXXZZ] | EO -> LG // LT -> EZ
    7.  [] + [XG] + [ELLLXXXXZZZ] | XG -> LZ

The first alternative's path returns 2 out of 3 target arcospheres from its second stage, so that supplementing the
input with one L allows immediately running the tesseract recipe again in parallel with the 3rd stage.

Similarly, the second alternative's path returns 2 out of 3 target arcospheres from its second stage, so that
supplementing the input wiht one (other) L allows running the tesseract recipe again in parallel with the 3rd stage.

With a 2nd catalyst, they can be compressed a bit:

    EPT -> LXZ x4 + GO  =>  EO -> LG // ET -> PO // PG -> XO |  EO -> LG // EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // PG -> XO // XG -> LZ |  EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // PG -> XO
    1.  [EEPPPTTT] + [EEGOPT] + [] | EO -> LG // ET -> PO // PG -> XO
    2.  [PPPTT] + [EEGLOOPT] + [X] | EO -> LG // EO -> LG // LT -> EZ // PG -> XO
    3.  [PPT] + [EGGLOPTX] + [LXZ] | EO -> LG // LT -> EZ // PG -> XO // XG -> LZ
    4.  [P] + [EGLOPT] + [LLXXZZZ] | EO -> LG // LT -> EZ // PG -> XO
    5.  [] + [EGOP] + [LLLXXXZZZZ] | EO -> LG // PG -> XO

    GOP -> LXZ x4 + EG  =>  EO -> LG // PG -> XO // PG -> XO // PG -> XO |  LO -> XT // XG -> LZ // XG -> LZ |  GOTZ -> ELPX // LO -> XT // LO -> XT // PZ -> EG |  EO -> LG // EO -> LG // LT -> EZ // PG -> XO |  EO -> LG // LT -> EZ // XG -> LZ // XG -> LZ
    1.  [GGOOOP] + [EGGGOPPP] + [] | EO -> LG // PG -> XO // PG -> XO // PG -> XO
    2.  [GOOOOOP] + [GGLOXX] + [X] | LO -> XT // XG -> LZ // XG -> LZ
    3.  [OO] + [GLLOOOPTZZ] + [XX] | GOTZ -> ELPX // LO -> XT // LO -> XT // PZ -> EG
    4.  [XT] + [EEGLOOPT] + [XXXX] | EO -> LG // EO -> LG // LT -> EZ // PG -> XO
    5.  [] + [EGGLOTXX] + [LXXXXZ] | EO -> LG // LT -> EZ // XG -> LZ // XG -> LZ

The first alternative, gaining for having a full target set returned by the second stage, while the second alternative
instead loses a bit latency-wise, with only a single L returned by the fourth stage.


##  Naquium Processor

The recipe itself:

    EP + GOTZ -> LLLLLX | LXXXXX

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    LLLLLX -> EPGOTZ x2 + O  =>  LO -> XT |  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO // LO -> XT // XZ -> PT |  LO -> XT // LT -> EZ // LT -> EZ // PG -> XO // XZ -> PT |  ELPX -> GOTZ // ET -> PO // LT -> EZ // PZ -> EG // XZ -> PT
    LLLLLX -> EPGOTZ x2 + T  =>  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO // LO -> XT |  LO -> XT // LT -> EZ // PG -> XO // XZ -> PT |  ELPX -> GOTZ // LO -> XT // LT -> EZ // XZ -> PT |  ET -> PO // LT -> EZ // PZ -> EG // XZ -> PT
    LLLLLX -> EPGOTZ x2 + T  =>  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO // LO -> XT |  LO -> XT // LT -> EZ // PG -> XO // XZ -> PT |  ET -> PO // LO -> XT // LT -> EZ // XZ -> PT // XZ -> PT |  ELPX -> GOTZ // LT -> EZ // PZ -> EG

    LXXXXX -> EPGOTZ x2 + G  =>  XG -> LZ |  XZ -> PT |  LT -> EZ |  ELPX -> GOTZ // XZ -> PT |  LT -> EZ // PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT // XZ -> PT |  ELPX -> GOTZ // ET -> PO // LT -> EZ // PZ -> EG // XZ -> PT
    LXXXXX -> EPGOTZ x2 + Z  =>  XZ -> PT |  LT -> EZ |  ELPX -> GOTZ // XZ -> PT |  PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT |  ELPX -> GOTZ // LT -> EZ // XG -> LZ // XZ -> PT // XZ -> PT |  ET -> PO // LT -> EZ // PZ -> EG
    LXXXXX -> EPGOTZ x2 + Z  =>  XZ -> PT |  LT -> EZ |  ELPX -> GOTZ // XZ -> PT |  PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT |  ET -> PO // LT -> EZ // PZ -> EG // XG -> LZ // XZ -> PT |  ELPX -> GOTZ // LT -> EZ // XZ -> PT
    LXXXXX -> EPGOTZ x2 + Z  =>  XZ -> PT |  LT -> EZ |  ELPX -> GOTZ // XZ -> PT |  PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT |  ET -> PO // LT -> EZ // XG -> LZ // XZ -> PT // XZ -> PT |  ELPX -> GOTZ // LT -> EZ // PZ -> EG

A minimum of 13 arcospheres is required, with T or Z as catalyst.

Editor's picks:

    LLLLLX -> EPGOTZ x2 + T  =>  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO // LO -> XT |  LO -> XT // LT -> EZ // PG -> XO // XZ -> PT |  ELPX -> GOTZ // LO -> XT // LT -> EZ // XZ -> PT |  ET -> PO // LT -> EZ // PZ -> EG // XZ -> PT
    1.  [LLLLLLLLLXX] + [LT] + [] | LT -> EZ
    2.  [ELLLLLLLLLX] + [XZ] + [] | XZ -> PT
    3.  [LLLLLLL] + [ELLPTX] + [] | ELPX -> GOTZ // LT -> EZ
    4.  [GLLLLLLZ] + [ELOT] + [Z] | ET -> PO // LO -> XT
    5.  [LLLL] + [GLLOPTXZ] + [Z] | LO -> XT // LT -> EZ // PG -> XO // XZ -> PT
    6.  [L] + [ELLLOPTXXZ] + [TZ] | ELPX -> GOTZ // LO -> XT // LT -> EZ // XZ -> PT
    7.  [] + [ELPTTXZZ] + [GOTTZ] | ET -> PO // LT -> EZ // PZ -> EG // XZ -> PT

    LXXXXX -> EPGOTZ x2 + Z  =>  XZ -> PT |  LT -> EZ |  ELPX -> GOTZ // XZ -> PT |  PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT |  ELPX -> GOTZ // LT -> EZ // XG -> LZ // XZ -> PT // XZ -> PT |  ET -> PO // LT -> EZ // PZ -> EG
    1.  [LLXXXXXXXXX] + [XZ] + [] | XZ -> PT
    2.  [LPXXXXXXXXX] + [LT] + [] | LT -> EZ
    3.  [XXXXXXX] + [ELPXXZ] + [] | ELPX -> GOTZ // XZ -> PT
    4.  [OTXXXXXX] + [GPXZ] + [T] | PZ -> EG // XG -> LZ
    5.  [XXXX] + [EGLOTXXZ] + [T] | EO -> LG // LT -> EZ // XG -> LZ // XZ -> PT
    6.  [] + [EGLLPTXXXXZZ] + [T] | ELPX -> GOTZ // LT -> EZ // XG -> LZ // XZ -> PT // XZ -> PT
    7.  [] + [ELPTTZ] + [GOPTTZZ] | ET -> PO // LT -> EZ // PZ -> EG

Not much in the way of early returns, but all other paths are even worse so...

With a 2nd catalyst, they can be compressed a bit:

    LLLLLX -> EGOPTZ x2 + EP  =>  ELPX -> GOTZ |  LO -> XT // LT -> EZ // XZ -> PT |  ET -> PO // LT -> EZ // PG -> XO // XZ -> PT |  ELPX -> GOTZ // LO -> XT // LO -> XT // LT -> EZ // PZ -> EG |  ET -> PO // LT -> EZ // LT -> EZ // XZ -> PT // XZ -> PT
    1.  [LLLLLLLLLX] + [ELPX] + [] | ELPX -> GOTZ
    2.  [GLLLLLLL] + [LLOTXZ] + [] | LO -> XT // LT -> EZ // XZ -> PT
    3.  [LLLLLL] + [EGLPTTXZ] + [] | ET -> PO // LT -> EZ // PG -> XO // XZ -> PT
    4.  [LL] + [ELLLLOOPPTXZ] + [] | ELPX -> GOTZ // LO -> XT // LO -> XT // LT -> EZ // PZ -> EG
    5.  [] + [ELLTTTXXZZ] + [EGGO] | ET -> PO // LT -> EZ // LT -> EZ // XZ -> PT // XZ -> PT

    LXXXXX -> EGOPTZ x2 + EP  =>  ELPX -> GOTZ |  LT -> EZ // XG -> LZ // XZ -> PT |  EO -> LG // LT -> EZ // PZ -> EG // XZ -> PT |  ELPX -> GOTZ // ET -> PO // XG -> LZ // XG -> LZ // XZ -> PT |  LT -> EZ // LT -> EZ // PZ -> EG // XZ -> PT // XZ -> PT
    1.  [LXXXXXXXXX] + [ELPX] + [] | ELPX -> GOTZ
    2.  [OXXXXXXX] + [GLTXXZ] + [] | LT -> EZ // XG -> LZ // XZ -> PT
    3.  [XXXXXX] + [ELOPTXZZ] + [] | EO -> LG // LT -> EZ // PZ -> EG // XZ -> PT
    4.  [XX] + [EEGGLPTXXXXZ] + [] | ELPX -> GOTZ // ET -> PO // XG -> LZ // XG -> LZ // XZ -> PT
    5.  [] + [LLPTTXXZZZ] + [GOOP] | LT -> EZ // LT -> EZ // PZ -> EG // XZ -> PT // XZ -> PT

No early returns, but still better latencies than the single catalyst paths, and a similar number of gravimetrics
facilities.


#   Advanced Science II

##  Macroscale Entanglement Data

The recipe itself:

    L + OT -> L + GZ | X + OT

Solve paths:

    LGZ -> LOT + P =>  PG -> XO | XZ -> PT
    LGZ -> LOT + X =>  XZ -> PT | PG -> XO

    XOT -> LOT x4 + EG  =>  EO -> LG // XG -> LZ |  XG -> LZ // XZ -> PT |  PZ -> EG |  ET -> PO // XG -> LZ |  PZ -> EG
    XOT -> LOT x4 + EG  =>  ET -> PO // XG -> LZ |  PZ -> EG |  EO -> LG // XG -> LZ |  XG -> LZ // XZ -> PT |  PZ -> EG
    XOT -> LOT x4 + GZ  =>  XG -> LZ // XZ -> PT |  PZ -> EG |  ET -> PO // XG -> LZ |  PZ -> EG |  EO -> LG // XG -> LZ
    XOT -> LOT x4 + PZ  =>  PZ -> EG |  EO -> LG // XG -> LZ |  XG -> LZ // XZ -> PT |  PZ -> EG |  ET -> PO // XG -> LZ
    XOT -> LOT x4 + PZ  =>  PZ -> EG |  ET -> PO // XG -> LZ |  PZ -> EG |  EO -> LG // XG -> LZ |  XG -> LZ // XZ -> PT

A minimum of 14 arcospheres is required, with PG or PZ as catalysts.

Editor's pick:

    XOT -> LOT x4 + EG  =>  EO -> LG // XG -> LZ |  XG -> LZ // XZ -> PT |  PZ -> EG |  ET -> PO // XG -> LZ |  PZ -> EG
    1.  [XXX] + [EGOX] + [OOOTTTT] | EO -> LG // XG -> LZ
    2.  [X] + [GXXZ] + [LLOOOTTTT] | XG -> LZ // XZ -> PT
    3.  [XT] + [PZ] + [LLLOOOTTTT] | PZ -> EG
    4.  [] + [EGTX] + [LLLOOOTTTT] | ET -> PO // XG -> LZ
    5.  [] + [PZ] + [LLLLOOOOTTTT] | PZ -> EG

This is the one path for the second alternative which returns 2 Ls from the first stage.

With a 3rd catalyst, the editor's pick can be compressed a bit:

    OTX -> LOT x4 + EEG  =>  EO -> LG // ET -> PO // XG -> LZ |  PZ -> EG // XG -> LZ |  XG -> LZ // XZ -> PT |  PZ -> EG
    1.  [XXX] + [EEGOTX] + [OOOTTT] | EO -> LG // ET -> PO // XG -> LZ
    2.  [XX] + [GPXZ] + [LLOOOOTTT] | PZ -> EG // XG -> LZ
    3.  [] + [GXXZ] + [ELLLOOOOTTT] | XG -> LZ // XZ -> PT
    4.  [] + [PZ] + [ELLLLOOOOTTTT] | PZ -> EG

With a 2nd catalyst, the first alternative can be compressed to a single stage:

    GLZ -> LOT + PX  =>  PG -> XO // XZ -> PT


##  Singularity Application Data

The recipe itself:

    E + OT -> E + GZ | P + OT

Solve paths:

    EGZ -> EOT + P =>  PG -> XO | XZ -> PT
    EGZ -> EOT + X =>  XZ -> PT | PG -> XO

    POT -> EOT x4 + XG  =>  XG -> LZ |  LO -> XT // PZ -> EG |  XG -> LZ |  LT -> EZ // PZ -> EG |  PG -> XO // PZ -> EG
    POT -> EOT x4 + XG  =>  XG -> LZ |  LT -> EZ // PZ -> EG |  PG -> XO // PZ -> EG |  XG -> LZ |  LO -> XT // PZ -> EG
    POT -> EOT x4 + GZ  =>  PG -> XO // PZ -> EG |  XG -> LZ |  LO -> XT // PZ -> EG |  XG -> LZ |  LT -> EZ // PZ -> EG
    POT -> EOT x4 + LZ  =>  LO -> XT // PZ -> EG |  XG -> LZ |  LT -> EZ // PZ -> EG |  PG -> XO // PZ -> EG |  XG -> LZ
    POT -> EOT x4 + LZ  =>  LT -> EZ // PZ -> EG |  PG -> XO // PZ -> EG |  XG -> LZ |  LO -> XT // PZ -> EG |  XG -> LZ

A minimum of 14 arcospheres is required, with XG or XZ as catalysts.

Editor's pick:

    POT -> EOT x4 + LZ  =>  LT -> EZ // PZ -> EG |  PG -> XO // PZ -> EG |  XG -> LZ |  LO -> XT // PZ -> EG |  XG -> LZ
    1.  [PPP] + [LPTZ] + [OOOOTTT] | LT -> EZ // PZ -> EG
    2.  [P] + [GPPZ] + [EEOOOOTTT] | PG -> XO // PZ -> EG
    3.  [PO] + [XG] + [EEEOOOOTTT] | XG -> LZ
    4.  [] + [LOPZ] + [EEEOOOOTTT] | LO -> XT // PZ -> EG
    5.  [] + [XG] + [EEEEOOOOTTTT] | XG -> LZ

This is the one path for the second alternative which returns 2 Es from the first stage.

With a 3rd catalyst, the editor's pick can be compressed a bit:

    OPT -> EOT x4 + GLZ  =>  LT -> EZ // PG -> XO // PZ -> EG |  PZ -> EG // XG -> LZ |  LO -> XT // PZ -> EG |  XG -> LZ
    1.  [PP] + [GLPPTZ] + [OOOOTTT] | LT -> EZ // PG -> XO // PZ -> EG
    2.  [PO] + [GPXZ] + [EEOOOOTTT] | PZ -> EG // XG -> LZ
    3.  [] + [LOPZ] + [EEEGOOOOTTT] | LO -> XT // PZ -> EG
    4.  [] + [XG] + [EEEEGOOOOTTTT] | XG -> LZ

With a 2nd catalyst, the first alternative can be compressed to a single stage:

    EGZ -> EOT + PX  =>  PG -> XO // XZ -> PT


##  Timespace Manipulation Data

The recipe itself:

    EL + O -> EL + G | PX + O

Solve paths:

    ELG -> ELO x4 + PT  =>  ET -> PO // PG -> XO |  PG -> XO // XG -> LZ |  XZ -> PT |  LT -> EZ // PG -> XO |  XZ -> PT
    ELG -> ELO x4 + PT  =>  LT -> EZ // PG -> XO |  XZ -> PT |  ET -> PO // PG -> XO |  PG -> XO // XG -> LZ |  XZ -> PT
    ELG -> ELO x4 + PX  =>  PG -> XO // XG -> LZ |  XZ -> PT |  LT -> EZ // PG -> XO |  XZ -> PT |  ET -> PO // PG -> XO
    ELG -> ELO x4 + XZ  =>  XZ -> PT |  ET -> PO // PG -> XO |  PG -> XO // XG -> LZ |  XZ -> PT |  LT -> EZ // PG -> XO
    ELG -> ELO x4 + XZ  =>  XZ -> PT |  LT -> EZ // PG -> XO |  XZ -> PT |  ET -> PO // PG -> XO |  PG -> XO // XG -> LZ

    PXO -> ELO + G =>  XG -> LX | PZ -> EG
    PXO -> ELO + Z =>  PZ -> EG | XG -> LZ

A minimum of 14 arcospheres is required, with PZ or XZ as catalysts.

Editor's pick:

    ELG -> ELO x4 + PT  =>  ET -> PO // PG -> XO |  PG -> XO // XG -> LZ |  XZ -> PT |  LT -> EZ // PG -> XO |  XZ -> PT
    1.  [GGG] + [EGPT] + [EEELLLL] | ET -> PO // PG -> XO
    2.  [G] + [GGPX] + [EEELLLLOO] | PG -> XO // XG -> LZ
    3.  [LG] + [XZ] + [EEELLLLOOO] | XZ -> PT
    4.  [] + [GLPT] + [EEELLLLOOO] | LT -> EZ // PG -> XO
    5.  [] + [XZ] + [EEEELLLLOOOO] | XZ -> PT

This is the one path from the first alternative which returns 2 Os from the first stage.

With a 3rd catalyst, the editor's pick can be compressed a bit:

    EGL -> ELO x4 + PTT  =>  ET -> PO // LT -> EZ // PG -> XO |  PG -> XO // XZ -> PT |  PG -> XO // XG -> LZ |  XZ -> PT
    1.  [GGG] + [EGLPTT] + [EEELLL] | ET -> PO // LT -> EZ // PG -> XO
    2.  [GG] + [GPXZ] + [EEEELLLOO] | PG -> XO // XZ -> PT
    3.  [] + [GGPX] + [EEEELLLOOOT] | PG -> XO // XG -> LZ
    4.  [] + [XZ] + [EEEELLLLOOOOT] | XZ -> PT

With a 2nd catalyst, the second alternative can be compressed to a single stage:

    PXO -> ELO + GZ  =>  PZ -> EG // XG -> LZ


#   Deep Space Science III

##  Space Dilation Data

The recipe itself:

    OZ -> LL | PP

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    LL -> OZ x2 + PG  =>  PG -> XO |  LO -> XT |  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO
    LL -> OZ x2 + XO  =>  LO -> XT |  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO |  PG -> XO

    PP -> OZ x2 + EZ  =>  PZ -> EG |  PG -> XO |  EO -> LG |  ELPX -> GOTZ // PG -> XO |  XG -> LZ |  LT -> EZ
    PP -> OZ x2 + LT  =>  LT -> EZ |  PZ -> EG |  PG -> XO |  EO -> LG |  ELPX -> GOTZ // PG -> XO |  XG -> LZ

A minimum of 7 arcospheres is required, with EPG or LXT as catalysts.

Editor's picks:

    LL -> OZ x2 + XO  =>  LO -> XT |  LT -> EZ |  XZ -> PT |  ELPX -> GOTZ // LT -> EZ |  ET -> PO |  PG -> XO
    1.  [LLLX] + [LO] + [] | LO -> XT
    2.  [LLXX] + [LT] + [] | LT -> EZ
    3.  [ELLX] + [XZ] + [] | XZ -> PT
    4.  [] + [ELLPTX] + [] | ELPX -> GOTZ // LT -> EZ
    5.  [G] + [ET] + [OZZ] | ET -> PO
    6.  [] + [PG] + [OOZZ] | PG -> XO

    PP -> OZ x2 + EZ  =>  PZ -> EG |  PG -> XO |  EO -> LG |  ELPX -> GOTZ // PG -> XO |  XG -> LZ |  LT -> EZ
    1.  [EPPP] + [PZ] + [] | PZ -> EG
    2.  [EEPP] + [PG] + [] | PG -> XO
    3.  [EPPX] + [EO] + [] | EO -> LG
    4.  [] + [EGLPPX] + [] | ELPX -> GOTZ // PG -> XO
    5.  [T] + [XG] + [OOZ] | XG -> LZ
    6.  [] + [LT] + [OOZZ] | LT -> EZ

Those are the paths which return something (even if incomplete) from the fourth stage.

With a 3rd catalyst, it's possible to solve them in only 4 stages, such as with:

    LL -> OZ x2 + EPX  =>  ELPX -> GOTZ |  LO -> XT // LT -> EZ |  ET -> PO // XZ -> PT |  LT -> EZ // PG -> XO
    1.  [LLL] + [ELPX] + [] | ELPX -> GOTZ
    2.  [LG] + [LLOT] + [Z] | LO -> XT // LT -> EZ
    3.  [LG] + [ETXZ] + [Z] | ET -> PO // XZ -> PT
    4.  [] + [GLPT] + [OPZ] | LT -> EZ // PG -> XO

    PP -> OZ x2 + ELX  =>  ELPX -> GOTZ |  PG -> XO // PZ -> EG |  EO -> LG // PG -> XO |  LT -> EZ // XG -> LZ
    1.  [PPP] + [ELPX] + [] | ELPX -> GOTZ
    2.  [PT] + [GPPZ] + [O] | PG -> XO // PZ -> EG
    3.  [T] + [EGOP] + [XO] | EO -> LG // PG -> XO
    4.  [] + [GLTX] + [OOX] | LT -> EZ // XG -> LZ

And as a bonus, one arcosphere of the target set is returned immediately by the first stage.


##  Space Folding Data

The recipe itself:

    LX -> EP | TZ

Of note: the second alternative exhibits flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    EP -> LX + G  =>  PG -> XO | EO -> LG
    EP -> LX + O  =>  EO -> LG | PG -> XO

    TZ -> LX x2 + EP  =>  ET -> PO // PZ -> EG |  EO -> LG // PG -> XO |  GOTZ -> ELPX
    TZ -> LX x2 + GO  =>  GOTZ -> ELPX |  ET -> PO // PZ -> EG |  EO -> LG // PG -> XO

A minimum of 6 arcospheres is required, with EP or any of [EP]+[GO] as catalysts.

Editor's pick:

    TZ -> LX x2 + GO  =>  GOTZ -> ELPX |  ET -> PO // PZ -> EG |  EO -> LG // PG -> XO
    1.  [TZ] + [GOTZ] + [] | GOTZ -> ELPX
    2.  [] + [EPTZ] + [LX] | ET -> PO // PZ -> EG
    3.  [] + [EGOP] + [LX] | EO -> LG // PG -> XO

This is the one path for the second alternative which returns one target set from the first stage.


##  Space Injection Data

The recipe itself:

    GT -> ZZ | EE

Of note: the second alternative exhibits flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    ZZ -> GT x2 + X   =>  XZ -> PT | PZ -> EG | ET -> PO | PG -> XO | XZ -> PT | PZ -> EG | EO -> LG | LO -> XT

    EE -> GT x2 + PT  =>  ET -> PO |  EO -> LG |  PG -> XO |  ELPX -> GOTZ // EO -> LG |  LO -> XT |  XZ -> PT
    EE -> GT x2 + XZ  =>  XZ -> PT |  ET -> PO |  EO -> LG |  PG -> XO |  ELPX -> GOTZ // EO -> LG |  LO -> XT

A minimum of 6 arcospheres is required, with XZ as catalysts.

Editor's pick:

    EE -> GT x2 + PT  =>  ET -> PO |  EO -> LG |  PG -> XO |  ELPX -> GOTZ // EO -> LG |  LO -> XT |  XZ -> PT
    1.  [EEEP] + [ET] + [] | ET -> PO
    2.  [EEPP] + [EO] + [] | EO -> LG
    3.  [EELP] + [PG] + [] | PG -> XO
    4.  [] + [EELOPX] + [] | ELPX -> GOTZ // EO -> LG
    5.  [Z] + [LO] + [GGT] | LO -> XT
    6.  [] + [XZ] + [GGTT] | XZ -> PT

This is the one path for the second alternative which returns one target set (and change) from the fourth stage.

With a 3rd catalyst, it's possible to solve the second alternative in only 4 stages, such as with:

    EE -> GT x2 + LPX  =>  ELPX -> GOTZ |  EO -> LG // ET -> PO |  EO -> LG // PG -> XO |  LO -> XT // XZ -> PT
    1.  [EEE] + [ELPX] + [] | ELPX -> GOTZ
    2.  [EZ] + [EEOT] + [G] | EO -> LG // ET -> PO
    3.  [Z] + [EGOP] + [LG] | EO -> LG // PG -> XO
    4.  [] + [LOXZ] + [GGL] | LO -> XT // XZ -> PT

With this one path returning a second sphere (though a catalyst) from the second stage.


##  Space Warping Data

The recipe itself:

    EP -> TZ | GO

Of note: both alternatives exhibit flipped polarity, requiring at least one inversion to restore it.

Solve paths:

    TZ -> EP x2 + GO  =>  GOTZ -> ELPX |  LT -> EZ // XZ -> PT |  ET -> PO // PZ -> EG
    TZ -> EP x2 + LX  =>  LT -> EZ // XZ -> PT |  ET -> PO // PZ -> EG |  GOTZ -> ELPX

    GO -> EP x2 + LX  =>  LO -> XT // XG -> LZ |  LT -> EZ // XZ -> PT |  GOTZ -> ELPX
    GO -> EP x2 + TZ  =>  GOTZ -> ELPX |  LO -> XT // XG -> LZ |  LT -> EZ // XZ -> PT

A minimum of 6 arcospheres is required, with LT, LX, or XZ as catalysts.

Editor's picks:

    TZ -> EP x2 + GO  =>  GOTZ -> ELPX |  LT -> EZ // XZ -> PT |  ET -> PO // PZ -> EG
    1.  [TZ] + [GOTZ] + [] | GOTZ -> ELPX
    2.  [] + [LTXZ] + [EP] | LT -> EZ // XZ -> PT
    3.  [] + [EPTZ] + [EP] | ET -> PO // PZ -> EG

    GO -> EP x2 + TZ  =>  GOTZ -> ELPX |  LO -> XT // XG -> LZ |  LT -> EZ // XZ -> PT
    1.  [GO] + [GOTZ] + [] | GOTZ -> ELPX
    2.  [] + [GLOX] + [EP] | LO -> XT // XG -> LZ
    3.  [] + [LTXZ] + [EP] | LT -> EZ // XZ -> PT

Those are the paths that return one target set from the first stage.


#   Deep Space Science IV

##  Wormhole Data

The recipe itself:

    EL + GZ -> PX + OT

Of note: no alternative in this one.

Solve paths:

    PXOT -> ELGZ + E  =>  EO -> LG |  LT -> EZ // XG -> LZ |  PZ -> EG
    PXOT -> ELGZ + G  =>  XG -> LZ |  LT -> EZ // PZ -> EG |  EO -> LG
    PXOT -> ELGZ + L  =>  LT -> EZ |  EO -> LG // PZ -> EG |  XG -> LZ
    PXOT -> ELGZ + Z  =>  PZ -> EG |  EO -> LG // XG -> LZ |  LT -> EZ

Exactly 5 arcospheres are required, with any of E, G, L, or Z as catalysts.

No editor's pick: all paths return 3 of the 4 target arcospheres from their second stage, and the 4th target arcosphere
from their 3rd and last stage. They do return different subsets, though: ELZ, EGZ, ELG, and GLZ respectively.

With a second catalyst, it's possible to solve in only 2 stages:

    PXOT -> ELGZ + EG  =>  EO -> LG // XG -> LZ |  LT -> EZ // PZ -> EG
    PXOT -> ELGZ + EL  =>  EO -> LG // LT -> EZ |  PZ -> EG // XG -> LZ
    PXOT -> ELGZ + EZ  =>  EO -> LG // PZ -> EG |  LT -> EZ // XG -> LZ
    PXOT -> ELGZ + LG  =>  LT -> EZ // XG -> LZ |  EO -> LG // PZ -> EG
    PXOT -> ELGZ + GZ  =>  PZ -> EG // XG -> LZ |  EO -> LG // LT -> EZ
    PXOT -> ELGZ + LZ  =>  LT -> EZ // PZ -> EG |  EO -> LG // XG -> LZ

And once again all paths are equivalent.
