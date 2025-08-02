use cj_bitmask_vec::prelude::BitmaskVec;

const QUEUED: u8 = 0b00000001;
const RUNNING: u8 = 0b00000010;
const COMPLETED: u8 = 0b00000100;
const FAILED: u8 = 0b00001000;

fn main() {
    let mut tasks = BitmaskVec::<u8, String>::new();

    init_tasks(&mut tasks);
    dispatch_tasks(&mut tasks);
    check_completion(&mut tasks);

    for task in tasks.filtered(&FAILED) {
        println!("Uh oh SpaghettiOs: {} Failed!", task);
    }
}

fn init_tasks(tasks: &mut BitmaskVec<u8, String>) {
    tasks.push_with_mask(QUEUED, "Task 1".to_string());
    tasks.push_with_mask(QUEUED, "Task 2".to_string());
    tasks.push_with_mask(QUEUED, "Task 3".to_string());
    tasks.push_with_mask(QUEUED, "Task 4".to_string());
    tasks.push_with_mask(QUEUED, "Task 5".to_string());
    tasks.push_with_mask(QUEUED, "Task 6".to_string());
}

fn dispatch_tasks(tasks: &mut BitmaskVec<u8, String>) {
    for task in tasks.filtered_with_mask_mut(&QUEUED) {
        // Simulate dispatched tasks
        task.set_mask(RUNNING);
        println!("Dispatching: {}", task.item);
    }
}

fn check_completion(tasks: &mut BitmaskVec<u8, String>) {
    let mut completed = true;
    let mut failed = false;

    for task in tasks.filtered_with_mask_mut(&RUNNING) {
        // Simulate completion
        if completed {
            task.set_mask(COMPLETED);

            if failed {
                task.bitmask = COMPLETED | FAILED;
                println!("Failed: {}", task.item);
            } else {
                println!("Completed: {}", task.item);
            }

            failed = !failed;
        }

        completed = !completed;
    }
}
