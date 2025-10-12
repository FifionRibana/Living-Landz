use legion::*;
use crate::world::components::*;

/// Système de production de ressources
#[system]
pub fn production(
    #[resource] tick: &u64,
    query: &mut Query<(&Building, &Position)>,
) {
    for (building, pos) in query.iter() {
        // Production selon type de bâtiment
        match building.building_type {
            BuildingType::Farm => {
                // Produire nourriture
            },
            BuildingType::Mine => {
                // Extraire minerai
            },
            _ => {}
        }
    }
}

/// Système de consommation (unités mangent)
#[system]
pub fn consumption(
    #[resource] tick: &u64,
    query: &mut Query<&mut Unit>,
) {
    for mut unit in query.iter_mut() {
        // Consommer nourriture depuis inventaire
        // Réduire bonheur si pas assez
    }
}

/// Système de construction (avancement)
#[system]
pub fn construction(
    #[resource] tick: &u64,
    query: &mut Query<&mut Building>,
) {
    for mut building in query.iter_mut() {
        if building.construction_progress < 1.0 {
            // Avancer construction
            building.construction_progress += 0.1; // 10% par tick
            building.construction_progress = building.construction_progress.min(1.0);
        }
    }
}