// ------------------------------------------------------------------------------------------------
// WILDCARD TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                             | Test Expr                           | Expected Matches
//  ------------------------------------|-------------------------------------|------------------
//  Blank[]                             | 5                                   | 1
//  Blank[]                             | foo                                 | 1
//  Blank[]                             | f[1, 2]                             | 1
//  f[Blank[]]                          | f[1]                                | 1
//  f[Blank[]]                          | f[f[1, 2]]                          | 1
//  f[Blank[]]                          | f[1, 2]                             | 0
//  f[Blank[], Blank[]]                 | f[1, 2]                             | 1
//  f[Blank[], Blank[]]                 | f[1, 2, 3]                          | 0
//  f[Blank[], 2, Blank[]]              | f[1, 2, 3]                          | 1
//  f[Blank[], 2, Blank[]]              | f[1, 3, 3]                          | 0
//  f[g[Blank[]], 2]                    | f[g[99], 2]                         | 1
//  f[g[Blank[]], 2]                    | f[g[99], 3]                         | 0
//  f[g[Blank[]]]                       | f[h[1]]                             | 0


// ------------------------------------------------------------------------------------------------
// HEAD PATTERN (WILDCARD WITH HEAD CONSTRAINT) TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                             | Test Expr                           | Expected Matches
//  ------------------------------------|-------------------------------------|------------------
//  Blank[f]                            | f[1, 2]                             | 1
//  Blank[f]                            | g[1, 2]                             | 0
//  Blank[f]                            | 5                                   | 0
//  f[Blank[g]]                         | f[g[1]]                             | 1
//  f[Blank[g]]                         | f[h[1]]                             | 0
//  f[Blank[g], Blank[g]]               | f[g[1], g[2]]                       | 1
//  f[Blank[g], Blank[g]]               | f[g[1], h[2]]                       | 0
//  f[Blank[g], Blank[h]]               | f[g[1], h[2]]                       | 1
//  f[Blank[f]]                         | f[f[f[]]]                           | 1


// ------------------------------------------------------------------------------------------------
// NAMED VARIABLE / PATTERN BINDING TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                                                          | Test Expr             | Expected Matches
//  -----------------------------------------------------------------|-----------------------|------------------
//  Pattern[x, Blank[]]                                              | 5                     | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]]]                      | f[1, 1]               | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]]]                      | f[1, 2]               | 0
//  f[Pattern[x, Blank[]], Pattern[y, Blank[]]]                      | f[1, 2]               | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]]                   | f[1, g[1]]            | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]]                   | f[1, g[2]]            | 0
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] | f[3, 3, 3]            | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] | f[3, 3, 4]            | 0
//  f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]]                    | f[f[1], f[1]]         | 1
//  f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]]                    | f[f[1], f[2]]         | 0
