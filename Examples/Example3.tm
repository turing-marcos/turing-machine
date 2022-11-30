{01110111100}; //(x, y)

F = {f};
I={ q0 };

//Search for the separator
(q0, 1, 1, R, q0);
(q0, 0, 0, R, q1);

//Go to the end of y
(q1, 1, 1, R, q1);
(q1, 0, 0, L, q2);

//Substracting 1 from y
(q2, 1, 0, L, q3);

//If there is no more y, we stop
(q2, 0, 0, H, f);

//Find the separator
(q3, 1, 1, L, q3);
(q3, 0, 0, L, q4);

//Go to the beginning of x
(q4, 1, 1, L, q4);
(q4, 0, 0, R, q5);

//Start all over again
(q5, 1, 0, R, q0);
(q5, 0, 0, R, q6); //This means that x>y

//Find all the ones and delete them
(q6, 1, 0, R, q6);
(q6, 0, 0, H, f);

(f, 0, 0, H, f);
(f, 1, 1, H, f);