
# Insertion sort

* **Why the insertion sort mentioned in this chapter?** the goal of that making you familiar with the framework , design and analysis of algorithms.


* **Insertion sort mechanism** imagine you have playing cards . You want to arrange it . you will make a starting card . You will compare it with it's prev card if it less order you will move the prev card one step forward to make it in right position then repeat until find the correct position of starting card then choose a new starting card 

* **Loop Invariant** It is a formal property used to prove the correctness of an algorithm. It is very similar to Mathematical Induction:

1. Initialization: The property is true before the first iteration (Base case).

Maintenance: If it's true before an iteration, it remains true before the next one (Inductive step).

2. Termination: When the loop ends, the invariant gives us a useful property that shows the algorithm is correct.

3. In Insertion Sort: At the start of each iteration, the subarray sl[0..i] consists of the original

* **Best case:** when array or set of values is ordered . O(n) 

* **Worst case:** when array or set of values is inversely ordered . O(n^2)

* **Time complexity:** O(n^2) 

### [💡 Rust Implementation Notes (The `usize` vs `isize` challenge)](./sort/insertion_sort.rs)

In **C++**, we usually let the index `j` go down to `-1` to stop the loop. This is possible because `int` is signed.
In **Rust**, array indexing must be done using `usize` (unsigned). If `j` is `0` and you subtract `1`, the program will **panic** in debug mode due to "integer underflow".

* **C++ (Automatic conversion):**
    ```cpp
    int a = 5;
    short b = 2;
    a = b; // Works fine (Implicit conversion)
    ```
* **Rust (Strict typing):**
    ```rust
    let a: i32 = 5;
    let b: i16 = 2;
    // a = b;       <-- Error!
    a = b as i32; // Allowed (Explicit casting)
    ```

**Decision:** To keep the code safe and avoid `as isize` casting, I designed the loop to stay within the `usize` bounds by checking `j > 0` before accessing `sl[j-1]`.
