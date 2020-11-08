mod functions_2d;

fn main() {

    // Task 1
    let mut pop = Vec::new();

    // Crear poblacion inicial con 100 individuos.
    for _i in 0..100 {
        pop.push(functions_2d::Individual2D::new());
    }
    
    // Se ordena la poblacion de acuerdo al fitness.
    pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    // correr 1 millon de iteraciones
    for _i in 0..1000000 {
        
        // cada iteracion se crean 10 nuevos individuos y se "matan"
        // a los peores 10.
        for i in 0..10 {
            let new = pop[i].offspring(&pop[i + 1]);
            pop.push(new);
        }

        pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        // "matar" a los peores 10.
        for _i in 0..10 {
            pop.remove(0);
        }
    }

    // el mejor individuo esta al frente de la lista.
    println!("\nTask 1 Best individual:");
    println!("{:?}", pop[pop.len() - 1]);
}
