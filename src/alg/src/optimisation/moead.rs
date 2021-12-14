use crate::optimisation::operators::*;
use rand::prelude::*;
use std::f64;

pub fn run<
    X: Clone,
    Init: InitPop<X>,
    Mutate: Mutation<X>,
    Eval: Evaluate<X>,
    Cross: Crossover<X>,
>(
    init_pop: &Init,
    evaluate: &mut Eval,
    mutation: &Mutate,
    crossover: &Cross,
    mut iteration_observer: impl FnMut(usize, &Vec<Solution<X>>, &Vec<Constraint<Vec<f64>>>, bool),
    num_obj: usize,
    max_evaluations: usize,
    pop_size: usize,
    num_neighbours: usize,
) {
    let weight_vectors: Vec<Vec<f64>> = get_weights(pop_size, num_obj);
    let neighbours: Vec<Vec<usize>> = get_neighbours(num_neighbours, &weight_vectors, num_obj);

    let pop_size = weight_vectors.len();

    // Initial population
    let mut weights_pop = init_pop.apply(pop_size);

    let mut objectives: Vec<Constraint<Vec<f64>>> = weights_pop
        .iter_mut()
        .map(|ind| evaluate.apply(ind))
        .collect();

    let mut evaluations = weights_pop.len();
    iteration_observer(evaluations, &weights_pop, &objectives, false);

    let (mut ref_point, mut nadir_point) = get_ref_points(&objectives, num_obj);

    let mut rng = rand::thread_rng();

    while evaluations < max_evaluations {
        for i in 0..pop_size {
            if evaluations >= max_evaluations {
                break;
            }

            let n_a = rng.gen_range(0, num_neighbours);
            let n_b = rng.gen_range(0, num_neighbours);

            let ind_a = neighbours[i][n_a];
            let ind_b = neighbours[i][n_b];

            let children = crossover.apply(&weights_pop[ind_a], &weights_pop[ind_b]);
            let mut children: Vec<Solution<X>> = children
                .into_iter()
                .map(|ind| mutation.apply(&ind))
                .collect();
            let ch_objs: Vec<Constraint<Vec<f64>>> =
                children.iter_mut().map(|ind| evaluate.apply(ind)).collect();

            evaluations = evaluations + children.len();

            for n_id in &neighbours[i] {
                for j in 0..children.len() {
                    let ch_fit = tchebycheff(
                        &ch_objs[j],
                        &weight_vectors[*n_id],
                        &ref_point,
                        &nadir_point,
                    );

                    let cu_fit = tchebycheff(
                        &objectives[*n_id],
                        &weight_vectors[*n_id],
                        &ref_point,
                        &nadir_point,
                    );

                    if ch_fit < cu_fit {
                        weights_pop[*n_id] = children[j].clone();
                        objectives[*n_id] = ch_objs[j].clone();
                    }
                }
            }

            let (r, n) = get_ref_points(&objectives, num_obj);
            ref_point = r;
            nadir_point = n;
        }

        // iteration_observer(evaluations, &weights_pop, &objectives, false);
    }

    iteration_observer(evaluations, &weights_pop, &objectives, true);
}

fn get_neighbours(
    num_neighbours: usize,
    weight_vectors: &Vec<Vec<f64>>,
    num_obj: usize,
) -> Vec<Vec<usize>> {
    let mut neighbours: Vec<Vec<usize>> =
        vec![Vec::with_capacity(num_neighbours); weight_vectors.len()];

    for i in 0..weight_vectors.len() {
        let mut dists: Vec<(usize, f64)> = Vec::with_capacity(weight_vectors.len());

        for j in 0..weight_vectors.len() {
            // Calculate distance between vector i and j
            let mut dist = 0.0;
            for k in 0..num_obj {
                dist = dist + (weight_vectors[i][k] - weight_vectors[j][k]).powf(2.0);
            }
            dist = dist.sqrt();

            dists.push((j, dist));
        }

        dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let neighbour_idxs: Vec<usize> = dists[0..num_neighbours].iter().map(|x| x.0).collect();
        neighbours[i] = neighbour_idxs;
    }

    neighbours
}

fn get_weights(pop_size: usize, num_obj: usize) -> Vec<Vec<f64>> {
    if num_obj == 2 {
        let mut weights = Vec::with_capacity(pop_size);

        for i in 0..pop_size {
            let a = (i as f64) / (pop_size - 1) as f64;

            let weight = vec![a, 1.0 - a];

            let mag = weight.iter().map(|w| w.powf(2.0)).sum::<f64>().sqrt();
            let weight = weight.into_iter().map(|w| w / mag).collect();

            weights.push(weight);
        }

        return weights;
    } else if num_obj == 3 {
        let pop_size = pop_size as i32;

        let pop_to_h = vec![
            28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325,
            351, 378, 406, 435, 465, 496, 528, 561, 595,
        ];

        let mut dist = pop_size - pop_to_h[0];

        let mut i = 0;

        loop {
            let c_dist = (pop_size - pop_to_h[i]).abs();
            if c_dist < dist {
                dist = c_dist;
            }

            if c_dist > dist {
                break;
            }

            i = i + 1;
        }

        let h = i + 5;

        let mut weights = Vec::new();
        for i in 0..=h {
            for j in 0..=h {
                if i + j <= h {
                    let k = h - i - j;
                    let mut weight = Vec::with_capacity(num_obj);

                    weight.push(i as f64 / h as f64);
                    weight.push(j as f64 / h as f64);
                    weight.push(k as f64 / h as f64);

                    // Normalise weight
                    let mag = weight.iter().map(|w| w.powf(2.0)).sum::<f64>().sqrt();
                    let weight = weight.into_iter().map(|w| w / mag).collect();

                    weights.push(weight);
                }
            }
        }

        return weights;
    } else {
        unimplemented!()
    }
}

pub fn get_ref_points(
    objectives: &Vec<Constraint<Vec<f64>>>,
    num_obj: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut ref_point = vec![f64::MAX; num_obj];
    let mut nadir_point = vec![f64::MIN; num_obj];

    for obj in objectives {
        if !obj.is_feasible() {
            continue;
        }

        let obj = obj.unwrap();

        for i in 0..num_obj {
            if obj[i] < ref_point[i] {
                ref_point[i] = obj[i];
            }

            if obj[i] > nadir_point[i] {
                nadir_point[i] = obj[i];
            }
        }
    }

    (ref_point, nadir_point)
}

pub fn tchebycheff(
    objectives: &Constraint<Vec<f64>>,
    weights: &Vec<f64>,
    ref_point: &Vec<f64>,
    nadir_point: &Vec<f64>,
) -> f64 {
    let mut max = f64::MIN;

    if !objectives.is_feasible() {
        return f64::MAX;
    }

    let objectives = objectives.unwrap();

    for i in 0..objectives.len() {
        let dist =
            weights[i] * ((objectives[i] - ref_point[i]) / (nadir_point[i] - ref_point[i])).abs();

        if dist > max {
            max = dist;
        }
    }
    max
}