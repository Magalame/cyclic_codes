#include <stdio.h>
#include <string.h>
// gcc -o flint_fac.o -c flint_fac.c -lflint -lmpfr -lmpir -fPIC && gcc -shared -o libflintfac.so  flint_fac.o && sudo mv libflintfac.so /usr/local/lib/ && cargo clean -p mnlc_simulations && cargo run
//gcc test_link.c -L/home/nouey/Projects/simul/mnlc_simulations/tests -lflintfac -lflint -lmpfr -lmpir && ./a.out
#include "/usr/local/include/flint/flint.h"
#include "/usr/local/include/flint/fmpz_poly.h"
#include "/usr/local/include/flint/nmod_poly.h"
#include "/usr/local/include/flint/ulong_extras.h"
#include "/usr/local/include/flint/nmod_poly.h"

char** factorize(char* poly_str, int* length){

    nmod_poly_t  f; 
    nmod_poly_factor_t facs;
    mp_limb_t n = 2; // GF(2)

    //printf("Poly: %s end\n", poly_str);

    nmod_poly_init(f,n);
    nmod_poly_factor_init(facs);

    nmod_poly_set_str(f, poly_str);
    nmod_poly_factor(facs, f); //the factorization

    char** my_factors = malloc((facs->num) * sizeof(char*)); // we allocate enough space for the nb of factors + 1
    int i;

    char buffer[10]; //will temporarily store the exponent of each factor as a string, after their conversion from int to str

    for (i=0; i < facs->num; i++){

        *(my_factors + i) = nmod_poly_get_str(facs->p + i); //iterate over the factors in facs->p
        sprintf(buffer, "%ld", facs->exp[i]); 
        strcat(*(my_factors + i)," ^");
        strcat(*(my_factors + i), buffer);

    }

    *length = facs->num;

    nmod_poly_clear(f);
    nmod_poly_factor_clear(facs);
    
    return my_factors;
   
}

char* get_char(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(g, f, r);

    //nmod_poly_factor(facs, g); //the factorization

    char* char_poly = nmod_poly_get_str(g);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_factor_clear(facs);

    
    return char_poly;
   
}

char* get_prefactor(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;
    nmod_poly_t  pre_factor;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);
    nmod_poly_init(pre_factor, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(g, f, r);
    nmod_poly_div_basecase(pre_factor, f, g);
    //nmod_poly_factor(facs, g); //the factorization

    char* char_poly = nmod_poly_get_str(pre_factor);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_clear(pre_factor);
    nmod_poly_factor_clear(facs);
    
    return char_poly;
   
}

// void mk_mul(const size_t* places1, size_t nb1, const size_t* places2, size_t nb2){

//     nmod_poly_t  f; 
//     nmod_poly_t  g;

//     nmod_poly_t  tmp;

//     nmod_poly_t  res;

    
//     nmod_poly_init(f, 2); // GF(2)
//     nmod_poly_init(g, 2);

//     nmod_poly_init(tmp, 2);

//     nmod_poly_init(res, 2);


//     size_t index = 0;

//     for (index = 0; index < nb1; index ++) {
//         nmod_poly_set_coeff_ui(f, places1[index], 1);
//     }

//     for (index = 0; index < nb2; index ++) {
//         nmod_poly_set_coeff_ui(g, places2[index], 1);
//     }

    
//     nmod_poly_mul_classical(tmp, f , g);
//     nmod_poly_mul_classical(res , tmp , g);

//     slong degree = nmod_poly_degree(res);

//     int i = 0;

//     int sum = 0;

//     for (i=0;i <= degree; i++){

//         if (nmod_poly_get_coeff_ui(res, i) == 1) {
//             sum ++;
//         }

//     }

//     if (sum %2 ==  6) {

//         // nmod_poly_reverse(tmp, res, degree + 1);
        
//         // if (nmod_poly_equal(tmp,res)){
//             printf("%li,%li,%li,%li,%li\n", places1[0],places1[1],places1[2],places1[3],places1[4]);
//             nmod_poly_print(res);
//             printf("\n");
//         // }
        

//     }


//     nmod_poly_clear(f);
//     nmod_poly_clear(g);

//     nmod_poly_clear(res);

    
   
// }

void print_array(size_t *ptr, size_t len){
    size_t i;

    printf("\n");

    for (i = 0; i < len; i ++){
        printf("%li,",ptr[i]);
    }

    printf("\n");
}

