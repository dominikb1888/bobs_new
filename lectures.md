# Lectures

{: .highlight .sr }
>
> **Required Reading:** [The Secret Life of Programs](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6071143&query=the%20secret%20life)
>

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
[Let's implement addition without a + operator in Rust](https://medium.com/@julien-ctx/how-to-add-numbers-without-plus-sign-d543b46c9aa4)


### Byte: A group of Bits

### Representing Text


## 2. Combinatorial Logic


### The Case for Digital Computers

- Why Bits and why Digital Computers?

Continuous vs. Discrete Measures: Analog mean continuous measures, Digital by definition means discrete. A bit is either 0 or 1. This dualism is only release in Quantum Computing where a Qbit can take also any state between 0 an 1.

- Size Matters

Electricity travels at 300 Million meters per Second. Can you make electricity faster? Maybe with a whip? No. So making computers faster means reducing the distance our electrons need to travel.

- Stability at miniature scale - Digital wins

The smaller things become the more suscept they are to interference. Think Schroedingers Cat in the Quantum Space. Anlog signals are easier to interfer with from the outside with than digital signals, because the have distinct decision criteria. There are no in-between values in digital! Well, if everything is shielded well and there is no crosstalk between wires. Think very small wires at a few nm (10 ^ -9 meters distance between wires on a chip) vs. a human hair at 100.000 nm.

- The Transition between Analog and Digital

Transfer Functions: Logistic Function with a clear threshold.

- So, why bits?

Simple answer: 1 threshold is easier to deal with than 10.
See figure 2.7, page 41.

### A short primer on electricity

### NAND Game

[https://www.nandgame.com/](https://www.nandgame.com/)



## 3. Sequential Logic

- Latches, FlipFlops, Counters, and Registers
- Memory Organization and Adressing
- Block Devices




## 4. Computer Anatomy

- Memory: https://educatedguesswork.org/posts/memory-management-1/
- I/O: CPU, ALU
- CPU Instructions
- GPU


## 5. Computer Architecture

- Basic Architectures (von Neumann, Harvard)
- Procedures, Subroutines, and Functions
- Stacks
- Interrupts
- Relative Addressing
- MMUs (Memory Management Units)
- Arranging Data in Memory
- Running Programs


## 6. Communications Breakdown

- Low-level I/O
- Parallel & Serial Communication
- Networking and the Internet
- Analog in the Digital World (Sound, Images, Videos)
- Human-Computer Interfaces (Terminal, GUI, Keyboard, Mouse)


## 7. Organizing Data

- Primitive Data Types
- Compound Data Types (Linked Lists, Trees)
- Dynamic Memory Allocation
- Garbage Collection
- Databases
- Vectored I/O
- Sorting and Hashing

## 8. Language Processing

- Assembly Language
- High-Level Languages
- State Machines
- Regular Expressions
- Parse Trees
- Interpreters
- Compilers
- Optimization
- System Architectures/Targets (x86-64, aarch64, ...)

## 9. The Web Browser

- Markup Languages
- URLs
- HTML Documents and the DOM
- CSS
- XML
- JavaScript and jQuery
- SVG
- HTML5
- JSON

## 10. Application and System Programming

- Guess the Animal (WEB)
- Guess the Animal (CLI): Data Structures and Memory Management



## 11. Shortcuts and Approximations

- Table Lookup vs. Calculation: Characters and
- Integer Methods: "Straight" Lines
- Recursive Subdivision
- Math Avoidance
- Random Stuff and Quantization

## 12. Deadlocks and Race Conditions

- Race Conditions
- Processes and Shared Resources
- Locks
- Asynchronous Functions and Promises

## 13. Security

- Security and Privacy
- Cryptography
- Software Hygiene

## 14. Machine Intelligence

- Machine Learning
- Artificial Intelligence
- Big Data
- Reflection and Outlook: The industry and your role
