use cj_bitmask_vec::prelude::*;
use cj_common::prelude::Bitflag;

#[test]
fn test_real_world_game_entities() {
    // Simulate a game entity system using bitmasks for component flags
    let mut entities = BitmaskVec::<u32, String>::new();
    
    // Component flags
    const POSITION: u32 = 0b00000001;
    const VELOCITY: u32 = 0b00000010;
    const RENDER: u32 = 0b00000100;
    const HEALTH: u32 = 0b00001000;
    const AI: u32 = 0b00010000;
    
    // Add various entities
    entities.push_with_mask(POSITION | RENDER, "Static Object".to_string());
    entities.push_with_mask(POSITION | VELOCITY | RENDER, "Moving Object".to_string());
    entities.push_with_mask(POSITION | VELOCITY | RENDER | HEALTH | AI, "Enemy".to_string());
    entities.push_with_mask(POSITION | VELOCITY | RENDER | HEALTH, "Player".to_string());
    entities.push_with_mask(RENDER, "UI Element".to_string());
    
    assert_eq!(entities.len(), 5);
    
    // Find all entities that can move (have VELOCITY)
    let mut moving_entities = 0;
    for entity in entities.iter_with_mask() {
        if entity.matches_mask(&VELOCITY) {
            moving_entities += 1;
        }
    }
    assert_eq!(moving_entities, 3);
    
    // Find all entities that need rendering and have position
    let mut renderable_positioned = 0;
    for entity in entities.iter_with_mask() {
        if entity.matches_mask(&(RENDER | POSITION)) {
            renderable_positioned += 1;
        }
    }
    assert_eq!(renderable_positioned, 4);
    
    // Remove an entity and verify
    let removed = entities.remove_with_mask(2);
    assert_eq!(removed.item, "Enemy");
    assert_eq!(entities.len(), 4);
}

#[test]
fn test_permission_system() {
    // Simulate a permission system using bitmasks
    let mut users = BitmaskVec::<u16, String>::new();
    
    // Permission flags
    const READ: u16 = 0b0001;
    const WRITE: u16 = 0b0010;
    const EXECUTE: u16 = 0b0100;
    const ADMIN: u16 = 0b1000;
    
    // Add users with different permissions
    users.push_with_mask(READ, "guest".to_string());
    users.push_with_mask(READ | WRITE, "user".to_string());
    users.push_with_mask(READ | WRITE | EXECUTE, "developer".to_string());
    users.push_with_mask(READ | WRITE | EXECUTE | ADMIN, "admin".to_string());
    
    // Count users with write permissions
    let mut writers = 0;
    for user in users.iter_with_mask() {
        if user.matches_mask(&WRITE) {
            writers += 1;
        }
    }
    assert_eq!(writers, 3);
    
    // Find admin users
    let mut admins = Vec::new();
    for user in users.iter_with_mask() {
        if user.matches_mask(&ADMIN) {
            admins.push(user.item.clone());
        }
    }
    assert_eq!(admins, vec!["admin"]);
    
    // Modify permissions
    for user in users.iter_with_mask_mut() {
        if user.item == "developer" {
            user.bitmask.set_bit(3, true); // Grant admin
        }
    }
    
    // Verify admin count increased
    let mut admin_count = 0;
    for user in users.iter_with_mask() {
        if user.matches_mask(&ADMIN) {
            admin_count += 1;
        }
    }
    assert_eq!(admin_count, 2);
}

#[test]
fn test_complex_filtering_operations() {
    let mut data = BitmaskVec::<u8, i32>::new();
    
    // Add test data with various bit patterns
    for i in 0..100 {
        data.push_with_mask(i as u8, i);
    }
    
    // Complex filtering: find numbers where bits 0, 2, and 4 are set
    let pattern = 0b00010101u8;
    let mut matching_values = Vec::new();
    
    for item in data.iter_with_mask() {
        if item.matches_mask(&pattern) {
            matching_values.push(item.item);
        }
    }
    
    // Verify the filtering worked correctly
    for &value in &matching_values {
        let mask = value as u8;
        assert!(mask & pattern == pattern);
    }
    
    // Test filter_mask iterator method
    let mut filter_count = 0;
    let mut iter = data.iter_with_mask();
    while let Some(_) = iter.filter_mask(&pattern) {
        filter_count += 1;
    }
    
    assert_eq!(filter_count, matching_values.len());
}

