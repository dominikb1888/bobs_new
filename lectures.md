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

Watch a Video with Venn Diagrams: [](https://www.youtube.com/watch?v=LBGbwQDhceg)

Let's Play: [](https://betterinformatics.com/resources/inf1-cl/venn)

    Rust Exercises:
    - [Bob](https://classroom.github.com/a/SrWip1V0)
    - [Leap Year](https://classroom.github.com/a/QGy47nk5)


### Representing Numbers



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

