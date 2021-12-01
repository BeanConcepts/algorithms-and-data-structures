## It looks like you can relate a problem and solve it with the right algorithm.

### Now, let's see how you actually implement one.

Select and Re-arrange the following instructions to form 
an algorithm to solve [The tower of Hanoi](https://en.wikipedia.org/wiki/Tower_of_Hanoi)?

![Tower_of_Hanoi](https://upload.wikimedia.org/wikipedia/commons/8/8d/Iterative_algorithm_solving_a_6_disks_Tower_of_Hanoi.gif)

The final procedure should be a function of the form move(n,A,B) which will move 
a stack of size n from tower A to tower B using our valid operations. 

Please note that you may not need all the instructions and may need to repeat some instructions at least twice.


1.
```
def move(n,A,B): 
```
2.
```
Let C be the third tower (that isn't A or B) 
```
3.
```
if (n>1): 
```
4.
```
move(n-1,C,B)
```
5.
```
move(1,A,B)
```
6.
```
if (n == 1): 
```
7.
```
move(n-1,A,C) 
```
8.
```
move(n-1,B,C)
```        