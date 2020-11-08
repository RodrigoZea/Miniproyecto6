mod task1;
mod task2;
mod task3;

fn main() {

    // Task 1
    let mut pop = Vec::new();

    // Crear poblacion inicial con 100 individuos.
    for _i in 0..100 {
        pop.push(task1::Individual2D::new());
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

    // Task 2
    let mut pop2 = Vec::new();

    // Crear poblacion inicial con 100 individuos.
    for _i in 0..100 {
        pop2.push(task2::Individual2D::new());
    }
    
    // Se ordena la poblacion de acuerdo al fitness.
    pop2.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    // correr 1 millon de iteraciones
    for _i in 0..1000000 {
        
        // cada iteracion se crean 10 nuevos individuos y se "matan"
        // a los peores 10.
        for i in 0..10 {
            let new = pop2[i].offspring(&pop2[i + 1]);
            pop2.push(new);
        }

        pop2.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        // "matar" a los peores 10.
        for _i in 0..10 {
            pop2.remove(0);
        }
    }

    // Task 3
    let mut pop3 = Vec::new();

    // Crear poblacion inicial con 100 individuos.
    for _i in 0..100 {
        pop3.push(task3::Individual2D::new());
    }
    
    // Se ordena la poblacion de acuerdo al fitness.
    pop3.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    // correr 1 millon de iteraciones
    for _i in 0..1000000 {
        
        // cada iteracion se crean 10 nuevos individuos y se "matan"
        // a los peores 10.
        for i in 0..10 {
            let new = pop3[i].offspring(&pop3[i + 1]);
            pop3.push(new);
        }

        pop3.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        // "matar" a los peores 10.
        for _i in 0..10 {
            pop3.remove(0);
        }
    }

    // el mejor individuo esta en la ultima pos de la lista.
    println!("\nTask 1 Best individual:");
    println!("{:?}", pop[pop.len() - 1]);

    println!("\nTask 2 Best individual:");
    println!("{:?}", pop2[pop2.len() - 1]);

    println!("\nTask 3 Best individual:");
    println!("{:?}", pop3[pop3.len() - 1]);
}
