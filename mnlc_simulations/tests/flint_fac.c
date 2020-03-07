#include <stdio.h>
#include <string.h>
// gcc -o flint_fac.o -c flint_fac.c -lflint -lmpfr -lmpir -fPIC && gcc -shared -o libflintfac.so  flint_fac.o && sudo mv libflintfac.so /usr/local/lib/ && cargo clean -p mnlc_simulations && cargo run
//gcc test_link.c -L/home/nouey/Projects/simul/mnlc_simulations/tests -lflintfac -lflint -lmpfr -lmpir && ./a.out
#include "/usr/local/include/flint/flint.h"
#include "/usr/local/include/flint/fmpz_poly.h"
#include "/usr/local/include/flint/nmod_poly.h"
#include "/usr/local/include/flint/ulong_extras.h"
#include "/usr/local/include/flint/nmod_poly.h"

#define MAX_G_PER_N 100

typedef unsigned long long int ulint;

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

void mul_array(nmod_poly_t  g, nmod_poly_struct** fac_to_mul, int nb_fac){

    nmod_poly_set(g, fac_to_mul[0]);

    // nmod_poly_t  tmp;
    // nmod_poly_init(tmp,2);

    for (int i = 1; i < nb_fac; i++){
        nmod_poly_mul_classical(g,  g, fac_to_mul[i]);
        //nmod_poly_set(g, tmp);
    }

    // nmod_poly_clear(tmp);

}

//https://www.geeksforgeeks.org/iterating-over-all-possible-combinations-in-an-array-using-bits/
void find_gs(nmod_poly_struct** factors, int nb_of_facs, nmod_poly_struct** gs, int* nb_of_gs_ptr, int target_k){

    int nb_of_gs = 0;

    printf("nb of facs:%d\n",nb_of_facs);

    ulint range = (1ULL << nb_of_facs) - 1; 

    printf("range:%llu\n",range);

    ulint trials = 0;
  
    for (ulint i = 1; i <= range; i++) {

        /*
        i will hold place of the indexes, from 000...001 to 111...111
        */ 

        // x is where we are in the bit representation of i
        // we copy i to y, then will consume y to keep track of the "1" in i
        // sum is the sum of the degrees
        int x = 0, sum = 0;
        int nb_fac_loaded = 0; 
        ulint y = i;
        trials ++;


        //this section gets the degrees of thepoly determined by i
        // and checks if we get the good k

        int first_x;
        int found_fx = 0;
  
        while (y > 0) { 
  
            if (y & 1 == 1) { 
  
                // sum over degrees 
                sum += nmod_poly_degree(factors[x]); 
                nb_fac_loaded++;

                if (!found_fx){
                    found_fx = 1;
                    first_x = x;
                }

                if (sum > target_k) {
                    i = i + (1ULL << first_x) -1; // -1 since it'll be added at the next loop
                    break;
                }
            } 

            x++; 
            y = y >> 1; 

        } 


  
        // If sum is found, get the g that comes out of it
        if (sum == target_k){

            nmod_poly_struct** fac_to_mul = malloc(nb_fac_loaded * sizeof(nmod_poly_t)); 

            int x = 0;
            ulint y = i;
            int i_mul = 0;


            while (y > 0) { 
  
                if (y & 1 == 1) { 

                    fac_to_mul[i_mul] = factors[x];
                    i_mul++;

                } 

                x++; 
                y = y >> 1; 

            }

            nmod_poly_struct* g = malloc(sizeof(nmod_poly_struct));

            nmod_poly_init(g,2);

            mul_array(g, fac_to_mul, nb_fac_loaded);

            int found = 0;

            for (int j = 0; j < nb_of_gs; j++){

                if (nmod_poly_equal(g, gs[j])){
                    found = 1;
                    nmod_poly_clear(g);
                }

            }

            if (found == 0) {

                gs[nb_of_gs] = g;
                //flint_printf("nb_of_gs:%d\n",nb_of_gs);
                nb_of_gs++;
                *nb_of_gs_ptr = nb_of_gs;

            }

            free(fac_to_mul); //FREEEEE
            

            if (nb_of_gs >= MAX_G_PER_N) { // if we reached the number of distinct g, we break out
                printf("Nb of trials:%llu",trials);
                break;
            }

        }
    } 

    printf("Nb of trials:%llu",trials);
    
} 

