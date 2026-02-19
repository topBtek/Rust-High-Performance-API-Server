use criterion::{criterion_group, criterion_main, Criterion};
use rust_high_performance_api_server::{models::Task, state::AppState};
use std::sync::Arc;

fn bench_task_creation(c: &mut Criterion) {
    let state = AppState::new();
    
    c.bench_function("create_task", |b| {
        b.iter(|| {
            let task = Task::new("Benchmark Task".to_string(), None);
            state.tasks.insert(task.id, task);
        });
    });
}

fn bench_task_lookup(c: &mut Criterion) {
    let state = AppState::new();
    
    // Pre-populate with tasks
    let mut ids = Vec::new();
    for i in 0..1000 {
        let task = Task::new(format!("Task {}", i), None);
        ids.push(task.id);
        state.tasks.insert(task.id, task);
    }
    
    c.bench_function("lookup_task", |b| {
        b.iter(|| {
            for id in &ids[..100] {
                let _ = state.tasks.get(id);
            }
        });
    });
}

criterion_group!(benches, bench_task_creation, bench_task_lookup);
criterion_main!(benches);