void mk_mul(size_t* places1, size_t nb1, const size_t* places2, size_t nb2){

    nmod_poly_t  f; 
    nmod_poly_t  g;

    nmod_poly_t  res;

    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(g, 2);

    nmod_poly_init(res, 2);


    size_t index = 0;

    for (index = 0; index < nb1; index ++) {
        nmod_poly_set_coeff_ui(f, places1[index], 1);
    }

    for (index = 0; index < nb2; index ++) {
        nmod_poly_set_coeff_ui(g, places2[index], 1);
    }

    // printf("\nf:");
    // nmod_poly_print(f);
    // printf("\ng:");
    // nmod_poly_print(g);
    
    nmod_poly_mul_classical(res , f , g);
    nmod_poly_mul_classical(f , res , g);

    slong degree = nmod_poly_degree(f);

    int i = 0;

    int sum = 0;

    int tmp = 0;

    for (i=0;i <= degree; i++){

        tmp = nmod_poly_get_coeff_ui(f, i);

        if (tmp == 1) {
            sum ++;
        }

    }

    if (sum % 2 == 0 && sum < 4) {
        
        printf("\n Found sum = %i, w=%li",sum,nb1);
        print_array(places1, nb1);
        nmod_poly_print(f);

    }


    nmod_poly_clear(f);
    nmod_poly_clear(g);

    nmod_poly_clear(res);

    
   
}

char** get_pf_and_char(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;
    nmod_poly_t  pre_factor;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);
    nmod_poly_init(pre_factor, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(g, f, r);
    nmod_poly_div_basecase(pre_factor, f, g);
    //nmod_poly_factor(facs, g); //the factorization

    char** res = malloc(2*sizeof(char*));

    *res = nmod_poly_get_str(pre_factor);
    *(res+1) = nmod_poly_get_str(g);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_clear(pre_factor);
    nmod_poly_factor_clear(facs);
    
    return res;
   
}

double get_multiplicity(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(g, f, r);

    nmod_poly_factor(facs, g); //the factorization

    int i;
    double mul = 0;


    for (i=0; i < facs->num; i++){

        mul = mul + facs->exp[i]*facs->exp[i];

    }


    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_factor_clear(facs);
    
    return mul;
   
}

char* heya(){
    char* str = malloc(63 * sizeof(char));
    strcpy(str,"heeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeya");
    *(str+62) = '\0';
    //printf("%s",str);
    return str;
}

void deallocate(char** factors){
    free(factors);
}

char* mem_test1(){
    char* poly1 = "128 2  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1";
    char* poly2 = "82 2  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1";

    nmod_poly_t f;
    nmod_poly_t r;
    nmod_poly_t g;

    nmod_poly_init(f, 2);
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);

    nmod_poly_set_str(f, poly1);
    nmod_poly_set_str(r, poly2);

    nmod_poly_gcd_euclidean(g, f, r);

    char *str = nmod_poly_get_str(g);

    

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);


    return str;
}

char* mem_test2(size_t* places){
    
    
    int nb = 5;

    int ring_n = 120;


    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(g, f, r);

    //nmod_poly_factor(facs, g); //the factorization

    char* char_poly = nmod_poly_get_str(g);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_factor_clear(facs);

    
    return char_poly;
}

void deallocate_str(char* str){
    flint_free(str);
}

// int main()
// {
//     //char* poly = "128 2  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1";
//     //char* poly = "82 2  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1";
//     // char* poly = "4 2  1 0 0  1";
//     // int* length;

//     // char** my_factors = factorize(poly,length);
    
//     // int i;

//     // printf("length:%d\n",*length);

//     // for (i = 0; i < *length; i++) { 

//     //     printf("%s\n",*(my_factors+i));
        
//     // }

//     // deallocate(my_factors);

//    // size_t coef_f[5] = {0, 1, 14, 25, 37};
//     size_t coef_r[2] = {0,120};


//     nmod_poly_t f;
//     nmod_poly_t r;
//     nmod_poly_t g;

//     nmod_poly_init(f, 2);
//     nmod_poly_init(r, 2);
//     nmod_poly_init(g, 2);

    
//     // nmod_poly_set_coeff_ui(f, coef_f[0], 1);
//     // nmod_poly_set_coeff_ui(f, coef_f[1], 1);
//     // nmod_poly_set_coeff_ui(f, coef_f[2], 1);
//     // nmod_poly_set_coeff_ui(f, coef_f[3], 1);
//     // nmod_poly_set_coeff_ui(f, coef_f[4], 1);

//    nmod_poly_set_str(f, "21 2  1 1 0 0 1 0 0 0 1 1 0 0 1 0 0 0 1 1 0 0 1");

