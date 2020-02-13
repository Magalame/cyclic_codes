#ifndef _FLINT_FAC_
#define _FLINT_FAC_

    extern char** factorize(char* poly_str, int* length); 
    extern double get_multiplicity(int* places, int nb, int ring_n);
    extern char* get_char(const size_t* places, size_t nb, size_t ring_n);
    extern char* get_prefactor(const size_t* places, size_t nb, size_t ring_n);
    extern char** get_pf_and_char(const size_t* places, size_t nb, size_t ring_n); 
    extern void deallocate(char** factors);
    extern void deallocate_str(char* factors);
    extern char* heya();
    extern char* mem_test1();
    extern char* mem_test2(const size_t* places);

#endif
