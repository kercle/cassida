// ------------------------------------------------------------------------------------------------
// PREDICATE TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                                              | Test Expr       | Expected Matches
//  ---------------------------------------------------------------------|-----------------|------------------
//  PatternTest[Blank[], NumberQ]                                         | 5               | 1
//  PatternTest[Blank[], NumberQ]                                         | foo             | 0
//  PatternTest[Blank[], NumberQ]                                         | f[1]            | 0
//  PatternTest[Blank[], SymbolQ]                                         | foo             | 1
//  PatternTest[Blank[], SymbolQ]                                         | 5               | 0
//  f[PatternTest[Blank[], NumberQ]]                                      | f[3]            | 1
//  f[PatternTest[Blank[], NumberQ]]                                      | f[x]            | 0
//  f[PatternTest[Blank[], NumberQ], PatternTest[Blank[], SymbolQ]]       | f[1, x]         | 1
//  f[PatternTest[Blank[], NumberQ], PatternTest[Blank[], SymbolQ]]       | f[x, 1]         | 0
//  f[PatternTest[Pattern[x, Blank[]], NumberQ]]                          | f[3]            | 1
//  f[PatternTest[Pattern[x, Blank[]], NumberQ]]                          | f[foo]          | 0
//  Add[PatternTest[Blank[], NumberQ], PatternTest[Blank[], SymbolQ]]     | Add[x, 1]       | 1
//  Add[PatternTest[Blank[], NumberQ], PatternTest[Blank[], SymbolQ]]     | Add[1, 2]       | 0
