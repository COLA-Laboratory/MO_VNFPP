use crate::optimisation::operators::*;
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
    k: f64, // IBEA specific parameter
) {
    let mut pop = init_pop.apply(pop_size);
    let mut child_pop: Vec<Solution<X>> = Vec::with_capacity(pop_size);

    let mut objectives = Vec::with_capacity(pop_size);

    let mut evaluations;

    for ind in pop.iter_mut() {
        objectives.push(evaluate.apply(ind));
    }
    evaluations = pop.len();

    // Initial population
    iteration_observer(evaluations, &pop, &objectives, false);

    loop {
        // Calculate solution fitness
        let mut fitnesses = vec![0.0; pop.len()];

        for i in 0..pop.len() {
            let obj_i = &objectives[i];
            if !obj_i.is_feasible() {
                fitnesses[i] = std::f64::NEG_INFINITY;
                continue;
            }

            let mut fitness = 0.0;
            for j in 0..pop.len() {
                if i == j {
                    continue;
                }

                let obj_j = &objectives[j];
                if obj_j.is_feasible() {
                    fitness += -(-add_eps_indicator(&obj_i.unwrap(), &obj_j.unwrap()) / k).exp();
                }
            }

            fitnesses[i] = fitness;
        }

        while pop.len() > pop_size {
            let worst_idx = get_worst_ind(&fitnesses, &objectives);

            let worst_objs = &objectives[worst_idx];
            if worst_objs.is_feasible() {
                // Update fitness values of the remaining individuals
                for i in 0..pop.len() {
                    let objs_i = &objectives[i];
                    if !objs_i.is_feasible() {
                        continue;
                    }

                    fitnesses[i] +=
                        (-add_eps_indicator(&objs_i.unwrap(), &worst_objs.unwrap()) / k).exp()
                }
            }

            pop.remove(worst_idx);
            fitnesses.remove(worst_idx);
            objectives.remove(worst_idx);
        }

        if evaluations >= max_evaluations {
            break;
        }

        let ts = TournamentSelection::new(pop.len(), |x, y| fitnesses[x] > fitnesses[y]);

        while child_pop.len() < pop_size {
            let parent_one = ts.tournament(2);
            let parent_two = ts.tournament(2);

            let new_children = crossover.apply(&pop[parent_one], &pop[parent_two]);
            for child in new_children {
                let mut child = mutation.apply(&child);

                objectives.push(evaluate.apply(&mut child));
                evaluations = evaluations + 1;

                child_pop.push(child);
            }
        }

        pop.append(&mut child_pop);
    }

    iteration_observer(evaluations, &pop, &objectives, true);
}

pub fn get_worst_ind(fitness: &Vec<f64>, objectives: &Vec<Constraint<Vec<f64>>>) -> usize {
    let mut min_idx = 0;
    let mut min_fit = f64::MAX;

    for (i, fit) in fitness.iter().enumerate() {
        if !objectives[i].is_feasible() {
            return i;
        }

        if fit < &min_fit {
            min_idx = i;
            min_fit = *fit;
        }
    }

    min_idx
}

pub fn add_eps_indicator(a_objs: &Vec<f64>, b_objs: &Vec<f64>) -> f64 {
    let mut eps = f64::NEG_INFINITY;

    for i in 0..a_objs.len() {
        eps = f64::max(eps, b_objs[i] - a_objs[i]);
    }

    eps
}

pub fn norm_objectives(objectives: &Vec<f64>, max_obj: &Vec<f64>, min_obj: &Vec<f64>) -> Vec<f64> {
    objectives
        .iter()
        .enumerate()
        .map(|(i, x)| (x - min_obj[i]) / (max_obj[i] - min_obj[i]))
        .collect()
}
