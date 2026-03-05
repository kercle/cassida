// ------------------------------------------------------------------------------------------------
// BLANKSEQ (1 OR MORE) TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                             | Test Expr                           | Expected Matches
//  ------------------------------------|-------------------------------------|------------------
//  f[BlankSeq[]]                       | f[1]                                | 1
//  f[BlankSeq[]]                       | f[1, 2, 3]                          | 1
//  f[BlankSeq[]]                       | f[]                                 | 0
//  f[1, BlankSeq[]]                    | f[1, 2]                             | 1
//  f[1, BlankSeq[]]                    | f[1]                                | 0
//  f[1, BlankSeq[]]                    | f[1, 2, 3, 4]                       | 1
//  f[BlankSeq[], 1]                    | f[2, 1]                             | 1
//  f[BlankSeq[], 1]                    | f[1]                                | 0
//  f[BlankSeq[], 1]                    | f[2, 3, 1]                          | 1
//  f[1, BlankSeq[], 2]                 | f[1, 99, 2]                         | 1
//  f[1, BlankSeq[], 2]                 | f[1, 2]                             | 0
//  f[1, BlankSeq[], 2]                 | f[1, 3, 4, 2]                       | 1
//  f[BlankSeq[], BlankSeq[]]           | f[1]                                | 0
//  f[BlankSeq[], BlankSeq[]]           | f[1, 2]                             | 1
//  f[BlankSeq[], BlankSeq[]]           | f[1, 2, 3]                          | 2
//  f[BlankSeq[], BlankSeq[]]           | f[1, 2, 3, 4]                       | 3


// ------------------------------------------------------------------------------------------------
// BLANKNULLSEQ (0 OR MORE) TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                    | Test Expr                    | Expected Matches
//  -------------------------------------------|------------------------------|------------------
//  f[BlankNullSeq[]]                          | f[]                          | 1
//  f[BlankNullSeq[]]                          | f[1]                         | 1
//  f[BlankNullSeq[]]                          | f[1, 2, 3]                   | 1
//  f[1, BlankNullSeq[]]                       | f[1]                         | 1
//  f[1, BlankNullSeq[]]                       | f[1, 2, 3]                   | 1
//  f[BlankNullSeq[], 1]                       | f[1]                         | 1
//  f[BlankNullSeq[], 1]                       | f[2, 3, 1]                   | 1
//  f[BlankNullSeq[], BlankNullSeq[]]          | f[]                          | 1
//  f[BlankNullSeq[], BlankNullSeq[]]          | f[1]                         | 2
//  f[BlankNullSeq[], BlankNullSeq[]]          | f[1, 2]                      | 3
//  f[BlankNullSeq[], BlankNullSeq[]]          | f[1, 2, 3]                   | 4
//  f[BlankSeq[], BlankNullSeq[]]              | f[1]                         | 1
//  f[BlankSeq[], BlankNullSeq[]]              | f[1, 2]                      | 2
//  f[BlankSeq[], BlankNullSeq[]]              | f[1, 2, 3]                   | 3
//  f[BlankNullSeq[], BlankSeq[]]              | f[1]                         | 1
//  f[BlankNullSeq[], BlankSeq[]]              | f[1, 2]                      | 2


// ------------------------------------------------------------------------------------------------
// NAMED SEQUENCE BINDING TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                                              | Test Expr          | Expected Matches
//  ---------------------------------------------------------------------|--------------------|-----------------
//  f[Pattern[x, BlankSeq[]]]                                            | f[1]               | 1
//  f[Pattern[x, BlankSeq[]]]                                            | f[1, 2, 3]         | 1
//  f[Pattern[x, BlankNullSeq[]]]                                        | f[]                | 1
//  f[Pattern[x, BlankNullSeq[]]]                                        | f[1, 2]            | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]                    | f[1, 1]            | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]                    | f[1, 2, 1, 2]      | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]                    | f[1, 2]            | 0
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]                    | f[1, 2, 3]         | 0
//  f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]]            | f[]                | 1
//  f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]]            | f[1, 1]            | 1
//  f[1, Pattern[x, BlankSeq[]], 2]                                      | f[1, 5, 6, 2]      | 1
//  f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]]           | f[1, 2, 1]         | 1
//  f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]]           | f[1, 2, 3]         | 0
