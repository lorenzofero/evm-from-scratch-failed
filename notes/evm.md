
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
- volatile memory model of bytes array, word-addressed;
- storage model of word array, word-addressed;
- does not follow the standard von Neumann architecture. Rather than storing program code in generally-accessible memory or storage, it is stored separately in a virtual ROM interactable only through specialised instruction.


