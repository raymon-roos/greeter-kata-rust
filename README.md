### Greetings!

The goal is simple, given some names as input, produce a greeting as output. This is an
exercise in writing unit tests, originally made by Bit Academy. See the requirements:

1. **Write a method `greet(name)`** to offer a simple greeting. For instance, with the
   name “Ada Lovelace”, this method should return `"Hello Ada Lovelace."`.

2. **Greet strangers.** If a name is unknown, and you are presented with an empty string
   or list, respond with `"Hello friend."`.

3. **Accommodate those who are hard hearing.** If the name is all in uppercase, ensure
   the response mirrors this. For example, for the name “GRACE HOPPER”, the response
   should be `"HELLO GRACE HOPPER"`.

4. **Handle pairs of names appropriately.** If the name is a list with two items,
   include both names with 'and' between them. For instance, if the names are
   `["Grace Hopper", "Margaret Hamilton"]`, the result should be
   `"Hello Grace Hopper and Margaret Hamilton."`.

5. **Properly manage multiple names.** If the name is a list with more than two items,
   list all names separated by commas, with 'and' preceding the last one (no Oxford Comma!).
   For example, for `["Ada Lovelace", "Grace Hopper", "Margaret Hamilton"]`, the greeting
   should be `"Hello Ada Lovelace, Grace Hopper, and Margaret Hamilton."`.

6. **Offer distinct greetings for those who are hard hearing and those who are not.**
   For instance, if the list of names includes `["Ada Lovelace", "GRACE HOPPER", "Margaret Hamilton"]`,
   the response should be `"Hello Ada Lovelace and Margaret Hamilton. AND HELLO GRACE HOPPER."`.

7. **If the list of names has entries containing commas, split the names accordingly.**
   For example, if the name is `["Ada Lovelace", "Grace Hopper, Margaret Hamilton"]`,
   greet as `"Hello Ada Lovelace, Grace Hopper, and Margaret Hamilton."`.

These make for clearly defined test cases. I've felt the benefit of such tests, as
refactoring is more pleasant when you can see at a glance that your program's behaviour
remained unchanged.

Doing this in Rust was more trouble than it was in PHP originally. The strict type system,
exhaustive matching, and library functions that return Options makes it that you have to
handle cases more explicitly. In PHP, I could write a single method that handles either
strings or arrays, and converts between them effortlessly.

Rust offers a way more expressive syntax through pattern matching and functional patterns,
and a beautiful standard library, allowing for a relatively concise solution.
