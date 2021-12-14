use crate::optimisation::operators::*;
use std::cmp::Ordering;
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
    max_evaluations: usize,
    pop_size: usize,
) {
    let mut parent_pop: Vec<NSGAII_Solution<X>> = init_pop
        .apply(pop_size)
        .into_iter()
        .map(|solution| NSGAII_Solution::new(solution))
        .collect();

    parent_pop
        .iter_mut()
        .for_each(|ind| ind.objectives = evaluate.apply(&mut ind.solution));

    let mut evaluations = parent_pop.len();

    // Initial population
    iteration_observer(
        evaluations,
        &parent_pop.iter().map(|ind| ind.solution.clone()).collect(),
        &parent_pop
            .iter()
            .map(|ind| ind.objectives.clone())
            .collect(),
        false,
    );

    let mut child_pop = Vec::with_capacity(pop_size);
    let mut combined_pop = Vec::with_capacity(pop_size * 2);

    while evaluations < max_evaluations {
        combined_pop.append(&mut parent_pop);
        combined_pop.append(&mut child_pop);

        let mut nondominated_fronts = fast_nondominated_sort(&mut combined_pop);

        for front in &mut nondominated_fronts {
            crowding_distance_assignment(front);
        }

        let mut i = 0;
        while parent_pop.len() + nondominated_fronts[i].len() < pop_size {
            parent_pop.append(&mut nondominated_fronts[i]);
            i = i + 1;
        }

        nondominated_fronts[i].sort_by(|x, y| crowding_comparison_operator(x, y));

        let mut j = 0;
        while parent_pop.len() < pop_size {
            parent_pop.push(nondominated_fronts[i][j].clone());
            j = j + 1;
        }

        let ts = TournamentSelection::new(parent_pop.len(), |x, y| {
            crowding_comparison_operator(&parent_pop[x], &parent_pop[y]) == Ordering::Less
        });

        while child_pop.len() < pop_size {
            let parent_one = ts.tournament(2);
            let parent_two = ts.tournament(2);

            let new_children = crossover.apply(
                &parent_pop[parent_one].solution,
                &parent_pop[parent_two].solution,
            );

            for child in new_children {
                let solution = mutation.apply(&child);
                child_pop.push(NSGAII_Solution::new(solution));
            }
        }

        child_pop
            .iter_mut()
            .for_each(|ind| ind.objectives = evaluate.apply(&mut ind.solution));

        evaluations = evaluations + child_pop.len();
        combined_pop.clear();

        // iteration_observer(
        //     evaluations,
        //     &parent_pop.iter().map(|ind| ind.solution.clone()).collect(),
        //     &parent_pop
        //         .iter()
        //         .map(|ind| ind.objectives.clone())
        //         .collect(),
        //     false,
        // );
    }

    iteration_observer(
        evaluations,
        &parent_pop.iter().map(|ind| ind.solution.clone()).collect(),
        &parent_pop
            .iter()
            .map(|ind| ind.objectives.clone())
            .collect(),
        true,
    );
}

fn fast_nondominated_sort<X: Clone>(
    pop: &mut Vec<NSGAII_Solution<X>>,
) -> Vec<Vec<NSGAII_Solution<X>>> {
    let mut dominates = Vec::with_capacity(pop.len());
    let mut dom_counted = Vec::with_capacity(pop.len());

    let mut ranks = vec![Vec::new()];
    let mut output = vec![Vec::new()];

    for p in 0..pop.len() {
        let mut p_dominates = Vec::new();
        let mut dom_count = 0;

        for q in 0..pop.len() {
            if pop[p].dominates(&pop[q]) {
                p_dominates.push(q);
            } else if pop[q].dominates(&pop[p]) {
                dom_count = dom_count + 1;
            }
        }

        if dom_count == 0 {
            pop[p].rank = 0;
            ranks[0].push(p);

            output[0].push(pop[p].clone());
        }

        dominates.push(p_dominates);
        dom_counted.push(dom_count);
    }

    let mut i = 0;
    while !ranks[i].is_empty() {
        let mut next_rank = Vec::new();
        let mut next_output = Vec::new();

        for p in &ranks[i] {
            for q in &dominates[*p] {
                dom_counted[*q] -= 1;

                if dom_counted[*q] == 0 {
                    pop[*q].rank = i + 1;
                    next_rank.push(*q);

                    next_output.push(pop[*q].clone());
                }
            }
        }

        i = i + 1;
        ranks.push(next_rank);
        output.push(next_output);
    }

    output
}

