fn main() {
    const DAYS_OF_XMAS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "a partridge in a pear tree.\n",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for verse in 1..=12 {
        println!(
            "On the {} day of Christmas\nmy true love gave to me",
            DAYS_OF_XMAS[verse - 1]
        );

        if verse == 1 {
            println!("A partridge in a pear tree.\n");
            continue;
        }

        if verse >= 2 {
            for day in (2..=verse).rev() {
                println!("{}", GIFTS[day - 1]);
            }
            println!("And {}", GIFTS[0]);
        }
    }
}