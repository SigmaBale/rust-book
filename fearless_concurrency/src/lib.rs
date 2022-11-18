//Fearless Concurrency

/*
Handling concurrent programming safely and efficiently is another of Rust’s major goals. 
Concurrent programming, where different parts of a program execute independently.
Parallel programming, where different parts of a program execute at the same time.
Those two are becoming increasingly important as more computers take advantage of their multiple processors. 
Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.

Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods. 
Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems! 
By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. 
Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, 
incorrect code will refuse to compile and present an error explaining the problem. 
As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production. 
We’ve nicknamed this aspect of Rust fearless concurrency. 
Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

Note: For simplicity’s sake, we’ll refer to many of the problems as concurrent rather than being more precise by saying concurrent and/or parallel. 
If this book were about concurrency and/or parallelism, we’d be more specific. 
For this chapter, please mentally substitute concurrent and/or parallel whenever we use concurrent.

Many languages are dogmatic about the solutions they offer for handling concurrent problems. 
For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads. 
Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages, 
because a higher-level language promises benefits from giving up some control to gain abstractions. 
However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. 
Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:
- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library
*/


//Using Threads to Run Code Simultaneously

/*
In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once. 
Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads. 
For example, a web server could have multiple threads so that it could respond to more than one request at the same time.

Splitting the computation in your program into multiple threads to run multiple tasks at the same time can improve performance, but it also adds complexity. 
Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. 
This can lead to problems, such as:

    - Race conditions, where threads are accessing data or resources in an inconsistent order
    - Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
    - Bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought 
and requires a code structure that is different from that in programs running in a single thread.

Programming languages implement threads in a few different ways, and many operating systems provide an API the language can call for creating new threads. 
The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread. 
There are crates that implement other models of threading that make different tradeoffs to the 1:1 model.
*/


//Creating a New Thread with spawn

/*
To create a new thread, we call the thread::spawn function and pass it a closure containing the code we want to run in the new thread. 
The example in Listing 16-1 prints some text from a main thread and other text from a new thread:

    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    Listing 16-1


Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running. 
The output from this program might be a little different every time, but it will look similar to the following:

    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!


The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. 
The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. 
In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. 
And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down.

If you run this code and only see output from the main thread, or don’t see any overlap, 
try increasing the numbers in the ranges to create more opportunities for the operating system to switch between the threads.
*/


//Waiting for All Threads to Finish Using join Handles

/*
The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending, 
but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!

We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. 
The return type of thread::spawn is JoinHandle. 
A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish. 
Listing 16-2 shows how to use the JoinHandle of the thread we created in Listing 16-1 and call join to make sure the spawned thread finishes before main exits:

    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
    Listing 16-2


Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
Blocking a thread means that thread is prevented from performing work or exiting. 
Because we’ve put the call to join after the main thread’s for loop, running Listing 16-2 should produce output similar to this:

    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 1 from the spawned thread!
    hi number 3 from the main thread!
    hi number 2 from the spawned thread!
    hi number 4 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!


The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.
But let’s see what happens when we instead move handle.join() before the for loop in main, like this:

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore, as shown here:

    hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 3 from the main thread!
    hi number 4 from the main thread!

Small details, such as where join is called, can affect whether or not your threads run at the same time.
*/


//Using move Closures with Threads

/*
We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, 
thus transferring ownership of those values from one thread to another. 
In the “Capturing the Environment with Closures” section of Chapter 13, we discussed move in the context of closures. 
Now, we’ll concentrate more on the interaction between move and thread::spawn.

Notice in Listing 16-1 that the closure we pass to thread::spawn takes no arguments: we’re not using any data from the main thread in the spawned thread’s code. 
To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. 
Listing 16-3 shows an attempt to create a vector in the main thread and use it in the spawned thread. 
However, this won’t yet work, as you’ll see in a moment.

    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
    Listing 16-3


The closure uses v, so it will capture v and make it part of the closure’s environment. 
Because thread::spawn runs this closure in a new thread, we should be able to access v inside that new thread. 
But when we compile this example, we get the following error:

    $ cargo run
    Compiling threads v0.1.0 (file:///projects/threads)
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    --> src/main.rs:6:32
    |
    6 |     let handle = thread::spawn(|| {
    |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
    |                                           - `v` is borrowed here
    |
    note: function requires argument type to outlive `'static`
    --> src/main.rs:6:18
    |
    6 |       let handle = thread::spawn(|| {
    |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
    | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
    |
    6 |     let handle = thread::spawn(move || {
    |                                ++++

    For more information about this error, try `rustc --explain E0373`.
    error: could not compile `threads` due to previous error


Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. 
However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.

Listing 16-4 provides a scenario that’s more likely to have a reference to v that won’t be valid:

    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        drop(v); // oh no!

        handle.join().unwrap();
    }
    Listing 16-4


If Rust allowed us to run this code, there’s a possibility the spawned thread would be immediately put in the background without running at all. 
The spawned thread has a reference to v inside, but the main thread immediately drops v, using the drop function we discussed in Chapter 15. 
Then, when the spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid.

To fix the compiler error in Listing 16-3, we can use the error message’s advice:

    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
    |
    6 |     let handle = thread::spawn(move || {
    |                                ++++


By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values. 
The modification to Listing 16-3 shown in Listing 16-5 will compile and run as we intend:

    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
    Listing 16-5


We might be tempted to try the same thing to fix the code in Listing 16-4 where the main thread called drop by using a move closure. 
However, this fix will not work because what Listing 16-4 is trying to do is disallowed for a different reason. 
If we added move to the closure, we would move v into the closure’s environment, and we could no longer call drop on it in the main thread. 
We would get this compiler error instead:

    $ cargo run
    Compiling threads v0.1.0 (file:///projects/threads)
    error[E0382]: use of moved value: `v`
    --> src/main.rs:10:10
    |
    4  |     let v = vec![1, 2, 3];
    |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5  | 
    6  |     let handle = thread::spawn(move || {
    |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
    |                                           - variable moved due to use in closure
    ...
    10 |     drop(v); // oh no!
    |          ^ value used here after move

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `threads` due to previous error


Rust’s ownership rules have saved us again! We got an error from the code in Listing 16-3 because Rust was being conservative and only borrowing v for the thread, 
which meant the main thread could theoretically invalidate the spawned thread’s reference. 
By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing Rust that the main thread won’t use v anymore. 
If we change Listing 16-4 in the same way, we’re then violating the ownership rules when we try to use v in the main thread. 
The move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.
*/