# AES attacks

Homemade implementation of several attacks against AES, written in Rust

## Overview

### Square Attack against 4 rounds

This crate contains a simple implementation of the Square Attack against 4 rounds AES, based on this amazing 
[website](https://www.davidwong.fr/blockbreakers/) (all credits goes to David Wong). The source code gathers all the 
functions to encrypt using 128-bit AES algorithm, and a module dedicated to square attack.

I already implemented this attack in Python in this [repository](https://github.com/thomasperrot/aes-square-attack), 
and wanted to reimplement it in Rust.

### Differential Fault Analysis using a single fault at eighth round

This crate contains an implementation of a Differential Fault Analysis on AES, using a single fault at eighth round. 
It is based on the research paper [Differential Fault Analysis of the Advanced Encryption Standard using a Single Fault](https://eprint.iacr.org/2009/575.pdf). 
This approach boils down to reducing the set of possible keys to `2^32` thanks to a first set of equations, then 
reducing this set to `2^8` possible keys thanks to a second set of equations. It relies on a single fault introduced in 
AES 8th round, before Sub Bytes.

**N.B**: The research paper contains several mistakes in the equations it provides. I corrected them in my source code.

I already implemented this attack in Python in this [repository](https://github.com/thomasperrot/aes-differential-fault-analysis),
and wanted to reimplement it in Rust.

#### Performances

* In Python, the attack takes 8h to complete
* In Rust, the attack takes 2min to complete

So for this specific case, my Rust implementation is 240 times faster than my Python one.
