# accountant
[ABANDONED] Expense tracking CLI application written in Rust

## Usage

```
accountant 1.0
Sathwik Matsa <sathwikmatsa@gmail.com>
Expense tracking CLI application

USAGE:
    accountant [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --category <category>    kind of transaction
    -t, --tags <tags>            tags for data analysis

ARGS:
    <cost>           price of the item
    <description>    note on the transaction

SUBCOMMANDS:
    category    list or add new categories
    friend      add, delete or find a friend
    help        Prints this message or the help of the given subcommand(s)
    list        list the expenses in raw format

```

```
accountant-friend
add, delete or find a friend

USAGE:
    accountant friend [ARGS] [SUBCOMMAND]

ARGS:
    <name>      name for identification, must be unique
    <upi_id>    UPI ID
    <phone>     phone number

SUBCOMMANDS:
    delete    delete a friend and corresponding records
    find      find a friend
    list      list friends
```
```
accountant-friend-delete
delete a friend and corresponding records

USAGE:
    accountant friend delete <name>

ARGS:
    <name>    name of friend
```
```
accountant-friend-list
list friends

USAGE:
    accountant friend list

```
```
accountant-friend-find
find a friend

USAGE:
    accountant friend find <pattern>

ARGS:
    <pattern>    name of friend
```
```
accountant-category
list or add new categories

USAGE:
    accountant category [kind] [SUBCOMMAND]

ARGS:
    <kind>    kind of transactions

SUBCOMMANDS:
    delete    delete a category
    list      list available categories

```
```
accountant-category-delete
delete a category

USAGE:
    accountant category delete <kind>

ARGS:
    <kind>    name of category
```
```
accountant-category-list
list available categories

USAGE:
    accountant category list
```
```
accountant-list
list the expenses in raw format

USAGE:
    accountant list [FLAGS] [OPTIONS]

FLAGS:
        --all        display all expenses

OPTIONS:
    -f, --first <first>    display first `n` expenses
    -l, --last <last>      display latest `n` expenses
```
