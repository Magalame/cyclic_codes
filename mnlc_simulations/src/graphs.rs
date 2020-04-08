use pagenumber::*;

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

pub fn poly_to_edgelist_pg(coefs: &[usize], nb_bits: usize) -> (Vec<Edge>,Vec<Vertex>) { //edgelist for `pagenumber`
    
    // println!("coef:{:?}",coefs);
    let weight = coefs.len();
    let nb_checks = nb_bits; // statng it explicitely for clarity
    // let mut v1 = Vec::with_capacity(nb_checks*weight);
    // let mut v2 = Vec::with_capacity(nb_checks*weight);

    let vertices = (0..2*nb_checks).map(|x| Vertex(x)).collect();// we have as many bits as we have checks for cyclic codes
    let mut edges = Vec::with_capacity(nb_checks*weight);

    // note that there is a +1 offset between this version and the rest, I'm not sure why I index from 1 for the other, but it doesn't matter anyway
    for i in 0..nb_checks {
        for c in coefs {
            let v1 = Vertex(i);
            let v2 = Vertex(nb_checks + (c+i)%nb_bits);
            edges.push(Edge(v1,v2));
            // println!("E:{:?}",Edge(v1,v2));
        }
    }

    (edges,vertices)
    
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