//     nmod_poly_set_coeff_ui(r, coef_r[0], 1);
//     nmod_poly_set_coeff_ui(r, coef_r[1], 1);

//     nmod_poly_gcd_euclidean(g, f, r);

//     flint_printf("Polynomial:\n");
//     nmod_poly_print_pretty(f, "x");

//     nmod_poly_factor_t facs;
//     nmod_poly_factor_init(facs);
//     nmod_poly_factor(facs, g);

//     flint_printf("\nFactorisation:\n");
//     nmod_poly_factor_print(facs);

//     nmod_poly_clear(f);
//     nmod_poly_clear(r);
//     nmod_poly_clear(g);
//     nmod_poly_factor_clear(facs);

//     // size_t coef_f[5] = {0,1,5,25,46};
//     // size_t coef_r[2] = {0,60};


//     // nmod_poly_t f;
//     // nmod_poly_factor_t facs;

//     // nmod_poly_init(f, 2);
//     // nmod_poly_factor_init(facs);

    
//     // nmod_poly_set_coeff_ui(f, coef[0], 1);
//     // nmod_poly_set_coeff_ui(f, coef[1], 1);
//     // nmod_poly_set_coeff_ui(f, coef[2], 1);
//     // nmod_poly_set_coeff_ui(f, coef[3], 1);
//     // nmod_poly_set_coeff_ui(f, coef[4], 1);


//     // nmod_poly_factor(facs, f);


//     // flint_printf("Polynomial:\n");
//     // nmod_poly_print(f);
//     // flint_printf("\nFactorisation:\n");
//     // nmod_poly_factor_print(facs);

//     // nmod_poly_clear(f);
//     // nmod_poly_factor_clear(facs);


//     //

//     // nmod_poly_init(f, 2);
//     // nmod_poly_factor_init(facs);

    
//     // nmod_poly_set_coeff_ui(f, 0, 1);
//     // nmod_poly_set_coeff_ui(f, 60, 1);


//     // nmod_poly_factor(facs, f);


//     // flint_printf("Polynomial:\n");
//     // nmod_poly_print(f);
//     // flint_printf("\nFactorisation:\n");
//     // nmod_poly_factor_print(facs);

//     // nmod_poly_clear(f);
//     // nmod_poly_factor_clear(facs);


//     // size_t coef[2] = {0,3};

//     // size_t mul = get_multiplicity(coef,2);

//     // printf("%lu\n",mul);

//     return EXIT_SUCCESS;
// }

// int main()
// {


//     nmod_poly_t f;
//     nmod_poly_t f2;
//     nmod_poly_t res;

//     nmod_poly_init(f, 2);
//     nmod_poly_init(f2, 2);
//     nmod_poly_init(res, 2);

//     nmod_poly_set_str(f, "11 2  1 0 0 0 0 1 0 0 0 0 1");
//     nmod_poly_set_str(f2, "11 2  1 0 1 1 0 0 0 0 0 1 1");

//     nmod_poly_mul_classical(res , f , f2);


//     flint_printf("Polynomials:\n");
//     nmod_poly_print(f);
//     nmod_poly_print(f2);

//     flint_printf("\nMultiplication:\n");
//     nmod_poly_print_pretty(res, "x");

//     nmod_poly_clear(f);
//     nmod_poly_clear(f2);
//     nmod_poly_clear(res);

//     return EXIT_SUCCESS;
// }



// int main()
// {
//     int ring_n = 108;

//     nmod_poly_t pf1;
//     nmod_poly_t pf2;

//     nmod_poly_t g1;
//     nmod_poly_t g2;

//     nmod_poly_t fp1;
//     nmod_poly_t fp2;

//     nmod_poly_t c;

//     nmod_poly_t q;
//     nmod_poly_t r;

//     nmod_poly_init(pf1, 2);
//     nmod_poly_init(pf2, 2);

//     nmod_poly_init(g1, 2);
//     nmod_poly_init(g2, 2);

//     nmod_poly_init(fp1, 2);
//     nmod_poly_init(fp2, 2);

//     nmod_poly_init(c, 2);

//     nmod_poly_init(q, 2);
//     nmod_poly_init(r, 2);

//     nmod_poly_set_str(pf1, "15 2  0 0 0 1 1 0 0 1 0 0 1 1 0 0 1");
//     //extend_f(pf2, pf1);

//     nmod_poly_set_str(g1, "7 2  1 1 0 1 0 1 1");

