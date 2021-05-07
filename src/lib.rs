#[macro_export]
macro_rules! ecsql {
    // For simple (no filters) queries
    ($world:expr, SELECT ( $( $one:ident $( $two:ty )? ),* )) => {
        $world.query::<( $( ecsql!(@mut_matcher $one $( $two )?) ),* )>()
    };
    // For filtered queries
    ($world:expr,
     SELECT ( $( $one:ident $( $two:ty )? ),* )
     $( WITH ( $( $with:ty ),* ))?
     $( WITHOUT ( $( $without:ty ),* ))?
     $( WHERE $($keyword:ident $type:ty),* )?
    ) => {
        $world.query_filtered::<( $( ecsql!(@mut_matcher $one $( $two )?) ),* ),
                               (
                                   $($( With<$with>, )*)?
                                   $($( Without<$without>, )*)?
                                   $($( ecsql!(@keyword_map $keyword $type), )*)?
                               )>()
    };

    // Helpers

    (@mut_matcher mut $type:ty) => {
        &mut $type
    };
    (@mut_matcher $type:ty) => {
        &$type
    };

    (@keyword_map ADDED $type:ty) => { Added<$type> };
    (@keyword_map CHANGED $type:ty) => { Changed<$type> };
    (@keyword_map $keyword:ident $type:ty) => { $keyword<$type> };
}

#[cfg(test)]
mod tests {
    #![allow(unused_parens)]

    use super::*;
    use bevy_ecs::{
        query::{Added, Changed, With, Without},
        world::World,
    };

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    struct Other;

    #[test]
    fn it_works() {
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

        // Random compilation tests

        let mut query = ecsql!(world, SELECT(Position));
        assert_eq!(6, query.iter(&mut world).len());

        let mut query = ecsql!(world, SELECT(mut Position));
        assert_eq!(6, query.iter_mut(&mut world).len());

        let mut query = ecsql!(world, SELECT(mut Position, Velocity));
        assert_eq!(2, query.iter_mut(&mut world).len());

        let mut query = ecsql!(world, SELECT (Position, Velocity) WITH (Other));
        assert_eq!(
            0,
            query
                .iter(&mut world)
                .collect::<Vec<(&Position, &Velocity)>>()
                .len()
        );

        let mut query = ecsql!(world, SELECT (Position) WITH (Other));
        assert_eq!(
            3,
            query.iter(&mut world).collect::<Vec<(&Position)>>().len()
        );

        let mut query = ecsql!(world, SELECT (Position) WITHOUT (Other, Velocity));
        assert_eq!(
            1,
            query.iter(&mut world).collect::<Vec<(&Position)>>().len()
        );

        let mut query = ecsql!(world, SELECT (Position) WITH (Other) WITHOUT (Velocity));
        assert_eq!(
            3,
            query.iter(&mut world).collect::<Vec<(&Position)>>().len()
        );

        let mut query = ecsql!(world, SELECT (Position) WHERE ADDED Other);
        assert_eq!(
            3,
            query.iter(&mut world).collect::<Vec<(&Position)>>().len()
        );

        let mut query =
            ecsql!(world, SELECT (mut Position, Velocity) WHERE ADDED Other, CHANGED Velocity);
        assert_eq!(0, query.iter_mut(&mut world).collect::<Vec<_>>().len());

        for (mut position, velocity) in query.iter_mut(&mut world) {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}
