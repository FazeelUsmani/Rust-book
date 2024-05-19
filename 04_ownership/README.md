# Ownership

1. A foundational goal of Rust is to ensure that your programs never have
   undefined behavior. That is the meaning of "safety." Undefined behavior
   is especially dangerous for low-level programs with direct access to memory.
   About 70% of reported security vulnerabilities in low-level systems are
   caused by memory corruption, which is one form of undefined behavior.

2. A secondary goal of Rust is to prevent undefined behavior at compile-time
   instead of run-time. This goal has two motivations:

   i. Catching bugs at compile-time means avoiding those bugs in production,
      improving the reliability of your software.
   ii. Catching bugs at compile-time means fewer runtime checks for those bugs,
      improving the performance of your software.

Rust cannot prevent all bugs. If an application exposes a public and
unauthenticated /delete-production-database endpoint, then a malicious actor
doesn't need a suspicious if-statement to delete the database. But Rust's
protections are still likely to make programs safer versus using a language
with fewer protections, e.g. as found by Google's Android team.

Use __Box__ to put data on the heap

```Rust
let a = Box::new([0;1_000_000]);
let b = a;
```

__Box deallocation principle (almost correct):__ If a variable is bound to a box,
when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

__Box deallocation principle (fully correct):__ If a variable owns a box, when Rust
deallocates the variable's frame, then Rust deallocates the box's heap memory.

__Moved heap data principle:__ if a variable x moves ownership of heap data to
another variable y, then x cannot be used after the move.

__Cloning Avoids Moves__
One way to avoid moving data is to clone it using the .clone() method. For example, we can fix the safety issue in the previous program with a clone.

Ownership is primarily a discipline of heap management:

* All heap data must be owned by exactly one variable.
* Rust deallocates heap data once its owner goes out of scope.
* Ownership can be transferred by moves, which happen on assignments and function calls.
* Heap data can only be accessed through its current owner, not a previous owner.

__Rust Avoids Simultaneous Aliasing and Mutation__
Pointers are a powerful and dangerous feature because they enable aliasing. Aliasing is accessing the same data through different variables. On its own, aliasing is harmless. But combined with mutation, we have a recipe for disaster. One variable can "pull the rug out" from another variable in many ways, for example:

* By deallocating the aliased data, leaving the other variable to point to deallocated memory.
* By mutating the aliased data, invalidating runtime properties expected by the other variable.
* By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

__Pointer Safety Principle:__ data should never be aliased and mutated at the same time.

__Read (R):__ data can be copied to another location.
__Write (W):__ data can be mutated in-place.
__Own (O):__ data can be moved or dropped.
