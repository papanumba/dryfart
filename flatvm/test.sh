gcc -c -std=c99 -pedantic -Wall -Wextra -Iinclude -O3 src/alzhmr.c
clang -c -std=c++11 -pedantic -Wall -Wextra -Iinclude -fPIE -O3 main.cpp
g++ *.o && strip a.out
