# Scalar Types

A scalar type represents a single value. 

- Rust has four primary scalar types: 

1. integers
2. floating-point numbers
3. Booleans
4. characters

## Integers

An *integer* is a number without a fractional component.

- The u32 type  declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up 32 bits of space. 

<table><thead><tr><th>Length</th><th>Signed</th><th>Unsigned</th></tr></thead><tbody>
<tr><td>8-bit</td><td><code class="hljs">i8</code></td><td><code class="hljs">u8</code></td></tr>
<tr><td>16-bit</td><td><code class="hljs">i16</code></td><td><code class="hljs">u16</code></td></tr>
<tr><td>32-bit</td><td><code class="hljs">i32</code></td><td><code class="hljs">u32</code></td></tr>
<tr><td>64-bit</td><td><code class="hljs">i64</code></td><td><code class="hljs">u64</code></td></tr>
<tr><td>128-bit</td><td><code class="hljs">i128</code></td><td><code class="hljs">u128</code></td></tr>
<tr><td>arch</td><td><code class="hljs">isize</code></td><td><code class="hljs">usize</code></td></tr>
</tbody></table>

- *Signed* and *unsigned* refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

- Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an `i8` can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a `u8` can store numbers from 0 to 28 - 1, which equals 0 to 255.

- the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

<div class="table-wrapper"><table><thead><tr><th>Number literals</th><th>Example</th></tr></thead><tbody>
<tr><td>Decimal</td><td><code class="hljs">98_222</code></td></tr>
<tr><td>Hex</td><td><code class="hljs">0xff</code></td></tr>
<tr><td>Octal</td><td><code class="hljs">0o77</code></td></tr>
<tr><td>Binary</td><td><code class="hljs">0b1111_0000</code></td></tr>
<tr><td>Byte (<code class="hljs">u8</code> only)</td><td><code class="hljs">b'A'</code></td></tr>
</tbody></table>
</div>


[<<< Data Types](README.md) | [Floating Point Types](101-Floating-Point-Types.md)