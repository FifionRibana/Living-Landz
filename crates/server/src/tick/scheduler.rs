use std::time::{Duration, Instant};
use bevy::prelude::World;

pub struct TickScheduler {
    pub tick_interval: Duration,
    pub current_tick: u64,
    pub last_tick: Instant,
}

impl TickScheduler {
    pub fn new(interval_seconds: u64) -> Self {
        Self {
            tick_interval: Duration::from_secs(interval_seconds),
            current_tick: 0,
            last_tick: Instant::now(),
        }
    }
    
    pub fn should_tick(&self) -> bool {
        Instant::now() - self.last_tick >= self.tick_interval
    }
    
    pub fn execute_tick(&mut self, world: &mut World) {
        self.current_tick += 1;
        self.last_tick = Instant::now();
        
        tracing::info!("Executing tick {}", self.current_tick);
        
        // Exécuter tous les systèmes
        // run_production_system(world);
        // run_consumption_system(world);
        // run_construction_system(world);
        // run_market_system(world);
        // ... autres systèmes
    }
    
    pub fn time_until_next_tick(&self) -> Duration {
        let elapsed = Instant::now() - self.last_tick;
        self.tick_interval.saturating_sub(elapsed)
    }
}