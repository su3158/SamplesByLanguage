#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(void)
{
    srand((unsigned)time(0UL));
    int r = rand() % 100;
    printf("%d", r);
    int n = rand() % 10000;
    printf("%d", n);
    return 0;
}