# Bevy ECSQL

ECSQL is a Query language for `bevy_ecs` inspired by SQL.

This is a meme crate, please don't use this.

Allows you to do the following:

```rust
let mut world = World::new();
world.spawn_batch(vec![
    (Position { x: 0.0, y: 0.0 }, Other),
    (Position { x: 1.0, y: 1.0 }, Other),
    (Position { x: 2.0, y: 2.0 }, Other),
]);
world.spawn().insert(Position { x: 0., y: 10. });
world.spawn_batch(vec![
    (Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }),
    (Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 1.0 }),
]);

let mut query = ecsql!(world, SELECT (mut Position, Velocity) WHERE ADDED Other, CHANGED Velocity);
for (mut position, velocity) in query.iter_mut(&mut world) {
    position.x += velocity.x;
    position.y += velocity.y;
}
```
