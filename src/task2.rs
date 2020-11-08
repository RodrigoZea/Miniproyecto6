use rand::Rng;

/**
 * Representacion de Individuo de 2 Dimensiones
 */
#[derive(Debug)]
pub struct Individual2D {
    fitness: f64,
    x: f64,
    y: f64
}

impl Individual2D {

    /**
     * Crea un nuevo individuo, con sus fenotipos iniciados a un valor
     * aleatorio entre [0, 1). 
     */
    pub fn new() -> Individual2D {
        let mut rng = rand::thread_rng();
        let mut i = Individual2D {
            fitness: 0.0,
            x: rng.gen(),
            y: rng.gen(),
        };

        i.update_fitness();
        
        return i;
    }

    /**
     * Crea un nuevo individuo, con los valores especificados de "x" y "y",
     * y calcula el fitness del individuo.
     */ 
    pub fn new_with_chr(x: f64, y: f64) -> Individual2D {
        let mut i = Individual2D { fitness: 0.0, x, y };
        i.update_fitness();
        
        return i;
    }

    /**
     * Provee acceso de lectura al fitness del individuo.
     */
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    /**
     * Funcion de fitness para Task 2.
     * Es una funcion privada.
     */
    fn update_fitness(&mut self) {
        self.fitness = 3.0 * self.x + 5.0 * self.y
    }

    /**
     * Operador de crossover, devuelve un nuevo individuo cuyos padres son los
     * especificados en los argumentos.
     * Es una funcion privada.
     */
    fn crossover(&self, other: &Individual2D) -> Individual2D {
        let mut i = Individual2D {
            fitness: 0.0,
            x: self.x + other.y,
            y: self.y + other.x
        };

        while i.x >= 4.0 {
            i.x = i.x / 2.0;
        }

        while (2.0 * i.y) >= 12.0 {
            i.y = i.y / 2.0;
        }

        while (3.0 * i.x + 2.0 * i.y) >= 18.0 {
            i.x = i.x / 2.0;
            i.y = i.y / 2.0;
        }

        return i;
    }

    /**
     * Muta el cromosoma del individuo especificado con un 50% de probabilidad.
     * Es una funcion privada.
     */
    fn mutate(&mut self) {
        if rand::random() {
            let mut rng = rand::thread_rng();
            self.x += rng.gen_range(-5.0, 5.0);
            self.y += rng.gen_range(-5.0, 5.0);
        }
        
        while self.x >= 4.0 {
            self.x = self.x / 2.0;
        }

        while (2.0 * self.y) >= 12.0 {
            self.y = self.y / 2.0;
        }

        while (3.0 * self.x + 2.0 * self.y) >= 18.0 {
            self.x = self.x / 2.0;
            self.y = self.y / 2.0;
        }
    }

    /**
     * Crea un nuevo individuo a partir de los padres especificados. Utilizar este metodo
     * para crear un nuevo individuo.
     */
    pub fn offspring(&self, other: &Individual2D) -> Individual2D {
        let mut new = self.crossover(other);
        new.mutate();
        new.update_fitness();

        return new;
    }
}
