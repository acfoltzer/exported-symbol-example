``library`` is a crate with a function ``load_and_run()`` that loads a dynamic library and runs a
function within it. It also contains a ``#[no_mangle]`` callback ``extern "C" fn module_callback()``
that prints a message to stdout.

``module`` is a crate that produces a dynamic library with an entrypoint suitable for running with
``library::load_and_run()``. The entrypoint first prints its own message, and then calls
``module_callback()`` via the FFI (it does not depend on ``library`` directly).

``src/main.rs`` is an executable that calls ``library::load_and_run()`` on the dynamic library
produced by ``module``.

Run ``./run.sh`` to build and run the example. A working version of this program would print the
following messages::

  Hello from the module!
  Hello from the callback!

However, the example as checked in does not do that. In ``src/main.rs``, there are two commented-out
bits of code, each of which will cause the example to at least run if uncommented. The first calls
``library::module_callback()`` directly, which prints an extra message::

  Hello from the callback!
  Hello from the module!
  Hello from the callback!

The second is a copy-paste of the definition of ``library::module_callback()``, but in the
executable's crate. It makes the output work as expected.

Note however that none of these variants work without the ``--export-dynamic`` linker flag as set in
``.cargo/config``.
