///SUMATORY FROM 1 TO X

{1111};

I = {q0};
F = {f};

//Eliminate the first one
(q0, 1, 0, R, q1);

//Slip a one
(q1, 1, 1, R, q2);

//STOP
(q1, 0, 0, H, f);

//Leave a mark
(q2, 1, 0, R, q3);
(q2, 0, 0, R, q1);

//Go to the end of x
(q3, 1, 1, R, q3);
(q3, 0, 0, R, q4);

//Go to the end and add 1
(q4, 0, 1, L, q5);
(q4, 1, 1, R, q4);

//Go to the separator
(q5, 1, 1, L, q5);
(q5, 0, 0, L, q6);

//Search for the mark, remove it and start over
(q6, 1, 1, L, q6);
(q6, 0, 1, R, q2);