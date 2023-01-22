// gcc test.c -o test_no_db
// gcc -g test.c -o test_dbg

#include <stdio.h>

void add(int *a)
{
    *a = *a + 5;
}

int main()
{
    int a = 5;

    add(&a);

    printf("A is now: %d", a);

    return 0;
}