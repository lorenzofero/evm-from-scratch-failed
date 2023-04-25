
# About

These are the notes I've taken while learning how to do this challenge. My main learning resource have been:
- the ethereum yellow paper (https://ethereum.github.io/yellowpaper/paper.pdf);
- https://ethereum.org/en/developers/tutorials/yellow-paper-evm/;
- chapter 13 of the book "Mastering Ethereum", by Dr. Gavin Wood;
- the holy grail of knowledge, *Wikipedia*.

I've a particular style of writing notes which recalls the one of a math textbook, so where possible I'll try to force a "definition-fact" style exposition.

Here, only the bare minimum required to understand and implement the EVM for this challenge is described. Other information, although extremely interesting and useful to understand the Ethereum blockchain as a whole, are out of this scope.

# Prerequisites

#### Definition: virtual machine
In this context, a **virtual machine** is a software which provides an abstraction of computation and storage, like a runtime environment that is agnostic of the underlying host OS or hardware, enabling its compatibility across a wide variety of systems.

#### Example: the JVM
The Java Virtual Machine is an example of the previous definition. High-level programming languages such as Java or Scala are compiled into the bytecode instruction set of this VM.

#### Definition: stack machine
a **stack machine** is a computer processor or a virtual machine in which the primary interaction is moving short-lived temporary values (volatile memory) to and from a push down stack


# Basics of the EVM

Defining a real entity such as the EVM is not possible, however, the following 'definition' will try to encapsulate the main characteristics of it. Other 'facts' will describe other features. The aim of these distinctions it to provide an easier way to skip through the content.

#### 'Definition': EVM
The main characteristics of the **EVM** are:
- stack-based virtual machine of 1024 bytes, with word size (and thus size of stack items) of 256-bit (32 bytes);
- volatile memory model of bytes array, with word-address and word-addressed;
- storage model of word array, word-addressed;
- does not follow the standard von Neumann architecture. Rather than storing program code in generally-accessible memory or storage, it is stored separately in a virtual ROM interactable only through specialised instruction.

#### Observation: about word-addressing
See https://en.wikipedia.org/wiki/Word_addressing. We have that every address contains a word (it is word-address) and it is not byte addressed (i.e. the address count increases by the number of byte in each address) but word address, which means that the address count will increase by one.




# The making of
This section describes some thoughts I had while implementing the EVM in Rust:
- One of my first achievements was to call the opcodes dynamically using an unsigned integer as 'key'. This has been quite difficult but I learned a lot about `HashMap`, closures and how to store them
- Writing classic OOP in Rust is quite a pain and should be avoided, but at the same times it shows you all the problems of this paradigm regarding ownership and mutability in a non-GC language. Also, I didn't want to mess around with `RefCell` and `Cell` for the moment, and keep it everything as clean as possible. In order to do that, you have to carefully think what you really need as a property of your struct, and what can be done in an easier and clear functional/procedural style.
- I'm not 100% with Rust module tree, and I don't like very much how the project tree is. Maybe it will change with time.



