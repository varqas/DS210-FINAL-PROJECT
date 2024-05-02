use rand::Rng;
use rand::seq::SliceRandom;

// I'm using my previous pagerank function from HW10 as a base for implementing pagerank for this program. Thus, the code is incomplete for now.

pub fn pagerank(graph: &Vec<Vec<usize>>, num_nodes:usize) -> Vec<(usize, f64)> {
    let mut page_values: Vec<(usize, usize)> = (0..num_nodes).map(|init_vertex: usize| (init_vertex, 0)).collect();
    let mut rank_values: Vec<(usize, f64)> = Vec::new();
    for init_vertex in 0..num_nodes {
        for _num_walks in 0..100 {
            let mut current_vertex = init_vertex;
            for _num_steps in 0..100 {
                    if rand::thread_rng().gen::<f64>() < 0.9 && !graph[current_vertex].is_empty() {
                        current_vertex = *graph[current_vertex].choose(&mut rand::thread_rng()).unwrap() as usize;
                    }
                    else {
                        current_vertex = rand::thread_rng().gen_range(0..num_nodes);
                    }
            }
            page_values[current_vertex].1 += 1;
        } 
    }
   
    return  page_values.into_iter().map(|(index, count)| (index, (count as f64/ (100 * num_nodes) as f64))).collect();
   
}
