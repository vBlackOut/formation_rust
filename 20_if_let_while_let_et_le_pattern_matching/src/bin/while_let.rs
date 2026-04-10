#[derive(Debug)]
enum Task {
    Work(String),
    Eat,
    Sleep,
    Stop,
}

fn main() {
    let mut tasks = vec![
        Task::Work("Tâche 1".to_string()),
        Task::Work("Tâche 2".to_string()),
        Task::Eat,
        Task::Work("Tâche 3".to_string()),
        Task::Sleep,
        Task::Stop,
    ];

    while let Some(task) = tasks.pop() {
        println!("{:?}", tasks);
        match task {
            Task::Work(description) => {
                println!("Travail : {}", description);
            }
            Task::Eat => {
                println!("Manger.");
            }
            Task::Sleep => {
                println!("Dormir.");
            }
            Task::Stop => {
                println!("Tâche 'Stop' atteinte. Arrêt de l'exécution.");
                break;
            }
        }
    }
}
