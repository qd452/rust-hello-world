use std::vec;

use rust_hello_world::{Bar, Foo, Location, Show, ShowLoc, Status};

// define a struct for a Order book, which contains a list of price levels, each price level contains a price and a size
#[derive(Debug)]
struct PriceLevel {
    price: f64,
    size: f64,
}
#[derive(Debug)]
struct OrderBook {
    price_levels: Vec<PriceLevel>,
}

trait QuackTrait {
    fn quack(&self);
}

struct Duck();

impl QuackTrait for Duck {
    fn quack(&self) {
        println!("Duck quack");
    }
}

/* Quack generic */
fn quack<Q>(q: &Q)
where
    Q: QuackTrait,
{
    q.quack();
}

fn quack_ref(q: &dyn QuackTrait) {
    q.quack();
}

fn quack_everyone<I>(iter: I)
where
    I: IntoIterator,
    I::Item: QuackTrait,
{
    for i in iter {
        i.quack();
    }
}

fn main() {
    use Status::Open;

    let d = Duck();
    d.quack();
    quack_ref(&d);
    quack(&d); // quack(&d) is equivalent to quack_ref(&d)

    dbg!(1, 2, 3);
    let x = 1;
    let y = 2;
    dbg!(x + y);

    println!("quack_everyone");
    let ducks = vec![Duck(), Duck(), Duck()];
    quack_everyone(ducks);

    let status = Status::Open;
    let foo = Foo(5);
    let v: Vec<&dyn Show> = vec![&status, &foo];
    for i in v {
        println!(" - {}", i.show());
    }

    let status = Box::new(Status::Filled);
    let foo = Box::new(Foo(10));
    let v: Vec<Box<dyn Show>> = vec![status, foo];
    for i in v {
        println!(" - {}", i.show());
    }

    println!(
        "status {:?}, {:?}, {:?}",
        Open,
        Status::Filled as i32,
        Status::Expired as i32
    );

    // oop
    println!("");
    let b = Bar::new("bar", "SG");
    // dbg!(b); - value moved here, error E0382  borrow of moved value: `b`

    dbg!(b.show());
    dbg!(b.location());

    let sl: &Bar = &b;
    dbg!(sl.show());
    dbg!(sl.location());

    fn show_all(r: &dyn ShowLoc) {
        dbg!(r.show(), r.location());
    }

    let b2 = Bar::new("bar2", "JP");
    show_all(&b2);

    // very very first rust program
    println!("");
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x: i32 = 5;
    let y: i32 = 10;
    println!("x = {} and y = {}", x, y);
    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);

    let f: Foo = Foo(5);
    println!("{:#?}", f); // pretty print

    // init a order book
    let mut order_book: OrderBook = OrderBook {
        price_levels: Vec::new(),
    };
    // add a price level
    order_book.price_levels.push(PriceLevel {
        price: 100.0,
        size: 100.0,
    });
    // add another price level
    order_book.price_levels.push(PriceLevel {
        price: 101.0,
        size: 200.0,
    });
    // print the order book
    println!("order_book = {:?}", order_book);
    // print the first price level
    println!("first price level = {:?}", order_book.price_levels[0]);
    // print the second price level
    println!("second price level = {:?}", order_book.price_levels[1]);

    println!(
        "p={:?},s={:?}",
        order_book.price_levels[0].price, order_book.price_levels[0].size
    );
}