//     nmod_poly_gcd_euclidean(c, pf1, g1);
//     nmod_poly_divrem_basecase(q, r, pf1, g1);

//     flint_printf("Prefactor:\n");

//     nmod_poly_print(pf1);
//     flint_printf("\n");
//     // nmod_poly_print(pf2);
//     // flint_printf("\n");

//     flint_printf("Q:\n");
//     nmod_poly_print(q);
//     flint_printf("\nR:\n");
//     nmod_poly_print(r);
//     flint_printf("\n");

//     flint_printf("\nCommon polynomial between f and g:\n");
//     nmod_poly_print(c);
//     flint_printf("\n");
//     // nmod_poly_print_pretty(c, "x");
//     // flint_printf("\n");

    
//     // nmod_poly_factor_t fac1;
//     // nmod_poly_factor_t fac2;
//     // nmod_poly_factor_init(fac1);
//     // nmod_poly_factor_init(fac2);
    
//     // nmod_poly_factor(fac1,pf1);
//     // nmod_poly_factor(fac2,pf2);

//     // flint_printf("\nFactorisation:\n");
//     // nmod_poly_factor_print(fac1);
//     // flint_printf("\nFactorisation 2:\n");
//     // nmod_poly_factor_print(fac2);

//     // nmod_poly_t ring;
//     // nmod_poly_t gcd1;
//     // nmod_poly_t gcd2;

//     // nmod_poly_init(ring, 2);
//     // nmod_poly_init(gcd1, 2);
//     // nmod_poly_init(gcd2, 2);

//     // nmod_poly_set_coeff_ui(ring, ring_n, 1);
//     // nmod_poly_set_coeff_ui(ring, 0, 1);

//     // nmod_poly_gcd_euclidean(gcd1, pf1, ring);
//     // nmod_poly_gcd_euclidean(gcd2, pf2, ring);

//     // flint_printf("\nGCD1: (should be 1) \n");
//     // nmod_poly_print(gcd1);
//     // flint_printf("\nGCD2: (should be 1) \n");
//     // nmod_poly_print(gcd2);
//     // flint_printf("\n");


//     nmod_poly_clear(pf1);
//     // nmod_poly_clear(pf2);
//     nmod_poly_clear(q);
//     nmod_poly_clear(r);
//     nmod_poly_clear(c);
//     nmod_poly_clear(g1);
//     nmod_poly_clear(g2);
//     // nmod_poly_factor_clear(fac1);
//     // nmod_poly_factor_clear(fac2);

//     return EXIT_SUCCESS;
// }



void init(size_t *ptr, size_t len){
    size_t i;

    for(i = 0; i < len; i ++){

        ptr[i] = i;

    }

}

void find_wt2(size_t n, size_t w, size_t* g, size_t w_g){

    size_t *indexes = (size_t*) malloc(w * sizeof(size_t));

    init(indexes, w);

    size_t i;

    while (indexes[1] <= n - w + 1 ) {

        //print_array(indexes,w);

        mk_mul(indexes, w, g, w_g);

        for (i = 2; i < w; i++){ //i=2 means we fix the first to 0
        
            if (indexes[i] == n - w + i) { // - deg_g
                indexes[i-1] = indexes[i-1] + 1;
                indexes[i] = indexes[i-1] + 1;

                size_t j;

                for (j = i + 1; j < w; j++){
                    indexes[j] = indexes[i] + j - i;
                }

                break;

            } else if (i == w-1) {
                indexes[i] = indexes[i] + 1;
            }

        } 

    }

}


int main(){
    

    size_t g_indices[7] = {0,3,4,5,6,7,10}; 
    size_t *g = g_indices;

    size_t w_g = 7;

    size_t w;

    for (w = 3; w < 7; w++){
        find_wt2(60,w, g, w_g);
    } 

    


    
    // for (i_0 = 0   ; i_0 <= n - 3 - deg_g; i_0 ++){
    //     for (i_1 = i_0 + 1; i_1 <= n - 2 - deg_g; i_1 ++){
    //         for (i_2 = i_1 + 1; i_2 <= n - 1 - deg_g; i_2 ++){
    //             for (i_3 = i_2 + 1; i_3 <= n - 0 - deg_g; i_3 ++){
                    
    //                 size_t indexes[4] = {i_0, i_1, i_2, i_3};
    //                 //printf("loop:%i,%i,%i,%i,%i\n", i_0,i_1,i_2,i_3,i_4);
    //                 mk_mul(indexes, 4, g, w_g);


                    
    //             }
    //         }
    //     }
    // }

    printf("\n");
}