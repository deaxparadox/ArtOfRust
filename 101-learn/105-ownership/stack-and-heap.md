# The Stack and the Heap

Both the stack and the heap are parts of  memory available to your code to use at runtime but they are structured in different ways.

- The stack stores values in the order it gets them and removes the values in the opposite order.
- This is refered to as ***last in, first out***.
- Adding data is called *pushing onto the stack*, 
- and removign data is called *popping off the stack*.
- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change must stored on the heap instead.

The headp is less organized: 

- you request a certain amount of space.
- The memory allocators finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of the location.
- This process is called *allocating on the heap* and is somethings abbreviated as just *allocating*
- Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data

- that location is always at the top of the stack.
- allocating space on the heap requeris more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

When your code calls a function, the values, passed into the function (including potentially, points to data on the heap)

- and the function's local variables get psuhed onto the stack.
- When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimiing the acount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses.