#include <stdio.h>
#include <stdlib.h>

typedef char String[1024];

int main(void)
{
    String name;
    printf("your name\n");
    scanf("%s", name);

    String ageStr;
    printf("your age\n");
    scanf("%s", ageStr);

    int age = atoi(ageStr);
    printf("wlecome %s for %d", name, age);
    return 0;
}