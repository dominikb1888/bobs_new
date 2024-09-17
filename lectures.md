# Lectures

Book: The Secret Life of Programs

   Rust Exercises:
   - [Hello World](https://classroom.github.com/a/cYd-7xPR)

## 1. The Internal Language of Computers

### From Morse to the Bit

- Binary Digit = Bit
- A morse code example: https://elucidation.github.io/MorsePy/
- The morse alphabet tells humans how to encode information into long and short sounds: https://en.wikipedia.org/wiki/Morse_code
- What is the same representation on a Computer?

### Boolean Logic

#### Basic Boolean Operations

- NOT This operation means “the opposite.” For example, if a bit is false, NOT that bit would be true. If a bit is true, NOT that bit would be false.
- AND This operation involves 2 or more bits. In a 2-bit operation, the result is true only if both the first AND second bit are true. When more than 2 bits are involved, the result is true only if all bits are true.
- OR This operation also involves 2 or more bits. In a 2-bit operation, the result is true if the first OR second bit is true; otherwise, the result is false. With more than 2 bits, the result is true if any bit is true.
- XOR The result of an exclusive-or operation is true if the first and second bits have different values. It’s either but not both. Because “exclusive-or” is a mouthful, we often use the abbreviation XOR (pronounced “ex-or”).

(taken from The Life of Programs, p.4)

#### Logic Tables

- NOT

| *F* | T |
| *T* | F |


- AND

| | *F* | *T* |
| *F* | F | F |
| *T* | F | T |

- OR

| | *F* | *T* |
| *F* | F | T |
| *T* | T | T |

- XOR

| | *F* | *T* |
| *F* | F | T |
| *T* | T | F |


#### De Morgan's Law

The operation *a AND b* is equivalent to the operation *NOT(NOT a OR NOT b)*

| a | b | *a AND b* | NOT a | NOT b | NOT a OR NOT b | *NOT (NOT a OR NOT b)* |
| F | F | *F* | T | T | T | *F* |
| F | T | *F* | T | F | T | *F* |
| T | F | *F* | F | T | T | *F* |
| T | T | *T* | F | F | F | *T* |

<iframe width="806" height="453" src="https://www.youtube.com/embed/LBGbwQDhceg" title="De Morgan&#39;s Laws (in a probability context)" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


[Watch a Video explaining De Morgan's Law with Venn Diagrams](https://www.youtube.com/watch?v=LBGbwQDhceg)

{: .note}
[Let's Play some Venn!](https://betterinformatics.com/resources/inf1-cl/venn)

{: .note }
[Generate your own Truthtables](https://mrieppel.net/prog/truthtable.html)

{: .highlight }
> - [Bob](https://classroom.github.com/a/SrWip1V0)
> - [Leap Year](https://classroom.github.com/a/QGy47nk5)


### Representing Numbers

Let's look at an extended version of Decimal Notation first. How do our 10 digits become an infinite amount of numbers? Each position is assigned a power of ten implicitly:

| 10<sup>3</sup> | 10<sup>2</sup> | 10<sup>1</sup> | 10<sup>0</sup> |
| 1000 | 100 | 10 | 1 |
| --- | --- | --- | --- |
| 5 | 8 | 2 | 0 |

$$
1000 *5 + 100 * 8 + 10 * 2 + 1 * 0 = 5820
$$

Now how would the same numer look like if we only had two digits instead of ten? We need to build powers of 2:

|2<sup>12</sup> | 2<sup>11</sup> | 2<sup>10</sup> | 2<sup>9</sup> | 2<sup>8</sup> | 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |
| 4096 | 2048 | 1024 | 512 | 256 | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 0 | 0 | 1 | 1 | 1 |  0 | 1 | 0 | 0 | 1 | 0 | 0 |


$$
5 + 5
$$


$$
1 * 2^{12} + 0* 2^{11} + 0 * 2^{10} + 1 * 2^9 + 1 *  2^8 + 1 * 2^7 + 0 * 2^6 + 1 * 2^5 + 0 * 2^4 + 0 *  2^3 + 1 * 2^2 + 0 * 2^1 + 0 * 2^0 = 5028
$$

#### Range of Bits

| Number of bits | Number of values | Range of values |
|---|---|---|
| 4 | 16 | 0…15 |
| 8 | 256 | 0…255 |
| 12 | 4,096 | 0…4,095 |
| 16 | 65,536 | 0…65,535 |
| 20 | 1,048,576 | 0…1,058,575 |
| 24 | 16,777,216 | 0…16,777,215 |
| 32 | 4,294,967,296 | 0…4,294,967,295 |
| 64 | 18,446,744,073,709,551,616 | 0…18,446,744,073,709,551,615 |

#### Most and Least Significant Bits

| MSB | | | | | | | | | | |  | | | LSB |
| 0 | 0 | 0| 1 | 0 | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |

Padding binary numbers with leading zeros doe not habe any effect just like adding a leading 0 to a decimal number (e.g. 05028)

#### Binary Addition

Decimal Addition

| 1 |
| 5 |
| --- |
| 6 |

Binary Addition

| 0 | 0 | 1 |
| 1 | 0 | 1 |
| --- | --- | --- |
| 1 | 1 | 0 |

Expressing the rules of Binary Addition in terms of logical expressions:

$$
a + b = (a  \otimes  b) \lor (a \land b) << 1
$$

This means addition is a three step process:

$$
a  \otimes b
$$

| 0 | 0 | 1 |
| 1 | 0 | 1 |
|---|---|---|
| 1 | 0 | 0 |


$$
a \land b
$$


| 0 | 0 | 1 |
| 1 | 0 | 1 |
|---|---|---|
| 0 | 0 | 1 |

$$
001 \ll 1 = 10
$$

$$
100 \lor 10 = 110
$$


{: .note }
[A nice visual explanation](https://yuminlee2.medium.com/leetcode-371-sum-of-two-integers-f36bdf4317c)

{: .note }
[Let's implement additon without a + operator in Rust](https://medium.com/@julien-ctx/how-to-add-numbers-without-plus-sign-d543b46c9aa4)


### Byte: A group of Bits

### Representing Text


## 2. Combinatorial Logic

-

## 4. Computer Anatomy
## 5. Computer Architecture
## 6. Communications Breakdown
## 7. Organizing Data
## 8. Language Processing
## 9. The Web Browser
## 10. Application and System Programming
## 11. Shortcuts and Approximations
## 12. Deadlocks and Race Conditions
## 13. Security
## 14. Machine Intelligence
## 15. Real-World Considerations

