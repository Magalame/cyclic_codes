#include <stdio.h>
#include <string.h>

#include "flint_fac.h"

int main(void)
{
    //char* poly = "128 2  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1";
    char* poly = "4 2  1 0 0 1";
    int* length;

    char** my_factors = factorize(poly, length);
    int i;

    for (i = 0; i < *length; i++) { 

        printf("%s\n",*(my_factors+i));
    }


}
