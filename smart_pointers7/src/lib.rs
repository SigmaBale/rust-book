//Creating a Tree Data Structure: a Node with Child Nodes

/*
To start, we’ll build a tree with nodes that know about their child nodes. 
We’ll create a struct named Node that holds its own i32 value as well as references to its children Node values:

    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }


We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly. 
To do this, we define the Vec<T> items to be values of type Rc<Node>. 
We also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.

Next, we’ll use our struct definition and create one Node instance named leaf with the value 3 and no children, 
and another instance named branch with the value 5 and leaf as one of its children, as shown in Listing 15-27:

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }
    Listing 15-27



We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. 
We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch. 
The reason is that leaf has no reference to branch and doesn’t know they’re related. 
We want leaf to know that branch is its parent. We’ll do that next.
*/



//Adding a Reference from a Child to Its Parent

/*
To make the child node aware of its parent, we need to add a parent field to our Node struct definition. 
The trouble is in deciding what the type of parent should be. 
We know it can’t contain an Rc<T>, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, 
which would cause their strong_count values to never be 0.

Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. 
However, a child should not own its parent: if we drop a child node, the parent should still exist. 
This is a case for weak references!

So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a RefCell<Weak<Node>>. 
Now our Node struct definition looks like this:

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }


A node will be able to refer to its parent node but doesn’t own its parent. 
In Listing 15-28, we update main to use this new definition so the leaf node will have a way to refer to its parent, branch:

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    Listing 15-28


Creating the leaf node looks similar to how creating the leaf node looked in Listing 15-27 with the exception of the parent field: leaf starts out without a parent, 
so we create a new, empty Weak<Node> reference instance.

At this point, when we try to get a reference to the parent of leaf by using the upgrade method, we get a None value.
We see this in the output from the first println! statement:

    leaf parent = None

When we create the branch node, it will also have a new Weak<Node> reference in the parent field, because branch doesn’t have a parent node.
We still have leaf as one of the children of branch. 
Once we have the Node instance in branch, we can modify leaf to give it a Weak<Node> reference to its parent. 
We use the borrow_mut method on the RefCell<Weak<Node>> in the parent field of leaf, 
and then we use the Rc::downgrade function to create a Weak<Node> reference to branch from the Rc<Node> in branch.

When we print the parent of leaf again, this time we’ll get a Some variant holding branch: now leaf can access its parent! 
When we print leaf, we also avoid the cycle that eventually ended in a stack overflow like we had in Listing 15-26; 
the Weak<Node> references are printed as (Weak):

    leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
    children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
    children: RefCell { value: [] } }] } })

The lack of infinite output indicates that this code didn’t create a reference cycle. 
We can also tell this by looking at the values we get from calling Rc::strong_count and Rc::weak_count.
*/

//Visualizing Changes to strong_count and weak_count

/*
Let’s look at how the strong_count and weak_count values of the Rc<Node> instances change by creating a new inner scope and moving the creation of branch into that scope.
By doing so, we can see what happens when branch is created and then dropped when it goes out of scope. 
The modifications are shown in Listing 15-29:

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    Listing 15-29

    After leaf is created, its Rc<Node> has a strong count of 1 and a weak count of 0. 
    In the inner scope, we create branch and associate it with leaf, at which point when we print the counts, 
    the Rc<Node> in branch will have a strong count of 1 and a weak count of 1 (for leaf.parent pointing to branch with a Weak<Node>). 
    When we print the counts in leaf, we’ll see it will have a strong count of 2, 
    because branch now has a clone of the Rc<Node> of leaf stored in branch.children, but will still have a weak count of 0.

    When the inner scope ends, branch goes out of scope and the strong count of the Rc<Node> decreases to 0, so its Node is dropped. 
    The weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped, so we don’t get any memory leaks!

    If we try to access the parent of leaf after the end of the scope, we’ll get None again. At the end of the program, 
    the Rc<Node> in leaf has a strong count of 1 and a weak count of 0, because the variable leaf is now the only reference to the Rc<Node> again.

    All of the logic that manages the counts and value dropping is built into Rc<T> and Weak<T> and their implementations of the Drop trait. 
    By specifying that the relationship from a child to its parent should be a Weak<T> reference in the definition of Node, 
    you’re able to have parent nodes point to child nodes and vice versa without creating a reference cycle and memory leaks.
*/

//Summary

/*

This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. 
The Box<T> type has a known size and points to data allocated on the heap. 
The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners. 
The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; 
it also enforces the borrowing rules at runtime instead of at compile time.

Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers.
We explored reference cycles that can cause memory leaks and how to prevent them using Weak<T>.
*/