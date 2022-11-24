/// Based on example II from the lecture slides
// Build a TM M that computes f(x,y), 
// where g(x,y) = x+y, M1 computes g(x,y) M1=({0,1}, {q0,q1,q2}, T1, q0, {q2}) 
// f(x,y) = x+1       if y=0
//			g(x,y)   if y>0

// Input x=4, y=3
{[p0]1111101111};
// Input x=4, y=0
//{[p0]1111101};

F = {q2, pf};

// Start with the initial state of M
// Check if y=0 or not
// If it is 0, leave zeros in place of y
// Else if y is >0, then go to the beginning of x and call M1

(p0, 1, 1, R, p0);
(p0, 0, 0, R, p1);

(p1, 1, 1, R, p2);

// If y == 0
(p1, 0, 0, L, p3);
(p3, 1, 0, N, pf);

// If y>0 -> g(x, y)
(p2, 1, 1, L, p4);

(p4, 1, 1, L, p4);
(p4, 0, 0, L, p5);

(p5, 1, 1, L, p5);
(p5, 0, 0, R, pf);

// For the simulator to halt
//(pf, 0, 0, H, pf);
//(pf, 1, 1, H, pf);
