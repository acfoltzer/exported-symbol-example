This is a set of test cases trying to explore the behavior of exported dynamic symbols in Rust
executables.

The variations ``lib*`` are crates with a function ``load_and_run()`` that loads a dynamic library
and runs a function within it; in some variations, this is a method on a trait ``T``. They also
all contain a ``#[no_mangle]`` callback ``extern "C" fn module_callback()`` that prints a message to
stdout.

``module`` is a crate that produces a dynamic library with an entrypoint suitable for running with
``load_and_run()``. The entrypoint first prints its own message, and then calls
``module_callback()`` via the FFI (it does not depend on ``library`` directly).

``test/working*.rs`` and ``test/broken*.rs`` are executables that call ``load_and_run()`` on the
dynamic library produced by ``module``, using the ``lib*`` crates that correspond to their names.

Run ``./run.sh`` to build and run the examples. Working versions of these programs print the
following messages::

  Hello from the module!
  Hello from the callback!

However, not all of the examples do that:

1. This variation works, because in the body of ``load_and_run`` it calls ``std::ptr::read_volatile``
   on the ``module_callback`` function pointer.

2. This variation doesn't work; it's identical to 1 except that the ``read_volatile`` is commented out.

3. This variation moves ``load_and_run`` into a trait definition ``T``, and provides an empty struct
   ``S`` that implements ``T``. Furthermore, it defines but does not call another trait method
   ``ensure_linked`` that prints a debug representation of ``S``. This version works depending on
   how it's called:

* ``tests/working3.rs`` casts an ``&S`` to a ``&dyn T`` before invoking ``load_and_run``, and
  succeeds.
* ``tests/broken3.rs`` invokes ``load_and_run`` directly on a ``&S``, and fails.

4. This variation differs only from 3 by removing the printing of ``S`` from the uncalled
   ``ensure_linked`` method. This is somehow enough to prevent the example from working!

Note also that none of these variants work without the ``--export-dynamic`` linker flag as set in
``.cargo/config``.
