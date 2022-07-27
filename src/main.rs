use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::repeat_with;

use qcell::QCellOwner;
use slotmap::{DenseSlotMap, SecondaryMap};

slotmap::new_key_type! {
    struct Entity;
}

type Storage<C> = SecondaryMap<Entity, C>;

#[derive(Copy, Clone, Debug, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug, Default)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Copy, Clone, Debug, Default)]
struct Mass(f32);

fn main() {
    let mut owner = QCellOwner::new();

    let mut entities = DenseSlotMap::<Entity, ()>::with_key();
    let entities: Vec<_> = repeat_with(|| entities.insert(())).take(10).collect();

    // create storages and insert data associated with the entity created above
    let storages = {
        let mut positions = Storage::default();
        let iter = entities.iter().map(|&entity| (entity, Position::default()));
        positions.extend(iter);
        let positions = owner.cell(Box::new(positions) as Box<dyn Any>);

        let mut velocities = Storage::default();
        let iter = entities.iter().map(|&entity| (entity, Velocity::default()));
        velocities.extend(iter);
        let velocities = owner.cell(Box::new(velocities) as Box<dyn Any>);

        let mut masses = Storage::default();
        let iter = entities.iter().map(|&entity| (entity, Mass::default()));
        masses.extend(iter);
        let masses = owner.cell(Box::new(masses) as Box<dyn Any>);

        HashMap::from([
            (TypeId::of::<Position>(), positions),
            (TypeId::of::<Velocity>(), velocities),
            (TypeId::of::<Mass>(), masses),
        ])
    };

    // get multiple mutable storages (sadly, only 3 storages are available for now)
    let (positions, velocities, masses) = owner.rw3(
        storages.get(&TypeId::of::<Position>()).unwrap(),
        storages.get(&TypeId::of::<Velocity>()).unwrap(),
        storages.get(&TypeId::of::<Mass>()).unwrap(),
        // storages.get(&TypeId::of::<NewComponentType>()).unwrap(),
    );
    // downcast them to the actual storage types
    let positions = positions.downcast_mut::<Storage<Position>>().unwrap();
    let velocities = velocities.downcast_mut::<Storage<Velocity>>().unwrap();
    let masses = masses.downcast_mut::<Storage<Mass>>().unwrap();

    // iterate for each entity and get mutable references on components
    for (i, (entity, position)) in positions.iter_mut().enumerate() {
        let velocity = velocities.get_mut(entity).unwrap();
        let mass = masses.get_mut(entity).unwrap();

        let f = i as f32;
        position.x += f * 10.0;
        position.y -= f * 10.0;
        velocity.dx -= f * 5.0;
        velocity.dy += f * 5.0;
        mass.0 += f;

        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, position, velocity, mass,
        );
    }
}
