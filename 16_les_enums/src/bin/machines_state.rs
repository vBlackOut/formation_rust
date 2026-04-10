#[derive(Debug)]
enum MachineState {
    Idle,
    Running {
        current_job: String,
        progress: f32,
    },
    Maintenance {
        technician: String,
        estimated_completion_time: u32,
    },
    Error {
        error_code: u32,
        description: String,
    }
}

fn main(){

    let machine1 = MachineState::Running{current_job:String::from("Test server 2"), progress:55.0};
    let machine2 = MachineState::Maintenance{technician:String::from("Zorro de la vega"), estimated_completion_time:160};
    let machine3 = MachineState::Error{error_code:805, description:String::from("Service unavailed temporary")};
    let machine4 = MachineState::Idle;

    println!("Etat machine 1 = {:?}", describe_machine_state(&machine1));
    println!("Etat machine 2 = {:?}", describe_machine_state(&machine2));
    println!("Etat machine 3 = {:?}", describe_machine_state(&machine3));
    println!("Etat machine 4 = {:?}", describe_machine_state(&machine4));
}

// fonction plus avancer pour recupere l'etat d'un enum
// et traité le retour souhaiter en fonction de l'etat de la machine.
fn describe_machine_state(state: &MachineState) -> String {
    match state {
        MachineState::Running { current_job, progress } => {
            format!("Running: job = '{current_job}', progress = {progress:.2}%")
        }
        MachineState::Maintenance {
            technician, estimated_completion_time,
        } => format!(
            "Maintenance: technician = '{technician}', estimated completion = {estimated_completion_time} mins"
        ),
        MachineState::Error {
            error_code, description
        } => format!(
            "Error: code = {error_code}, description = '{description}'",
        ),
        MachineState::Idle => "Idle".to_string(),
    }
}
