///Number x2

{1111};

I = {q0};
F = {q1};

(q0, 1, 0, R, q1);

(q1, 1, 0, R, q2);
(q1, 0, 0, H, q1);

(q2, 1, 1, R, q2);
(q2, 0, 0, R, q3);

(q3, 1, 1, R, q3);
(q3, 0, 1, L, q4);

(q4, 1, 1, L, q4);
(q4, 0, 0, L, q5);

(q5, 1, 1, L, q5);
(q5, 0, 1, R, q1);

