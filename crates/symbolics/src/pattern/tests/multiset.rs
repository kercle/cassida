// ------------------------------------------------------------------------------------------------
// MULTISET (COMMUTATIVE) LITERAL TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                             | Test Expr                           | Expected Matches
//  ------------------------------------|-------------------------------------|------------------
//  Add[1, 2]                           | Add[1, 2]                           | 1
//  Add[1, 2]                           | Add[2, 1]                           | 1
//  Add[1, 2, 3]                        | Add[3, 1, 2]                        | 1
//  Add[1, 2, 3]                        | Add[1, 2, 4]                        | 0
//  Add[1, 2]                           | Add[1, 2, 3]                        | 0
//  Add[1, 1, 2]                        | Add[1, 2, 1]                        | 1
//  Add[1, 1, 2]                        | Add[2, 1, 1]                        | 1
//  Add[1, 1, 2]                        | Add[1, 2, 2]                        | 0
//  Mul[2, 3]                           | Mul[3, 2]                           | 1
//  Mul[2, 3]                           | Mul[2, 4]                           | 0
//  Add[f[1], f[2]]                     | Add[f[2], f[1]]                     | 1
//  Add[f[1], f[2]]                     | Add[f[1], f[3]]                     | 0


// ------------------------------------------------------------------------------------------------
// MULTISET WILDCARD / BLANK TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                          | Test Expr              | Expected Matches
//  --------------------------------------------------|------------------------|------------------
//  Add[Blank[], 1]                                   | Add[1, 2]              | 1
//  Add[Blank[], 1]                                   | Add[2, 1]              | 1
//  Add[Blank[], 1]                                   | Add[2, 3]              | 0
//  Add[Blank[], Blank[]]                             | Add[1, 2]              | 2
//  Add[Blank[], Blank[]]                             | Add[1, 1]              | 2
//  Add[Blank[], Blank[], Blank[]]                    | Add[1, 2, 3]           | 6
//  Add[Blank[], 1, 2]                                | Add[2, 3, 1]           | 1
//  Add[Blank[], 1, 2]                                | Add[2, 1, 1]           | 0
//  Add[Pattern[x, Blank[]], Pattern[x, Blank[]]]     | Add[1, 1]              | 2
//  Add[Pattern[x, Blank[]], Pattern[x, Blank[]]]     | Add[1, 2]              | 0
//  Add[Pattern[x, Blank[]], Pattern[y, Blank[]]]     | Add[1, 2]              | 2


// ------------------------------------------------------------------------------------------------
// MULTISET BLANKSEQ / BLANKNULLSEQ TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                     | Test Expr                  | Expected Matches
//  ---------------------------------------------|----------------------------|------------------
//  Add[BlankNullSeq[]]                          | Add[]                      | 1
//  Add[BlankNullSeq[]]                          | Add[1, 2, 3]               | 1
//  Add[BlankSeq[]]                              | Add[]                      | 0
//  Add[BlankSeq[]]                              | Add[1]                     | 1
//  Add[BlankSeq[]]                              | Add[1, 2, 3]               | 1
//  Add[1, BlankNullSeq[]]                       | Add[1]                     | 1
//  Add[1, BlankNullSeq[]]                       | Add[1, 2, 3]               | 1
//  Add[1, BlankSeq[]]                           | Add[1]                     | 0
//  Add[1, BlankSeq[]]                           | Add[1, 2]                  | 1
//  Add[1, BlankSeq[]]                           | Add[2, 1, 3]               | 1
//  Add[Pattern[x, BlankNullSeq[]], 1]           | Add[1]                     | 1
//  Add[Pattern[x, BlankNullSeq[]], 1]           | Add[2, 3, 1]               | 1
//  Add[Pattern[x, BlankSeq[]], 1]               | Add[1]                     | 0
//  Add[Pattern[x, BlankSeq[]], 1]               | Add[2, 1]                  | 1