void get_polys(int n, int target_k){

    nmod_poly_t  f; 
    nmod_poly_factor_t facs;

    // printf("Poly: %s end\n", poly_str);

    nmod_poly_init(f,2);
    nmod_poly_factor_init(facs);

    nmod_poly_set_coeff_ui(f, 0, 1);// we just define the ring
    nmod_poly_set_coeff_ui(f, n, 1);
    nmod_poly_factor(facs, f); //the factorization


    int nb_of_facs = 0;
    //nb_of_facs holds the number of factor multiplicity included
    for (int i=0; i < facs->num; i++){

        nb_of_facs += facs->exp[i];

    }

    
    nmod_poly_struct** factors = malloc(nb_of_facs * sizeof(nmod_poly_t)); // stores the facs, dédoublés for multiplicity
    slong* degrees = malloc(nb_of_facs * sizeof(slong)); // holdsthe degree of each fac

    int count_facs = 0;

    for (int i=0; i < facs->num; i++){

        for(int j=0; j < facs->exp[i]; j++) {
            factors[count_facs] = (facs->p + i);
            degrees[count_facs] =  nmod_poly_degree(facs->p + i);
            count_facs += 1;
        }

    }

    nmod_poly_struct** gs = malloc(MAX_G_PER_N * sizeof(nmod_poly_t)); // we hope there are no more than 20 gs per 

    int nb_of_gs = 0;

    find_gs(factors, nb_of_facs, gs, &nb_of_gs, target_k);

    // // then need to find fs that match these gs

    // //int max_deg = n - target_k - 1; 
    // printf("nb g:%i\n",nb_of_gs);

    // for(int i = 0; i < nb_of_gs; i++){
    //     nmod_poly_print(gs[i]);
    //     printf("\n");
    //     free(gs[i]);
    // }
    

    // free(factors);
    // free(degrees);
    // free(gs);
    // nmod_poly_clear(f);
    // nmod_poly_factor_clear(facs);
    

   
}



  
size_t get_k(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  g;

    
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

    size_t k = nmod_poly_degree(g);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);

    
    return k;
   
}

char* get_gen(const size_t* places, size_t nb, size_t ring_n){

    nmod_poly_t  f; 
    nmod_poly_t  r;
    nmod_poly_t  h;
    nmod_poly_t  g;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(h, 2);
    nmod_poly_init(g, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(h, f, r);

    nmod_poly_div_basecase(g, r, h);

    //nmod_poly_factor(facs, g); //the factorization

    char* char_poly = nmod_poly_get_str(g);

    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(g);
    nmod_poly_factor_clear(facs);

    
    return char_poly;
   
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
    nmod_poly_t  h;

    nmod_poly_factor_t facs;
    nmod_poly_factor_init(facs);
    
    nmod_poly_init(f, 2); // GF(2)
    nmod_poly_init(r, 2);
    nmod_poly_init(g, 2);
    nmod_poly_init(h, 2);

    size_t index = 0;

    for (index = 0; index < nb; index ++) {
        nmod_poly_set_coeff_ui(f, places[index], 1);
    }

    nmod_poly_set_coeff_ui(r, ring_n, 1);
    nmod_poly_set_coeff_ui(r, 0, 1);

    nmod_poly_gcd_euclidean(h, f, r);

    nmod_poly_div_basecase(g, r, h);

    nmod_poly_factor(facs, h); //the factorization

    int i;
    double mul = 0;


    for (i=0; i < facs->num; i++){

        mul = mul + facs->exp[i]*facs->exp[i];

    }

    mul = mul/facs->num;


    nmod_poly_clear(f);
    nmod_poly_clear(r);
    nmod_poly_clear(h);
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

    

    //get_polys(n,k);

    // nmod_poly_t  f; 
    // nmod_poly_init(f, 2); // GF(2)
    
    // nmod_poly_factor_t facs;
    // nmod_poly_factor_init(facs);

    // nmod_poly_set_coeff_ui(f, 0, 1);
    // nmod_poly_set_coeff_ui(f, 12, 1);

    // nmod_poly_factor(facs, f);

    // nmod_poly_factor_print(facs);


}