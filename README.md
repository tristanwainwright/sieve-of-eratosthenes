# Sieve of Eratosthenes

## Description

The Sieve of Eratosthenes is an algorithm used to find all the prime numbers between the first prime $2$ and a given limit $n$.

## Usage

Build project

```shell
cargo build
```

Run project

```shell
cargo run -- -n <N>
```

## Psuedocode

<pre>
    <b>algorithm</b> Sieve of Eratosthenes <b>is</b>
    <b>input</b>: an integer <i>n</i> &gt; 1.
    <b>output</b>: all prime numbers from 2 through <i>n</i>.

    <b>let</b> <i>A</i> be an <b>array of <a href="/wiki/Boolean_data_type" title="Boolean data type">Boolean</a></b> values, indexed by <b>integer</b>s 2 to <i>n</i>,
    initially all <b>set</b> to <b>true</b>.

    <b>for</b> <i>i</i> = 2, 3, 4, ..., not exceeding <span class="texhtml"><i><span class="nowrap">√<span style="border-top:1px solid; padding:0 0.1em;">n</span></span></i></span> <b>do</b>
        <b>if</b> <i>A</i>[<i>i</i>] <b>is</b> <b>true</b>
            <b>for</b> <i>j</i> = <i>i</i><sup>2</sup>, <i>i</i><sup>2</sup>+<i>i</i>, <i>i</i><sup>2</sup>+2<i>i</i>, <i>i</i><sup>2</sup>+3<i>i</i>, ..., not exceeding <i>n</i> <b>do</b>
                <b>set</b> <i>A</i>[<i>j</i>]&nbsp;:= <b>false</b>

    <b>return</b> all <i>i</i> such that <i>A</i>[<i>i</i>] <b>is</b> <b>true</b>.
</pre>

**Source**: <https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Pseudocode>
