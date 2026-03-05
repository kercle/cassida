// ------------------------------------------------------------------------------------------------
// NESTED / MIXED DEEP STRUCTURE TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                                                       | Test Expr               | Expected Matches
//  ------------------------------------------------------------------------------|-------------------------|------------------
//  f[g[Blank[], Blank[]]]                                                         | f[g[1, 2]]              | 1
//  f[g[Blank[], Blank[]]]                                                         | f[g[1]]                 | 0
//  f[g[BlankSeq[]]]                                                               | f[g[1, 2, 3]]           | 1
//  f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]]                                 | f[g[5], 5]              | 1
//  f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]]                                 | f[g[5], 6]              | 0
//  f[BlankSeq[], g[BlankSeq[]]]                                                   | f[1, 2, g[3, 4]]        | 1
//  f[BlankSeq[], g[BlankSeq[]]]                                                   | f[g[3, 4]]              | 0
//  Add[Mul[Blank[], Blank[]], Blank[]]                                             | Add[Mul[2, 3], 4]       | 2
//  Add[Mul[Blank[], Blank[]], Blank[]]                                             | Add[4, Mul[2, 3]]       | 2
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]]            | f[1, g[1, 1]]           | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]]            | f[1, g[1, 2]]           | 0
//  Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]]                            | Add[f[3], f[3]]         | 2
//  Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]]                            | Add[f[3], f[4]]         | 0


// ------------------------------------------------------------------------------------------------
// EDGE CASES / DEGENERATE INPUTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                                                             | Test Expr  | Expected Matches
//  ------------------------------------------------------------------------------------|------------|------------------
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                               | f[1, 2, 3] | 3
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                               | f[1, 2]    | 1
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                               | f[1]       | 0
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                                   | f[]        | 1
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                                   | f[1]       | 3
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                                   | f[1, 2]    | 6
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]           | f[1, 1, 1] | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]           | f[1,2,1,2,1,2] | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]           | f[1, 2, 3] | 0
//  f[Blank[]]                                                                           | f[]        | 0
//  f[]                                                                                  | f[Blank[]] | 0
//  Add[]                                                                                | Add[]      | 1
//  Add[1]                                                                               | Add[1]     | 1
//  f[g[]]                                                                               | f[g[]]     | 1
//  f[g[]]                                                                               | f[g[1]]    | 0
