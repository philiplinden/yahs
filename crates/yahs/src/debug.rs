use bevy::prelude::*;

pub fn notify_on_added<T: Component>(query: Query<Entity, Added<T>>) {
    for entity in query.iter() {
        let type_name = std::any::type_name::<T>();
        info!("{} added to entity: {:?}", type_name, entity);
    }
}
