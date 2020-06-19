Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# SPECS Declaration
A simple macro to effectively create [SPECS](https://github.com/amethyst/specs/) systems.

## Adding To Your Project
Add the following to your Cargo.toml file:
```
specs_declaration = "*"
```

## Usage

- system!
- Give the name of the system and, optionally, the generic parameters
- Give the content of your usual SystemData, in the form of a closure
- Write your operations as if you were in the usual run(&self, params..) function of the System


```rust
system!(SystemName, |velocity: WriteStorage<'a, Velocity>| {
    for vel in (&velocity,).join() {
        println!("velocity: {}, {}, {}", vel.x, vel.y, vel.z);
    }
});
```

With generics:

```rust
system!(SystemName<T: Debug + Send + Sync + 'static>, |my_resource: Read<'a, T>, velocity: WriteStorage<'a, Velocity>| {
    println!("My Generic Resource: {:?}", my_resource);

    for vel in (&velocity,).join() {
        println!("velocity: {}, {}, {}", vel.x, vel.y, vel.z);
    }
});
```

Consider donating on my [patreon](https://www.patreon.com/jojolepro) if you find this useful!