#[test]
fn test_large_scale_operations() {
    let mut large_vec = BitmaskVec::<u64, usize>::new();
    
    // Add 10,000 elements
    for i in 0..10_000 {
        large_vec.push_with_mask(i as u64, i);
    }
    
    assert_eq!(large_vec.len(), 10_000);
    
    // Test bulk operations
    let mut other_vec = BitmaskVec::<u64, usize>::new();
    for i in 10_000..15_000 {
        other_vec.push_with_mask(i as u64, i);
    }
    
    large_vec.append(&mut other_vec);
    assert_eq!(large_vec.len(), 15_000);
    assert_eq!(other_vec.len(), 0);
    
    // Test large-scale filtering
    let mut even_count = 0;
    for item in large_vec.iter_with_mask() {
        if item.matches_mask(&0b1u64) { // Check if odd (bit 0 set)
            // This is actually checking for odd numbers
        } else {
            even_count += 1; // Even numbers (bit 0 not set)
        }
    }
    
    // Should have roughly half even numbers
    assert!(even_count > 7000 && even_count < 8000);
    
    // Test truncation on large collection
    large_vec.truncate(5000);
    assert_eq!(large_vec.len(), 5000);
}

#[test]
fn test_mixed_operations_workflow() {
    let mut workflow = BitmaskVec::<u8, String>::new();
    
    // Simulate a workflow with different states
    const PENDING: u8 = 0b001;
    const PROCESSING: u8 = 0b010;
    const COMPLETED: u8 = 0b100;
    const ERROR: u8 = 0b1000;
    
    // Add initial tasks
    workflow.push_with_mask(PENDING, "Task 1".to_string());
    workflow.push_with_mask(PENDING, "Task 2".to_string());
    workflow.push_with_mask(PENDING, "Task 3".to_string());
    
    // Start processing some tasks
    for task in workflow.iter_with_mask_mut() {
        if task.item.contains("Task 1") || task.item.contains("Task 2") {
            task.bitmask = PROCESSING;
        }
    }
    
    // Complete one task, error on another
    for task in workflow.iter_with_mask_mut() {
        if task.item == "Task 1" {
            task.bitmask = COMPLETED;
        } else if task.item == "Task 2" {
            task.bitmask = ERROR;
        }
    }
    
    // Count tasks in each state
    let mut pending = 0;
    let mut processing = 0;
    let mut completed = 0;
    let mut error = 0;
    
    for task in workflow.iter_with_mask() {
        if task.matches_mask(&PENDING) { pending += 1; }
        if task.matches_mask(&PROCESSING) { processing += 1; }
        if task.matches_mask(&COMPLETED) { completed += 1; }
        if task.matches_mask(&ERROR) { error += 1; }
    }
    
    assert_eq!(pending, 1);
    assert_eq!(processing, 0);
    assert_eq!(completed, 1);
    assert_eq!(error, 1);
    
    // Add more tasks dynamically
    workflow += (PENDING, "Task 4".to_string());
    workflow += (PROCESSING, "Task 5".to_string());
    
    assert_eq!(workflow.len(), 5);
}

#[test]
fn test_memory_efficiency() {
    // Test that the collection properly manages memory
    let mut vec = BitmaskVec::<u32, Vec<u8>>::new();
    
    // Add elements that would use significant memory
    for i in 0..1000 {
        let data = vec![i as u8; 100]; // 100 bytes per element
        vec.push_with_mask(i, data);
    }
    
    let initial_len = vec.len();
    assert_eq!(initial_len, 1000);
    
    // Remove half the elements
    for _ in 0..500 {
        vec.pop();
    }
    
    assert_eq!(vec.len(), 500);
    
    // Clear and verify empty
    vec.clear();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_iterator_combinations() {
    let mut data = BitmaskVec::<u8, i32>::new();
    
    // Setup test data
    for i in 0..20 {
        data.push_with_mask(i as u8, i);
    }
    
    // Test chaining different iterator operations
    let sum1: i32 = data.iter().sum();
    assert_eq!(sum1, (0..20).sum());
    
    // Test iter_with_mask combined with filtering
    let mut filtered_sum = 0;
    for item in data.iter_with_mask() {
        if item.matches_mask(&0b00000001u8) { // Odd numbers
            filtered_sum += item.item;
        }
    }
    
    let expected_odd_sum: i32 = (0..20).filter(|x| x % 2 == 1).sum();
    assert_eq!(filtered_sum, expected_odd_sum);
    
    // Test mutable iteration with modifications
    for item in data.iter_mut() {
        *item *= 2;
    }
    
    let doubled_sum: i32 = data.iter().sum();
    assert_eq!(doubled_sum, sum1 * 2);
}