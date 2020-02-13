pub fn poly_to_edgelist(coefs: &[usize], nb_bits: usize) -> (Vec<usize>,Vec<usize>) { //assumes right circulant
    
    let weight = coefs.len();
    let nb_checks = nb_bits; // statng it explicitely for clarity
    let mut v1 = Vec::with_capacity(nb_checks*weight);
    let mut v2 = Vec::with_capacity(nb_checks*weight);

    for i in 0..nb_checks {
        for c in coefs {
            v1.push(i+1);
            v2.push(nb_checks + (c+i)%nb_bits +1);
        }
    }

    (v1,v2)
    
}

pub fn poly_to_edgelist_3(i_0: usize, i_1: usize, nb_bits: usize) -> (Vec<usize>,Vec<usize>) { //assumes right circulant
    
    let weight = 3;
    let nb_checks = nb_bits; // statng it explicitely for clarity
    let mut v1 = Vec::with_capacity(nb_checks*weight);
    let mut v2 = Vec::with_capacity(nb_checks*weight);

    for i in 0..nb_checks {
        v1.push(i+1);
        v2.push(nb_checks + (0+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_0+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_1+i)%nb_bits +1);
        
    }

    (v1,v2)
    
}

pub fn poly_to_edgelist_5(i_0: usize, i_1: usize, i_2: usize, i_3: usize, nb_bits: usize) -> (Vec<usize>,Vec<usize>) { //assumes right circulant
    
    let weight = 3;
    let nb_checks = nb_bits; // statng it explicitely for clarity
    let mut v1 = Vec::with_capacity(nb_checks*weight);
    let mut v2 = Vec::with_capacity(nb_checks*weight);

    for i in 0..nb_checks {
        v1.push(i+1);
        v2.push(nb_checks + (0+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_0+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_1+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_2+i)%nb_bits +1);

        v1.push(i+1);
        v2.push(nb_checks + (i_3+i)%nb_bits +1);
    }

    (v1,v2)
    
}