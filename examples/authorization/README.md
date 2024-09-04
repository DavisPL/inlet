# ECommerce API

In this example, we've set up a skeleton for an MVC web application. Its layout is inspired by Ruby on Rails, and the the policy idea is inspired by the [Pundit](https://github.com/varvet/pundit) gem.

The idea goes like this - authorization (making sure that an authenticated user is allowed to access or modify particular resources) is easy to mess up, and you won't know if it works without extensive testing. Even then, you might have forgotten something. You might also have a spaghetti code problem and perform similar authentication logic in different places, resulting in unpleasant bugs that don't get discovered until your code is up in production.

While you can't statically guarantee that authentication will _always_ work without something like Dafny or Z3, you can at least make it difficult to introduce bugs by isolating the authentication logic and using types to enforce that isolation.

We can do that by requiring a `user` argument for every model method, and also requiring that this argument has a certain origin, depending on the method. For example, we might require that the `user` passed to `Product::create` was computed by the `policies::product::create::authorize` function. This means if we use a different function to check the user, or if we construct the user ourselves, we will get an error before the code even finishes compiling!

Yes, technically you could still make an error in your authentication logic. But at least it's guaranteed to be in a single place.

Run this example with `cargo run -- -p examples/authorization` from the Inlet project root.