fn crowding_distance_assignment<X: Clone>(pop: &mut [NSGAII_Solution<X>]) {
    // If population is empty or all infeasible
    if !pop.iter().any(|ind| ind.objectives.is_feasible()) {
        return;
    }

    let num_obj = pop[0].objectives.unwrap().len();

    for ind in pop.iter_mut() {
        ind.crowding_dist = 0.0;
    }

    let mut idxs: Vec<usize> = (0..pop.len()).into_iter().collect();

    for m in 0..num_obj {
        idxs.sort_by(|&x, &y| {
            pop[x].objectives.unwrap()[m]
                .partial_cmp(&pop[y].objectives.unwrap()[m])
                .unwrap()
        });

        let l = pop.len() - 1;

        let min_idx = idxs[0];
        let max_idx = idxs[l];

        pop[min_idx].crowding_dist = f64::INFINITY;
        pop[max_idx].crowding_dist = f64::INFINITY;

        let obj_min = pop[min_idx].objectives.unwrap()[m];
        let obj_max = pop[max_idx].objectives.unwrap()[m];

        let diff = if obj_min == obj_max {
            1.0
        } else {
            obj_max - obj_min
        };

        if l <= 1 {
            continue;
        }

        for i in 1..l {
            let curr = idxs[i];
            let next = idxs[i + 1];
            let pre = idxs[i - 1];

            pop[curr].crowding_dist +=
                (pop[next].objectives.unwrap()[m] - pop[pre].objectives.unwrap()[m]) / diff;
        }
    }
}

fn crowding_comparison_operator<X: Clone>(
    ind_a: &NSGAII_Solution<X>,
    ind_b: &NSGAII_Solution<X>,
) -> Ordering {
    if ind_a.rank < ind_b.rank
        || (ind_a.rank == ind_b.rank && ind_a.crowding_dist > ind_b.crowding_dist)
    {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// Wrapper around Solution struct with extra information for NSGA-II
#[derive(Clone)]
#[allow(non_camel_case_types)]
struct NSGAII_Solution<X: Clone> {
    solution: Solution<X>,
    objectives: Constraint<Vec<f64>>,
    crowding_dist: f64,
    rank: usize,
}

impl<X: Clone> NSGAII_Solution<X> {
    pub fn new(solution: Solution<X>) -> NSGAII_Solution<X> {
        NSGAII_Solution {
            solution,
            objectives: Constraint::Infeasible, // Infeasible until proven otherwise
            crowding_dist: 0.0,
            rank: 0,
        }
    }

    pub fn dominates(&self, other: &NSGAII_Solution<X>) -> bool {
        // Infeasible solutions are dominated by any feasible one
        if !self.objectives.is_feasible() {
            return false;
        }

        if !&other.objectives.is_feasible() {
            return true;
        }

        // Domination: Equal to or better than in all objective and strictly better than in one
        let self_obj = self.objectives.unwrap();
        let other_obj = other.objectives.unwrap();

        let num_obj = self_obj.len();

        let mut num_better = 0;
        let mut num_worse = 0;

        for i in 0..num_obj {
            if self_obj[i] < other_obj[i] {
                num_better = num_better + 1;
            } else if self_obj[i] > other_obj[i] {
                num_worse = num_worse + 1;
            }
        }

        num_better > 0 && num_worse == 0
    }
}
