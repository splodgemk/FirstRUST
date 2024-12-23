use bevy::{prelude::*, ecs::query::With};
use rand::Rng;

//COMPONENTS
//Units
#[derive(Component)]
struct SpaceMarine;

#[derive(Component)]
struct Tyranid;

//Units Stats
#[derive(Component)]
struct Stats{
    movement: u8,
    toughness: u8,
    wounds: u8,
    save: u8,
    leadership: u8,
}

//Weapon stats
#[derive(Component)]
struct Weapon{
    name: String,
    range: u8,
    attacks: u8,
    ap: u8,
    damage:u8,
}


fn main() {
    println!("Game Starting...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, combat_system)
        .run();
}

fn setup(mut commands: Commands){
    //Space Marines
    for _ in 0..5 {
        commands.spawn((
            SpaceMarine, // Standard Space Marine
            Stats {
                movement: 6,
                toughness: 4,
                wounds: 2,
                save: 3,
                leadership: 6,
            },
            Weapon {
                name: "Bolt Rifle".to_string(),
                range: 24,
                attacks: 2,
                ap: 0,
                damage: 1,
            },
        ));
    }

    //Tyranids
    for _ in 0..10 {
        commands.spawn((
            Tyranid, // Stats for a Tyranid Warrior
            Stats{ 
                movement:7,
                toughness: 5,
                wounds:3,
                save:4,
                leadership:8,
            },
            Weapon {
                name: "Sything Talons".to_string(),
                range:0,
                attacks:3,
                ap:1,
                damage:1,
            },
        ));
    }
}

fn combat_system(
    mut commands: Commands,
    marine_query: Query<(Entity, &Stats, &Weapon), With<SpaceMarine>>,
    mut tyranid_query: Query<(Entity, &mut Stats, &Weapon), With<Tyranid>>,
) {
    let mut rng = rand::thread_rng();

    // Iterate over the Space Marines
    for (marine_entity, marine_stats, marine_weapon) in marine_query.iter() {
        // Iterate over the Tyranids
        for (tyranid_entity, mut tyranid_stats, _) in tyranid_query.iter_mut() {
            println!("Space Marine shot Tyranid with {}!", marine_weapon.name);

            for _ in 0..marine_weapon.attacks {
                let roll_to_hit = rng.gen_range(1..=6);
                if roll_to_hit >= 3 {
                    println!("{}. HIT! Rolling to wound...", roll_to_hit);

                    let roll_to_wound = rng.gen_range(1..=6);
                    if roll_to_wound >= 4 {
                        println!("{}. WOUND! Rolling to save...", roll_to_wound);
                    
                        let roll_to_save = rng.gen_range(1..=6);
                        if roll_to_save < tyranid_stats.save {
                            println!("Save failed! Tyranid takes damage!");
                            let remaining_wounds = tyranid_stats.wounds.saturating_sub(marine_weapon.damage);
                            if remaining_wounds <= 0 {
                                println!("Tyranid is now dead!");
                                commands.entity(tyranid_entity).despawn();
                            } else {
                                tyranid_stats.wounds = remaining_wounds;
                            }
                        } else {
                            println!("Tyranid saves the attack! {} rolled", roll_to_save);
                        }
                    } else {
                        println!("Failed to wound! {} rolled", roll_to_wound);
                    }
                } else {
                    println!("Shot missed! {} rolled", roll_to_hit);
                }
            }
        }
    }
}